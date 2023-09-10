use inkwell::{
    types::FunctionType,
    values::{BasicValue, BasicValueEnum, PointerValue, StructValue},
};

use super::{Compiler, TyprIndex};

/// provides a standard library and adds the functions to the root envoirment
impl<'a, 'ctx> Compiler<'a, 'ctx> {
    // guideline for stdlib functions:
    // current must expect to be passed in thunks
    // before unthunk save the current function in self.fn_value

    fn extract_arguements_primitive<const N: usize>(
        &mut self,
        root: PointerValue<'ctx>,
    ) -> [StructValue<'ctx>; N] {
        let current_node = self
            .builder
            .build_alloca(self.types.generic_pointer, "arg pointer");
        self.builder.build_store(current_node, root);
        let mut args = [self.types.object.const_zero(); N];
        for i in 0..N {
            let arg_load =
                self.builder
                    .build_load(self.types.generic_pointer, current_node, "arg_load");
            let arg_object = self
                .builder
                .build_struct_gep(
                    self.types.args,
                    arg_load.into_pointer_value(),
                    0,
                    "arg data",
                )
                .unwrap();

            let arg_object = self
                .builder
                .build_load(self.types.object, arg_object, "arg data")
                .into_struct_value();

            let next_arg = self
                .builder
                .build_struct_gep(
                    self.types.args,
                    arg_load.into_pointer_value(),
                    1,
                    "next arg",
                )
                .unwrap();
            let next_arg =
                self.builder
                    .build_load(self.types.generic_pointer, next_arg, "load next arg");
            self.builder.build_store(current_node, next_arg);
            args[i] = arg_object;
        }
        args
    }

    // fn build_var_arg(&mut self, action: impl Fn(BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {

    // }

    pub(super) fn make_print(&mut self) {
        // maybe make print should turn into make string

        let print_fn_ty: FunctionType<'_> = self.types.object.fn_type(
            &[
                self.types.call_info.into(),
                self.types.generic_pointer.into(),
            ],
            false,
        );
        let print_fn = self.module.add_function("print", print_fn_ty, None);
        self.fn_value = Some(print_fn);
        let entry_block = self.context.append_basic_block(print_fn, "entry");
        let bool_block = self.context.append_basic_block(print_fn, "bool");
        let number_block = self.context.append_basic_block(print_fn, "number");
        let string_block = self.context.append_basic_block(print_fn, "string");
        let cons_block = self.context.append_basic_block(print_fn, "cons");
        // let lambda_block = self.context.append_basic_block(print_fn, "lambda");
        let symbol_block = self.context.append_basic_block(print_fn, "hempty");
        let hempty_block = self.context.append_basic_block(print_fn, "symbol");
        let ret_block = self.context.append_basic_block(print_fn, "return");
        let error_block = self.context.append_basic_block(print_fn, "error");
        self.builder.position_at_end(entry_block);
        let args = self.extract_arguements_primitive::<1>(
            print_fn.get_nth_param(1).unwrap().into_pointer_value(),
        );
        let args = self.actual_value(args[0]);
        let ty = self.extract_type(args).unwrap().into_int_value();
        self.builder.build_switch(
            ty,
            error_block,
            &[
                (
                    self.types.ty.const_int(TyprIndex::boolean as u64, false),
                    bool_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::number as u64, false),
                    number_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::string as u64, false),
                    string_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::cons as u64, false),
                    cons_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::hempty as u64, false),
                    hempty_block,
                ),
                (
                    self.types.ty.const_int(TyprIndex::symbol as u64, false),
                    symbol_block,
                ),
            ],
        );
        let print = self.functions.printf;
        let print_type = |block,
                          extracter: fn(
            &Compiler<'a, 'ctx>,
            StructValue<'ctx>,
        ) -> Result<BasicValueEnum<'ctx>, String>,
                          fmt_spec,
                          name| {
            self.builder.position_at_end(block);
            let val = extracter(self, args).unwrap();

            self.builder.build_call(
                print,
                &[
                    self.builder
                        .build_global_string_ptr(fmt_spec, &format!("{name} fmt specifier"))
                        .as_basic_value_enum()
                        .into(),
                    val.into(),
                ],
                &format!("print {name}"),
            );
            self.builder.build_unconditional_branch(ret_block);
            self.builder.get_insert_block().unwrap()
        };
        let bool_block = print_type(bool_block, Self::extract_bool, "%i", "boolean");
        let number_block = print_type(number_block, Self::extract_number, "%f", "number");
        let string_block = print_type(string_block, Self::extract_string, "%s", "string");
        let symbol_block = print_type(symbol_block, Self::extract_symbol, "%s", "symbol");
        self.builder.position_at_end(cons_block);
        let call_info = self.types.call_info.const_named_struct(&[
            self.context.i64_type().const_int(1, false).into(),
            self.types.generic_pointer.const_null().into(),
        ]);
        let value = self.make_args(&[args]);
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr("(", "open paren")
                .as_basic_value_enum()
                .into()],
            "print open",
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("car").unwrap(),
                &[call_info.into(), value.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder.build_call(
            print_fn,
            &[
                call_info.into(),
                self.make_args(&[val.into_struct_value()]).into(),
            ],
            "printcar",
        );
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(" ", "space")
                .as_basic_value_enum()
                .into()],
            "print space",
        );
        let val = self
            .builder
            .build_call(
                self.module.get_function("cdr").unwrap(),
                &[call_info.into(), value.into()],
                "getcar",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder.build_call(
            print_fn,
            &[
                call_info.into(),
                self.make_args(&[val.into_struct_value()]).into(),
            ],
            "printcar",
        );
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(" ", "space")
                .as_basic_value_enum()
                .into()],
            "print space",
        );

        let val = self
            .builder
            .build_call(
                self.module.get_function("cgr").unwrap(),
                &[call_info.into(), value.into()],
                "getcgr",
            )
            .try_as_basic_value()
            .unwrap_left();
        self.builder.build_call(
            print_fn,
            &[
                call_info.into(),
                self.make_args(&[val.into_struct_value()]).into(),
            ],
            "printcar",
        );
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr(")", "open paren")
                .as_basic_value_enum()
                .into()],
            "print open",
        );
        self.builder.build_unconditional_branch(ret_block);
        let cons_block = self.builder.get_insert_block().unwrap();
        self.builder.position_at_end(hempty_block);
        self.builder.build_call(
            print,
            &[self
                .builder
                .build_global_string_ptr("hempty", "hempty")
                .as_pointer_value()
                .into()],
            "printcar",
        );
        self.builder.build_unconditional_branch(ret_block);
        self.builder.position_at_end(error_block);
        self.builder.build_call(
            print,
            &[
                self.builder
                    .build_global_string_ptr("not valid type %d\n", "idk")
                    .as_pointer_value()
                    .into(),
                ty.into(),
            ],
            "printcar",
        );

        self.exit("", 1);

        self.builder.position_at_end(ret_block);
        let phi = self.builder.build_phi(self.types.object, "print return");
        phi.add_incoming(&[
            (&args, bool_block),
            (&args, number_block),
            (&args, string_block),
            (&args, cons_block),
            (&args, hempty_block),
            (&args, symbol_block),
        ]);
        self.builder.build_return(Some(&phi.as_basic_value()));
        self.insert_function("print".into(), print_fn)
    }

    // coulsd make this a in umpl defined function and have + be primitive function of 2 parameters
    // and would probally be easier and less duplication for * / - ...
    pub(super) fn make_add(&mut self) {
        let fn_ty = self.types.object.fn_type(
            &[
                self.types.call_info.into(),
                self.types.generic_pointer.into(),
            ],
            false,
        );
        let func = self.module.add_function("add", fn_ty, None);
        self.fn_value = Some(func);
        let entry = self.context.append_basic_block(func, "entry");

        self.builder.position_at_end(entry);
        let current_node = self
            .builder
            .build_alloca(self.types.generic_pointer, "arg pointer");
        self.builder
            .build_store(current_node, func.get_nth_param(1).unwrap());
        let init = self.types.number.const_zero();
        let sum = self.builder.build_alloca(self.types.number, "return sum");
        self.builder.build_store(sum, init);
        let is_done_bb = self.context.append_basic_block(func, "done args?");
        let process_arg_bb = self.context.append_basic_block(func, "process arg");
        let done_bb = self.context.append_basic_block(func, "done args");

        self.builder.build_unconditional_branch(is_done_bb);
        self.builder.position_at_end(is_done_bb);
        let load_args =
            self.builder
                .build_load(self.types.generic_pointer, current_node, "load args");
        let is_done = self
            .builder
            .build_is_null(load_args.into_pointer_value(), "mull = done args");
        self.builder
            .build_conditional_branch(is_done, done_bb, process_arg_bb);

        self.builder.position_at_end(process_arg_bb);
        let arg = self.builder.build_load(
            self.types.args,
            load_args.into_pointer_value(),
            "actual load arg",
        );
        let current = self
            .builder
            .build_extract_value(arg.into_struct_value(), 0, "get_arg_value")
            .unwrap();
        let current = self.actual_value(current.into_struct_value());
        let current = self.extract_number(current).unwrap().into_float_value();
        let old = self.builder.build_load(self.types.number, sum, "load sum");
        let init = self
            .builder
            .build_float_add(old.into_float_value(), current, "add");
        self.builder.build_store(sum, init);
        let next_arg = self
            .builder
            .build_extract_value(arg.into_struct_value(), 1, "get next arg")
            .unwrap();
        self.builder.build_store(current_node, next_arg);
        self.builder.build_unconditional_branch(is_done_bb);
        self.builder.position_at_end(done_bb);
        let sum = self
            .builder
            .build_load(self.types.number, sum, "load sum for returning");
        self.builder
            .build_return(Some(&self.number(sum.into_float_value())));
        self.insert_function("add".into(), func);
    }

    pub(super) fn make_accesors(&mut self) {
        let fn_ty = self.types.object.fn_type(
            &[
                self.types.call_info.into(),
                self.types.generic_pointer.into(),
            ],
            false,
        );
        let mut accesor = |idx, name| {
            let func = self.module.add_function(name, fn_ty, None);
            let entry = self.context.append_basic_block(func, name);
            self.builder.position_at_end(entry);
            self.fn_value = Some(func);

            let args = self.extract_arguements_primitive::<1>(
                func.get_nth_param(1).unwrap().into_pointer_value(),
            );
            // self.print_ir();
            let args = self.actual_value(args[0]);
            // self.print_ir();
            let cons_object = self.extract_cons(args).unwrap().into_struct_value();
            let car = self
                .builder
                .build_extract_value(cons_object, idx, &format!("get-{name}"))
                .unwrap();
            self.builder.build_return(Some(&car));
            self.insert_function(name.into(), func);
        };
        accesor(0, "car");
        accesor(1, "cdr");
        accesor(2, "cgr");
    }

    // create the hempty? function
    // could be written in pure umpl .. efficiency
    pub fn make_is_type(&mut self) {
        macro_rules! is_type {
            ($type:literal,$typeindex:ident) => {{
                let fn_ty = self.types.object.fn_type(
                    &[
                        self.types.call_info.into(),
                        self.types.generic_pointer.into(),
                    ],
                    false,
                );
                let func = self
                    .module
                    .add_function(&format!("{}?", $type), fn_ty, None);
                let entry = self
                    .context
                    .append_basic_block(func, &format!("{}?", $type));
                self.builder.position_at_end(entry);
                self.fn_value = Some(func);
                let args = self.extract_arguements_primitive::<1>(
                    func.get_nth_param(1).unwrap().into_pointer_value(),
                );
                let args = self.actual_value(args[0]);
                let is_type = {
                    let arg_type = self.extract_type(args).unwrap();
                    self.builder.build_int_compare(
                        inkwell::IntPredicate::EQ,
                        arg_type.into_int_value(),
                        self.types.ty.const_int(TyprIndex::$typeindex as u64, false),
                        "is hempty",
                    )
                };
                self.builder.build_return(Some(&self.boolean(is_type)));
                self.insert_function(format!("{}?", $type).into(), func);
            }};
        }
        is_type!("hempty", hempty);
        is_type!("number", number);
        is_type!("boolean", boolean);
        is_type!("string", string);
        is_type!("symbol", symbol);
        is_type!("lambda", lambda);
        is_type!("cons", cons);
        is_type!("thunk", thunk);
        is_type!("primitive", primitive)
    }

    // could be written in pure umpl .. efficiency
    pub fn make_newline(&mut self) {
        let fn_ty = self.types.object.fn_type(
            &[
                self.types.call_info.into(),
                self.types.generic_pointer.into(),
            ],
            false,
        );
        let func = self.module.add_function("newline", fn_ty, None);
        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);
        self.builder.build_call(
            self.functions.printf,
            &[self
                .builder
                .build_global_string_ptr("\n", "newline")
                .as_basic_value_enum()
                .into()],
            "print newline",
        );
        self.builder
            .build_return(Some(&self.const_string(&"\n".into())));
        self.insert_function("newline".into(), func);
    }

    pub(super) fn init_stdlib(&mut self) {
        self.make_accesors();
        self.make_add();
        self.make_print();
        self.make_is_type();
        self.make_newline();
    }

    fn make_args(&mut self, args: &[StructValue<'ctx>]) -> PointerValue<'ctx> {
        let null = self.types.generic_pointer.const_null();
        args.iter().fold(null, |init, current| {
            let ptr = self.builder.build_alloca(self.types.args, "add arg");
            self.builder.build_store(ptr, current.clone());
            let next = self
                .builder
                .build_struct_gep(self.types.args, ptr, 1, "next arg")
                .unwrap();
            self.builder.build_store(next, init);
            ptr
        })
    }
}
