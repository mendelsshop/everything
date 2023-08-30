use std::{
    collections::{HashMap, HashSet},
    iter,
};

use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    execution_engine::ExecutionEngine,
    intrinsics::Intrinsic,
    module::{Linkage, Module},
    passes::PassManager,
    types::{BasicType, FloatType, FunctionType, IntType, PointerType, StructType},
    values::{
        BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, GlobalValue, IntValue,
        PhiValue, PointerValue, StructValue,
    },
    AddressSpace,
};

use crate::{
    ast::{FlattenAst, UMPL2Expr},
    interior_mut::RC,
};
macro_rules! return_none {
    ($expr:expr) => {
        match $expr {
            Some(e) => e,
            _ => return Ok(None),
        }
    };
}
mod conditionals;
mod env;
mod export_code;
mod extract_object;
mod functions;
mod loops;
mod object;
mod stdlib;

/// needed for when we reach stoppers like stop or skip
/// to tell us what type of code to generate ie, br or return
#[derive(Clone, Copy, Debug)]
pub enum EvalType<'ctx> {
    // for a function since it's just build return we dont need to
    // keep any state from the function function
    Function,
    // for a loop we in case of a stop we need to know which block to branch too
    // and in case of a skip what was the start of a loop (block)
    // probably also need to keep track of function it was created in just in case we have the stopper being called from an inner function/thunk
    Loop {
        loop_bb: BasicBlock<'ctx>,
        done_loop_bb: BasicBlock<'ctx>,
        connection: PhiValue<'ctx>,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct Types<'ctx> {
    pub object: StructType<'ctx>,
    pub ty: IntType<'ctx>,
    pub boolean: IntType<'ctx>,
    pub number: FloatType<'ctx>,
    pub string: PointerType<'ctx>,
    pub cons: StructType<'ctx>,
    pub lambda: StructType<'ctx>,
    pub lambda_ty: FunctionType<'ctx>,
    pub symbol: PointerType<'ctx>,
    pub generic_pointer: PointerType<'ctx>,
    pub hempty: StructType<'ctx>,
    pub thunk: FunctionType<'ctx>,
    thunk_ty: StructType<'ctx>,
    primitive_ty: FunctionType<'ctx>,
    /// {param count, basicbock ptr}
    /// maintains information about a function calish
    /// It is a struct that keeps the number of arguments
    /// and also a pointer to a basic block which the function should jump too (if non null) for (gotos)
    call_info: StructType<'ctx>,
    args: StructType<'ctx>,
}

#[derive(Clone, Copy, Debug)]
/// Important function that the compiler needs to access
pub struct Functions<'ctx> {
    pub va_start: FunctionValue<'ctx>,
    pub va_end: FunctionValue<'ctx>,
    exit: FunctionValue<'ctx>,
    printf: FunctionValue<'ctx>,
}

#[derive(Clone, Debug)]
pub struct Compiler<'a, 'ctx> {
    context: &'ctx Context,
    pub(crate) module: &'a Module<'ctx>,
    variables: Vec<(HashMap<RC<str>, PointerValue<'ctx>>, Vec<RC<str>>)>,
    pub builder: &'a Builder<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub(crate) string: HashMap<RC<str>, GlobalValue<'ctx>>,
    // ident stores all used identifiers that were turned in a llvm string literal
    // so we don't store multiple sof the same identifiers
    pub(crate) ident: HashMap<RC<str>, GlobalValue<'ctx>>,
    fn_value: Option<FunctionValue<'ctx>>,
    jit: ExecutionEngine<'ctx>,
    links: HashMap<RC<str>, (PointerValue<'ctx>, FunctionValue<'ctx>)>,
    pub(crate) types: Types<'ctx>,
    // not were umpl functions are stored
    functions: Functions<'ctx>,
    state: Vec<EvalType<'ctx>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
#[allow(non_camel_case_types)]
/// when updating anything in this enum, remember to update the how object is set in [`Compiler::new`] as it is the only thing that won't automatically reflect changes made here
pub enum TyprIndex {
    #[default]
    Unknown = 100,
    boolean = 0,
    number = 1,
    string = 2,
    cons = 3,
    lambda = 4,
    symbol = 5,
    thunk = 6,
    // TODO: make hempty be 0 so object will be zeroinitilizer if its hempty
    hempty = 7,
    // TODO make primitive things so function like print cons, car .. dont needt to unthunk or take env pointers
    primitive = 8,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        context: &'ctx Context,
        module: &'a Module<'ctx>,
        builder: &'a Builder<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
    ) -> Self {
        let jit = module
            .create_jit_execution_engine(inkwell::OptimizationLevel::None)
            .unwrap();
        let env_ptr: PointerType<'ctx> = context
            .struct_type(&[], false)
            .ptr_type(AddressSpace::default());
        let kind = context.opaque_struct_type("object");
        // TODO: make the generic lambda function type not explicitly take an object, and also it should take a number, which signify the amount actual arguments
        // and also it should take a pointer (that if non-null should indirect br to that ptr)
        let call_info = context.struct_type(
            &[
                context.i64_type().into(),
                context.i8_type().ptr_type(AddressSpace::default()).into(),
            ],
            false,
        );
        let fn_type = kind.fn_type(&[env_ptr.into(), call_info.into()], true);
        let lambda = context.struct_type(
            &[
                fn_type.ptr_type(AddressSpace::default()).into(),
                env_ptr.into(),
            ],
            false,
        );
        let generic_pointer = context.i8_type().ptr_type(AddressSpace::default());
        let types = Types {
            object: kind,
            ty: context.i8_type(),
            boolean: context.bool_type(),
            number: context.f64_type(),
            string: context.i8_type().ptr_type(AddressSpace::default()),
            cons: context.struct_type(&[kind.into(), kind.into(), kind.into()], false),
            lambda,
            lambda_ty: fn_type,
            symbol: context.i8_type().ptr_type(AddressSpace::default()),
            generic_pointer,
            hempty: context.struct_type(&[], false),
            thunk_ty: context.struct_type(
                &[
                    kind.fn_type(&[env_ptr.into()], false)
                        .ptr_type(AddressSpace::default())
                        .into(),
                    env_ptr.into(),
                ],
                false,
            ),
            thunk: kind.fn_type(&[env_ptr.into()], false),
            primitive_ty: kind.fn_type(&[call_info.into()], true),
            args: context.struct_type(&[kind.into(), generic_pointer.into()], false),
            call_info,
        };
        let exit = module.add_function(
            "exit",
            context
                .void_type()
                .fn_type(&[context.i32_type().into()], false),
            Some(Linkage::External),
        );
        let printf = module.add_function(
            "printf",
            context.i32_type().fn_type(&[types.string.into()], true),
            Some(Linkage::External),
        );
        let va_arg_start = Intrinsic::find("llvm.va_start").unwrap();
        let va_start = va_arg_start.get_declaration(module, &[]).unwrap();
        let va_arg_end = Intrinsic::find("llvm.va_end").unwrap();
        let va_end = va_arg_end.get_declaration(module, &[]).unwrap();
        let functions = Functions {
            va_start,
            va_end,
            exit,
            printf,
        };
        kind.set_body(
            &[
                types.ty.as_basic_type_enum(),              //type
                types.boolean.as_basic_type_enum(),         // bool
                types.number.as_basic_type_enum(),          //number
                types.string.as_basic_type_enum(),          // string
                types.generic_pointer.as_basic_type_enum(), // cons (maybee turn it back to 3 elemement struct)
                types.lambda.as_basic_type_enum(),          // function
                types.symbol.as_basic_type_enum(),          // symbol
                types.thunk_ty.as_basic_type_enum(),        // thunk
                types.hempty.as_basic_type_enum(),          //hempty
                types.generic_pointer.as_basic_type_enum(), // primitive function
            ],
            false,
        );
        Self {
            context,
            module,
            variables: vec![],
            builder,
            fpm,
            string: HashMap::new(),
            ident: HashMap::new(),
            fn_value: None,
            jit,
            types,
            links: HashMap::new(),
            functions,
            state: vec![],
        }
    }

    pub(crate) fn build_n_select(
        &self,
        default: BasicValueEnum<'ctx>,
        choices: &[(IntValue<'ctx>, BasicValueEnum<'ctx>)],
    ) -> BasicValueEnum<'ctx> {
        choices.iter().fold(default, |prev, choice| {
            self.builder.build_select(choice.0, choice.1, prev, "")
        })
    }

    #[inline]
    fn current_fn_value(&self) -> Result<FunctionValue<'ctx>, &str> {
        self.fn_value.ok_or("could not find current function")
    }
    // / Creates a new stack allocation instruction in the entry block of the function.
    fn create_entry_block_alloca<T>(&self, ty: T, name: &str) -> Result<PointerValue<'ctx>, &str>
    where
        T: BasicType<'ctx>,
    {
        let old_block = self.builder.get_insert_block();
        let fn_value = self.current_fn_value()?;
        // if a function is already allocated it will have an entry block so its fine to unwrap
        let entry = fn_value.get_first_basic_block().unwrap();

        entry.get_first_instruction().map_or_else(
            || self.builder.position_at_end(entry),
            |first_instr| self.builder.position_before(&first_instr),
        );

        let build_alloca = self.builder.build_alloca(ty, name);
        if let Some(bb) = old_block {
            self.builder.position_at_end(bb);
        }
        Ok(build_alloca)
    }

    fn compile_expr(&mut self, expr: &UMPL2Expr) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        match expr {
            UMPL2Expr::Number(value) => Ok(Some(self.const_number(*value).as_basic_value_enum())),
            UMPL2Expr::Bool(value) => Ok(Some(self.const_boolean(*value).as_basic_value_enum())),
            UMPL2Expr::String(value) => Ok(Some(self.const_string(value).as_basic_value_enum())),
            UMPL2Expr::Fanction(r#fn) => self.compile_function(r#fn),
            UMPL2Expr::Ident(s) => self.get_var(s).map(Some),
            UMPL2Expr::Scope(_) => unreachable!(),
            UMPL2Expr::If(if_stmt) => self.compile_if(if_stmt),
            UMPL2Expr::Unless(_) => todo!(),
            // TODO: keep in mind the fact that the loop might be in outer function
            UMPL2Expr::Stop(s) => {
                let res = return_none!(self.compile_expr(s)?);
                match self
                    .state
                    .last()
                    .ok_or("a stop is found outside a funcion or loop")?
                {
                    EvalType::Function => {
                        self.builder.build_return(Some(&res));
                    }
                    EvalType::Loop {
                        loop_bb: _,
                        done_loop_bb,
                        connection,
                    } => {
                        let cont_bb = self
                            .context
                            .append_basic_block(self.fn_value.unwrap(), "loop-continue");
                        self.builder.build_conditional_branch(
                            self.context.bool_type().const_zero(),
                            cont_bb,
                            *done_loop_bb,
                        );
                        connection
                            .add_incoming(&[(&res, self.builder.get_insert_block().unwrap())]);
                        self.builder.position_at_end(cont_bb);
                    }
                };
                Ok(None)
            }
            UMPL2Expr::Skip => {
                // find the newesr "state event" that is a loop
                self.builder.build_unconditional_branch(
                    *self
                        .state
                        .iter()
                        .rev()
                        .find_map(|state| match state {
                            EvalType::Function => None,
                            EvalType::Loop { loop_bb, .. } => Some(loop_bb),
                        })
                        .ok_or("skip found outside loop")?,
                );
                Ok(None)
            }
            UMPL2Expr::Until(until_stmt) => self.compile_while_loop(until_stmt),
            UMPL2Expr::GoThrough(go) => self.compile_for_loop(go),
            UMPL2Expr::ContiueDoing(scope) => self.compile_loop(scope),
            UMPL2Expr::Application(application) => self.compile_application(application),
            UMPL2Expr::Quoted(q) => Ok(Some(q.clone().flatten(self).as_basic_value_enum())),
            // try to retrieve the function and block address from the goto hashmap
            // if not there save whatevers needed and once all codegen completed retry to get information function/address for label from goto hashmap
            // and information to build at the right positon and do it

            // should add unreachable after this?
            // what should this return?
            UMPL2Expr::Label(s) => {
                let link = self.links.get(s).unwrap();
                let call_info = self.types.call_info.const_named_struct(&[
                    self.context.i64_type().const_zero().into(),
                    link.0.into(),
                ]);

                // we subtract 2 b/c the first 2 params are just needed for evaluation (like captured environment, call_info like number of parameters ...)
                let args_count = link.1.count_params() - 2;
                let mut args = iter::repeat(self.types.object.const_zero())
                    .take(args_count as usize)
                    .map(std::convert::Into::into)
                    .collect::<Vec<BasicMetadataValueEnum<'ctx>>>();
                args.insert(0, call_info.into());
                args.insert(0, self.types.generic_pointer.const_null().into());
                self.builder.build_call(link.1, &args, "jump");
                // maybe should be signal that we jumped somewhere
                Ok(Some(self.hempty().into()))
            }
            UMPL2Expr::FnParam(s) => self.get_var(&s.to_string().into()).map(Some),
            UMPL2Expr::Hempty => Ok(Some(self.hempty().into())),
            UMPL2Expr::Link(_, _) => todo!(),
            // UMPL2Expr::Tree(_) => todo!(),
            UMPL2Expr::Let(i, v) => {
                let v = return_none!(self.compile_expr(v)?);
                let ty = self.types.object;
                let ptr = self.create_entry_block_alloca(ty, i).unwrap();
                // let ptr = self.module.add_global(ty, Some(AddressSpace::default()), i).as_pointer_value();
                self.builder.build_store(ptr, v);
                self.insert_variable(i.clone(), ptr);
                // self.context.o
                return Ok(Some(self.types.boolean.const_zero().as_basic_value_enum()));
            }
            // create new basic block use uncdoital br to new bb
            // store the block address and the current fn_value in some sort of hashmap with the name as the key
            UMPL2Expr::ComeTo(n) => {
                let block = self.context.append_basic_block(self.fn_value.unwrap(), n);
                self.links.insert(
                    n.clone(),
                    (
                        unsafe { block.get_address().unwrap() },
                        self.fn_value.unwrap(),
                    ),
                );
                self.builder.build_unconditional_branch(block);
                self.builder.position_at_end(block);
                Ok(Some(self.hempty().into()))
            }
        }
    }

    fn actual_value(&self, thunked: StructValue<'ctx>) -> StructValue<'ctx> {
        // needs entry /condin
        let current_fn = self.fn_value.unwrap();
        let current_bb = self.builder.get_insert_block().unwrap();
        let force = self.context.append_basic_block(current_fn, "force");
        let done_force = self.context.append_basic_block(current_fn, "done-force");

        let ty = self.extract_type(thunked).unwrap().into_int_value();
        let cond = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            ty,
            self.types.ty.const_int(TyprIndex::thunk as u64, false),
            "is thunk",
        );
        self.builder
            .build_conditional_branch(cond, force, done_force);
        self.builder.position_at_end(force);
        let unthunked = self.extract_thunk(thunked).unwrap().into_struct_value();
        let thunked_fn = self
            .builder
            .build_extract_value(unthunked, 0, "thunk-fn")
            .unwrap();
        let unthunked_env = self
            .builder
            .build_extract_value(unthunked, 1, "thunk-env")
            .unwrap();
        let unthunked = self
            .builder
            .build_indirect_call(
                self.types.thunk,
                thunked_fn.into_pointer_value(),
                &[unthunked_env.into()],
                "unthunk",
            )
            .try_as_basic_value()
            .unwrap_left()
            .into_struct_value();
        self.builder.build_unconditional_branch(done_force);
        let force = self.builder.get_insert_block().unwrap();
        self.builder.position_at_end(done_force);
        // we dont need to reget the block for unthunking because we are only calling a function and nothing elsse that would make another block in between
        let object = self.builder.build_phi(self.types.object, "value");
        object.add_incoming(&[(&thunked, current_bb), (&unthunked, force)]);
        object.as_basic_value().into_struct_value()
    }

    pub fn compile_scope(
        &mut self,
        body: &[UMPL2Expr],
    ) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let mut res = Err("scope does not have value".to_string());
        for expr in body {
            res = Ok(return_none!(self.compile_expr(expr)?));
        }
        res.map(Some)
    }

    fn is_null(&self, pv: PointerValue<'ctx>) -> IntValue<'ctx> {
        // let thingload = self.builder.build_load(ty, pv, "null check");
        self.builder.build_is_null(pv, "null check")
        // self.builder.build_int_compare(inkwell::IntPredicate::EQ, thingload, self.types.generic_pointer.const_null().into(), "null check");
    }

    fn is_hempty(&self, arg: StructValue<'ctx>) -> inkwell::values::IntValue<'ctx> {
        let arg_type = self.extract_type(arg).unwrap();
        let is_hempty = self.builder.build_int_compare(
            inkwell::IntPredicate::EQ,
            arg_type.into_int_value(),
            self.types.ty.const_int(TyprIndex::hempty as u64, false),
            "is hempty",
        );
        is_hempty
    }

    pub fn compile_program(
        &mut self,
        program: &[UMPL2Expr],
        _links: HashSet<RC<str>>,
    ) -> Option<String> {
        // self.module.add_function("va_arg", self.types.object.fn_type(&[], false), Some(Linkage::External));
        self.new_env();
        self.init_stdlib();
        self.new_env();
        let main_fn_type = self.context.i32_type().fn_type(&[], false);
        let main_fn = self.module.add_function("main", main_fn_type, None);
        let main_block = self.context.append_basic_block(main_fn, "entry");
        // TODO: maybe dont optimize make_* functions b/c indirect call branches

        self.fn_value = Some(main_fn);

        self.builder.position_at_end(main_block);

        for expr in program {
            match self.compile_expr(expr) {
                Ok(_) => continue,
                Err(e) => return Some(e),
            }
        }
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()));
        self.pop_env();

        let verify = main_fn.verify(true);

        if verify {
            self.fpm.run_on(&main_fn);
            let fpm = PassManager::create(());
            // TODO: more optimizations
            fpm.add_function_inlining_pass();
            fpm.add_merge_functions_pass();
            fpm.add_global_dce_pass();
            fpm.add_ipsccp_pass();
            // fpm.add_strip_symbol_pass();
            fpm.add_constant_merge_pass();

            fpm.add_new_gvn_pass();
            fpm.add_instruction_combining_pass();
            fpm.add_reassociate_pass();
            fpm.add_gvn_pass();
            fpm.add_basic_alias_analysis_pass();
            fpm.add_promote_memory_to_register_pass();
            fpm.add_aggressive_inst_combiner_pass();
            fpm.add_cfg_simplification_pass();
            fpm.add_aggressive_dce_pass();
            fpm.add_instruction_simplify_pass();
            fpm.add_function_inlining_pass();
            fpm.add_strip_dead_prototypes_pass();

            fpm.run_on(self.module);
            None
        } else {
            println!("error occurred");
            self.print_ir();
            unsafe {
                main_fn.delete();
            }

            Some("Invalid generated function.".to_string())
        }
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }
    pub fn run(&self) -> i32 {
        unsafe {
            self.jit
                .run_function(self.module.get_function("main").unwrap(), &[])
                .as_int(false) as i32
        }
    }

    pub fn exit(&self, reason: &str, code: i32) {
        self.builder.build_call(
            self.functions.printf,
            &[self
                .builder
                .build_global_string_ptr(reason, "error exit")
                .as_basic_value_enum()
                .into()],
            "print",
        );
        self.builder.build_call(
            self.functions.exit,
            &[self.context.i32_type().const_int(code as u64, false).into()],
            "exit",
        );

        self.builder.build_unreachable();
    }
}
