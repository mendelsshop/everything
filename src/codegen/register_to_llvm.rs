use inkwell::{
    basic_block::BasicBlock,
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::{BasicType, FunctionType, PointerType, StructType},
    values::{
        AggregateValue, BasicValue, BasicValueEnum, FunctionValue, IntValue, PhiValue,
        PointerValue, StructValue,
    },
    AddressSpace, FloatPredicate, IntPredicate,
};
use inkwell::{module::Linkage, types::BasicTypeEnum};
use itertools::Itertools;
use std::{collections::HashMap, iter, primitive};

use crate::ast::{ONE_VARIADIAC_ARG, ZERO_VARIADIAC_ARG};

use super::sicp::{Const, Expr, Goto, Instruction, Operation, Perform, Register};

macro_rules! fixed_map {
    (@inner $(#[$attrs:meta])* $struct:ident, <$($gen:tt),*>, $type:ty, $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block) => {
        $(#[$attrs])*
        pub struct $struct<$($gen),*> {
            $(
                $fields: $type,
            )*
        }

        impl <$($gen),*> $struct<$($gen),*> {
            pub fn $new($($param: $param_type),*) -> $ret $new_block
            pub const fn get(&self, index: $index) -> $type {
                match index {
                    $(
                        <$index>::$fields => self.$fields,
                    )*
                }
            }
        }
    };

($(#[$attrs:meta])* $struct:ident,$type:ident<$($gen:tt),*>, $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block ) => {
        fixed_map!(@inner $(#[$attrs])* $struct,<$($gen),*> , $type<$($gen),*>, $index {$($fields)*} fn $new($($param: $param_type),*) -> $ret $new_block);
    };
    ($(#[$attrs:meta])* $struct:ident,$type:ident $index:ty {$($fields:ident)*} fn $new:ident($($param:ident: $param_type:ty),*) -> $ret:ty $new_block:block ) => {
        fixed_map!(@inner $(#[$attrs])* $struct,<> , $type<>, $index {$($fields)*} fn $new($($param: $param_type),*) -> $ret $new_block);
    };

}

macro_rules! extract {
    ($fn_name:ident, $unchecked:ident, $type:ident, $name:literal) => {
        #[allow(unused)]
        pub(super) fn $fn_name(&self, val: StructValue<'ctx>) -> BasicValueEnum<'ctx> {
            let current_fn = self.current;
            let prefix = |end| format!("extract-{}:{end}", $name);
            let ret_block = self
                .context
                .append_basic_block(current_fn, &prefix("return"));

            let ty = self.extract_type(val).unwrap().into_int_value();
            let condition = self
                .builder
                .build_int_compare(
                    inkwell::IntPredicate::EQ,
                    ty,
                    self.context
                        .i32_type()
                        .const_int(TypeIndex::$type as u64, false),
                    &prefix("cmp-type"),
                )
                .unwrap();
            self.set_error(&format!("type mismtatch expected {}\n", $name), 1);
            self.builder
                .build_conditional_branch(condition, ret_block, self.error_block)
                .unwrap();

            self.builder.position_at_end(ret_block);
            self.$unchecked(val)
        }
        pub(super) fn $unchecked(&self, val: StructValue<'ctx>) -> BasicValueEnum<'ctx> {
            let prefix = |end| format!("extract-{}:{end}", $name);
            let pointer = self
                .builder
                .build_extract_value(val, 1, &prefix("return"))
                .unwrap();
            (self
                .builder
                .build_load(
                    self.types.types.get(TypeIndex::$type),
                    pointer.into_pointer_value(),
                    &prefix(""),
                )
                .unwrap())
        }
    };
}

fixed_map!(TypeMap, BasicTypeEnum<'ctx>,TypeIndex {empty bool number string symbol label cons primitive thunk lambda}
      fn new(
        empty: BasicTypeEnum<'ctx>,
        bool: BasicTypeEnum<'ctx>,
        number: BasicTypeEnum<'ctx>,
        string: BasicTypeEnum<'ctx>,
        symbol: BasicTypeEnum<'ctx>,
        label: BasicTypeEnum<'ctx>,
        cons: BasicTypeEnum<'ctx>,
        primitive: BasicTypeEnum<'ctx>,
        thunk: BasicTypeEnum<'ctx>,
        lambda: BasicTypeEnum<'ctx>
    ) -> Self {
        Self {
            empty,
            bool,
            number,
            string,
            symbol,
            label,
            cons,
            primitive,
            thunk,
            lambda,
        }
    }
);

fixed_map!(#[allow(non_snake_case)]RegiMap, PointerValue<'ctx>,Register {Env Argl Val Proc Continue Thunk}
    fn new(builder: &Builder<'ctx>, ty: StructType<'ctx>) -> Self {
        let create_register = |name| builder.build_alloca(ty, name).unwrap();
        Self {
            Env: create_register("env"),
            Argl: create_register("argl"),
            Val: create_register("val"),
            Proc: create_register("proc"),
            Continue: create_register("continue"),
            Thunk: create_register("thunk"),
        }
    }

);

#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TypeIndex {
    empty = 0,
    bool = 1,
    number = 2,
    string = 3,
    symbol = 4,
    label = 5,
    cons = 6,
    primitive = 7,
    thunk = 8,
    lambda = 9,
}

pub struct Types<'ctx> {
    object: StructType<'ctx>,
    string: StructType<'ctx>,
    cons: StructType<'ctx>,
    pointer: PointerType<'ctx>,
    types: TypeMap<'ctx>,
    stack: StructType<'ctx>,
    primitive: FunctionType<'ctx>,
    error: StructType<'ctx>,
}
/// Important function that the compiler needs to access
pub struct Functions<'ctx> {
    exit: FunctionValue<'ctx>,
    strncmp: FunctionValue<'ctx>,
    printf: FunctionValue<'ctx>,
    rand: FunctionValue<'ctx>,
    srand: FunctionValue<'ctx>,
    time: FunctionValue<'ctx>,
}

impl<'ctx> Functions<'ctx> {
    pub fn new(module: &Module<'ctx>, context: &'ctx Context) -> Self {
        let exit = module.add_function(
            "exit",
            context
                .void_type()
                .fn_type(&[context.i32_type().into()], false),
            Some(Linkage::External),
        );
        let rand = module.add_function(
            "rand",
            context.i32_type().fn_type(&[], false),
            Some(Linkage::External),
        );
        let printf = module.add_function(
            "printf",
            context.i32_type().fn_type(
                &[context.bool_type().ptr_type(AddressSpace::default()).into()],
                true,
            ),
            Some(Linkage::External),
        );
        let strncmp = module.add_function(
            "strncmp",
            context.i32_type().fn_type(
                &[
                    context.i8_type().ptr_type(AddressSpace::default()).into(),
                    context.i8_type().ptr_type(AddressSpace::default()).into(),
                    context.i32_type().into(),
                ],
                false,
            ),
            Some(Linkage::External),
        );
        let srand = module.add_function(
            "srand",
            context
                .void_type()
                .fn_type(&[context.i32_type().into()], false),
            Some(Linkage::External),
        );
        let time = module.add_function(
            "time",
            context.i32_type().fn_type(
                &[context.i32_type().ptr_type(AddressSpace::default()).into()],
                false,
            ),
            Some(Linkage::External),
        );
        Self {
            exit,
            strncmp,
            printf,
            rand,
            srand,
            time,
        }
    }
}

macro_rules! make_accessors {
    ($name:ident $outer:ident $inner:ident) => {
        pub fn $name(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
            self.$outer(self.$inner(cons))
        }
    };
}

macro_rules! is_type {
    ($name:ident,$type:literal,$typeindex:ident) => {
        #[allow(dead_code)]
        fn $name(&self, obj: StructValue<'ctx>) -> IntValue<'ctx> {
            let arg_type = self.extract_type(obj).unwrap();
            self.builder
                .build_int_compare(
                    inkwell::IntPredicate::EQ,
                    arg_type.into_int_value(),
                    self.context
                        .i32_type()
                        .const_int(TypeIndex::$typeindex as u64, false),
                    "is hempty",
                )
                .unwrap()
        }
    };
}
pub struct CodeGen<'a, 'ctx> {
    context: &'ctx Context,
    builder: &'a Builder<'ctx>,
    module: &'a Module<'ctx>,
    current: FunctionValue<'ctx>,
    labels: HashMap<String, BasicBlock<'ctx>>,
    registers: RegiMap<'ctx>,
    types: Types<'ctx>,
    functions: Functions<'ctx>,
    flag: PointerValue<'ctx>,
    fpm: &'a PassManager<FunctionValue<'ctx>>,
    stack: PointerValue<'ctx>,
    error_block: BasicBlock<'ctx>,
    error_phi: inkwell::values::PhiValue<'ctx>,
}

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    fn extract_type(&self, cond_struct: StructValue<'ctx>) -> Option<BasicValueEnum<'ctx>> {
        self.builder
            .build_extract_value(cond_struct, 0, "get_type")
            .ok()
    }
    is_type!(is_hempty, "hempty", empty);
    is_type!(is_number, "number", number);
    is_type!(is_primitive, "primitive", primitive);
    is_type!(is_boolean, "boolean", bool);
    is_type!(is_string, "string", string);
    is_type!(is_symbol, "symbol", symbol);
    is_type!(is_cons, "cons", cons);
    is_type!(is_label, "label", label);
    is_type!(is_thunk, "thunk", thunk);
    extract!(
        get_primitive,
        unchecked_get_primitive,
        primitive,
        "primitive"
    );
    extract!(get_label, unchecked_get_label, label, "label");
    extract!(get_number, unchecked_get_number, number, "number");
    extract!(get_bool, unchecked_get_bool, bool, "bool");
    extract!(get_cons, unchecked_get_cons, cons, "cons");
    extract!(get_symbol, unchecked_get_symbol, symbol, "symbol");
    extract!(get_string, unchecked_get_string, string, "string");
    extract!(get_thunk, unchecked_get_thunk, thunk, "thunk");
    extract!(get_lambda, unchecked_get_lambda, lambda, "lambda");
    pub fn new(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
    ) -> Self {
        let pointer = context.bool_type().ptr_type(AddressSpace::default());
        let object = context.struct_type(&[context.i32_type().into(), pointer.into()], false);
        let string = context.struct_type(&[context.i32_type().into(), pointer.into()], false);
        let cons = context.struct_type(&[pointer.into(), pointer.into()], false);
        let stack = context.struct_type(&[object.into(), pointer.into()], false);
        let primitive_type = object.fn_type(&[object.into()], false);
        let error = context.struct_type(&[pointer.into(), context.i32_type().into()], false);
        let types = Types {
            object,
            string,
            cons,
            pointer,
            stack,
            primitive: primitive_type,
            error,
            types: TypeMap::new(
                context.struct_type(&[], false).into(),
                context.bool_type().into(),
                context.f64_type().into(),
                string.into(),
                string.into(),
                pointer.into(),
                cons.into(),
                pointer.into(),
                context
                    .struct_type(&[object.into(), object.into()], false)
                    .into(),
                // TODO: make these be not objects but the actual only type that each can be
                context
                    .struct_type(&[object.into(), object.into(), object.into()], false)
                    .into(),
            ),
        };
        let functions = Functions::new(module, context);
        let main = module.add_function("main", context.i32_type().fn_type(&[], false), None);
        let entry_bb = context.append_basic_block(main, "entry");
        let (error_block, error_phi) = init_error_handler(
            main,
            context,
            builder,
            error,
            functions.printf,
            functions.exit,
        );

        builder.position_at_end(entry_bb);
        // init random number seed
        {
            let time = builder
                .build_call(
                    functions.time,
                    &[types.pointer.const_null().into()],
                    "get time to further randomize rng",
                )
                .unwrap()
                .try_as_basic_value()
                .unwrap_left();
            builder
                .build_call(functions.srand, &[time.into()], "set rng seed")
                .unwrap();
        }
        let registers = RegiMap::new(builder, object);
        let mut this = Self {
            stack: builder.build_alloca(stack, "stack").unwrap(),
            context,
            flag: builder.build_alloca(types.object, "flag").unwrap(),
            current: main,
            builder,
            module,
            labels: HashMap::new(),
            registers,
            types,
            fpm,
            functions,
            error_phi,
            error_block,
        };
        this.init_primitives();
        this
    }

    fn make_print(&mut self) {
        self.create_function(
            "print-obj",
            self.context
                .void_type()
                .fn_type(&[self.types.object.into()], false),
            |this, print, entry| {
                let exp = print.get_first_param().unwrap().into_struct_value();
                let ty = this.extract_type(exp).unwrap();
                let namer = |str: &str| format!("print:{str}");
                let make_print_block =
                    |name: &str, code: fn(&Self, BasicBlock<'ctx>, StructValue<'ctx>)| {
                        let block = this.context.append_basic_block(this.current, &namer(name));
                        this.builder.position_at_end(block);
                        code(this, block, exp);
                        this.builder.build_return(None).unwrap();
                        block
                    };

                let empty_bb = make_print_block("empty", |this, _, _| {
                    this.make_printf("()", vec![]);
                });
                let bool_bb = make_print_block("bool", |this, _, exp| {
                    let value = this.unchecked_get_bool(exp);
                    let true_str = this
                        .builder
                        .build_global_string_ptr("true", "true")
                        .unwrap()
                        .as_pointer_value();
                    let false_str = this
                        .builder
                        .build_global_string_ptr("false", "false")
                        .unwrap()
                        .as_pointer_value();
                    let value = this
                        .builder
                        .build_select(value.into_int_value(), true_str, false_str, "bool value")
                        .unwrap();
                    this.builder
                        .build_call(this.functions.printf, &[value.into()], "printf debug")
                        .unwrap();
                });

                let number_bb = make_print_block("number", |this, _, exp| {
                    let value = this.unchecked_get_number(exp);
                    this.make_printf("%.2f", vec![value]);
                });
                let string_bb = make_print_block("string", |this, _, exp| this.print_string(exp));
                let symbol_bb = make_print_block("symbol", |this, _, exp| this.print_string(exp));
                let label_bb = make_print_block("label", |this, _, _| {
                    // TODO: labels should save their name
                    this.make_printf("label", vec![]);
                });
                let cons_bb = {
                    let block = this
                        .context
                        .append_basic_block(this.current, &namer("cons"));
                    this.builder.position_at_end(block);
                    let car = this.make_unchecked_car(exp);
                    let cdr = this.make_unchecked_cdr(exp);
                    this.make_printf("(", vec![]);
                    this.builder
                        .build_call(print, &[car.into()], "print car")
                        .unwrap();
                    this.make_printf(" ", vec![]);
                    // TODO: dot notation and not printing () at end
                    this.builder
                        .build_call(print, &[cdr.into()], "print car")
                        .unwrap();
                    this.make_printf(")", vec![]);
                    this.builder.build_return(None).unwrap();
                    block
                };
                let primitive_bb = make_print_block("primitive", |this, _, _| {
                    // TODO: primitives should save their name
                    this.make_printf("primitive", vec![]);
                });
                let thunk_bb = make_print_block("thunk", |this, _, _| {
                    this.make_printf("thunk", vec![]);
                });
                let lambda_bb = make_print_block("lambda", |this, _, _| {
                    this.make_printf("user define procedure", vec![]);
                });
                this.builder.position_at_end(entry);
                this.set_error("invalid type", 1);
                let make_number = |n| this.context.i32_type().const_int(n, false);
                this.builder.build_switch(
                    ty.into_int_value(),
                    this.error_block,
                    &[
                        (make_number(0), empty_bb),
                        (make_number(1), bool_bb),
                        (make_number(2), number_bb),
                        (make_number(3), string_bb),
                        (make_number(4), symbol_bb),
                        (make_number(5), label_bb),
                        (make_number(6), cons_bb),
                        (make_number(7), primitive_bb),
                        (make_number(8), thunk_bb),
                        (make_number(9), lambda_bb),
                    ],
                );
            },
        );
    }

    // takes an object (asssumed to be a string or symbol) amd prints at runtime
    // the reason it can be an string or symbol is because besides for the type tag both are really the same
    fn print_string(&self, str: StructValue<'ctx>) {
        let str = self.unchecked_get_symbol(str).into_struct_value();
        let len = self.builder.build_extract_value(str, 0, "strlen").unwrap();
        let str = self.builder.build_extract_value(str, 1, "strlen").unwrap();
        // https://stackoverflow.com/questions/256218/the-simplest-way-of-printing-a-portion-of-a-char-in-c
        // for how to print fixed length strings
        self.make_printf("%.*s", vec![len, str]);
    }

    fn make_primitive_pair(
        &self,
        name: &str,
        function: FunctionValue<'ctx>,
    ) -> (StructValue<'ctx>, StructValue<'ctx>) {
        let fn_pointer = function.as_global_value().as_pointer_value();
        let primitive = self.make_object(&fn_pointer, TypeIndex::primitive);
        let name = self.create_symbol(name);
        (name, primitive)
    }
    fn init_accessors(&mut self) -> Vec<(&'static str, FunctionValue<'ctx>)> {
        macro_rules! accessors {
        ($(($name:literal $acces:ident )),*) => {
            vec![$(($name, self.create_primitive($name, |this,func,_|{
                let argl = func.get_first_param().unwrap().into_struct_value();
                // this.print_object(argl);
                self.builder.build_return(Some(&this.$acces(this.make_car(func.get_first_param().unwrap().into_struct_value())))).unwrap();
            }))),*]
        };

        }
        accessors!(("car" make_car), ("cdr" make_cdr), ("caar" make_caar),("cadr" make_cadr),("cdar" make_cdar),("cddr" make_cddr),("caaar" make_caaar),("caadr" make_caadr),("cadar" make_cadar),("caddr" make_caddr),("cdaar" make_cdaar),("cdadr" make_cdadr),("cddar" make_cddar),("cdddr" make_cdddr),("caaaar" make_caaaar),("caaadr" make_caaadr),("caadar" make_caadar),("caaddr" make_caaddr),("cadaar" make_cadaar),("cadadr" make_cadadr),("caddar" make_caddar),("cadddr" make_cadddr),("cdaaar" make_cdaaar),("cdaadr" make_cdaadr),("cdadar" make_cdadar),("cdaddr" make_cdaddr),("cddaar" make_cddaar),("cddadr" make_cddadr),("cdddar" make_cdddar),("cddddr" make_cddddr))
    }

    fn make_eq_obj(&mut self) {
        self.create_function(
            "eq-obj",
            self.types
                .types
                .bool
                .fn_type(&[self.types.object.into(), self.types.object.into()], false),
            |this, eq_fn, _| {
                let e1 = eq_fn.get_first_param().unwrap().into_struct_value();
                let e2 = eq_fn.get_nth_param(1).unwrap().into_struct_value();
                let t1 = this.extract_type(e1).unwrap().into_int_value();
                let t2 = this.extract_type(e2).unwrap().into_int_value();

                let is_same_type = this
                    .builder
                    .build_int_compare(IntPredicate::EQ, t1, t2, "same type?")
                    .unwrap();
                let same_bb = this.context.append_basic_block(this.current, "same type");
                let not_same_bb = this
                    .context
                    .append_basic_block(this.current, "not same type");
                this.builder
                    .build_conditional_branch(is_same_type, same_bb, not_same_bb)
                    .unwrap();
                {
                    this.builder.position_at_end(not_same_bb);
                    this.builder
                        .build_return(Some(&this.types.types.bool.const_zero()))
                        .unwrap();
                }
                {
                    this.builder.position_at_end(same_bb);
                    let hempty_bb = this.context.append_basic_block(this.current, "hempty");
                    let string_bb = this.context.append_basic_block(this.current, "string");
                    let number_bb = this.context.append_basic_block(this.current, "number");
                    let bool_bb = this.context.append_basic_block(this.current, "bool");
                    let symbol_bb = this.context.append_basic_block(this.current, "symbol");
                    let cons_bb = this.context.append_basic_block(this.current, "cons");
                    let thunk_bb = this.context.append_basic_block(this.current, "thunk");
                    let primitive_bb = this.context.append_basic_block(this.current, "primitive");
                    let label_bb = this.context.append_basic_block(this.current, "label");
                    let lambda_bb = this.context.append_basic_block(this.current, "lambda");
                    let make_number = |n| this.context.i32_type().const_int(n as u64, false);
                    this.builder.build_switch(
                        t1,
                        this.error_block,
                        &[
                            (make_number(TypeIndex::empty), hempty_bb),
                            (make_number(TypeIndex::bool), bool_bb),
                            (make_number(TypeIndex::number), number_bb),
                            (make_number(TypeIndex::string), string_bb),
                            (make_number(TypeIndex::symbol), symbol_bb),
                            (make_number(TypeIndex::cons), cons_bb),
                            (make_number(TypeIndex::lambda), lambda_bb),
                            (make_number(TypeIndex::primitive), primitive_bb),
                            (make_number(TypeIndex::label), label_bb),
                            (make_number(TypeIndex::thunk), thunk_bb),
                        ],
                    );
                    this.set_error("invalid type", 1);
                    {
                        this.builder.position_at_end(hempty_bb);
                        this.make_printf("hempty", vec![]);

                        let basic_value = this.types.types.bool.into_int_type().const_int(1, false);

                        this.builder.build_return(Some(&basic_value)).unwrap();
                    }
                    {
                        this.builder.position_at_end(bool_bb);
                        let b1 = this.unchecked_get_bool(e1).into_int_value();
                        let b2 = this.unchecked_get_bool(e2).into_int_value();
                        let same = this
                            .builder
                            .build_int_compare(IntPredicate::EQ, b1, b2, "bool compare")
                            .unwrap();

                        this.builder.build_return(Some(&same)).unwrap();
                    }
                    {
                        this.builder.position_at_end(number_bb);
                        let b1 = this.unchecked_get_number(e1).into_float_value();
                        let b2 = this.unchecked_get_number(e2).into_float_value();
                        let equal = this
                            .builder
                            .build_float_compare(FloatPredicate::OEQ, b1, b2, "number compare")
                            .unwrap();
                        // this.make_printf(
                        //     "\n%.2f = %.2f: %s\n",
                        //     vec![
                        //         b1.as_basic_value_enum(),
                        //         b2.into(),
                        //         this.builder
                        //             .build_select(
                        //                 equal,
                        //                 this.builder
                        //                     .build_global_string_ptr("true", "true")
                        //                     .unwrap(),
                        //                 this.builder
                        //                     .build_global_string_ptr("false", "false")
                        //                     .unwrap(),
                        //                 "equal string",
                        //             )
                        //             .unwrap(),
                        //     ],
                        // );
                        this.builder.build_return(Some(&equal));
                    }
                    {
                        this.builder.position_at_end(string_bb);

                        this.builder.build_return(Some(&this.compare_str(
                            Self::unchecked_get_string,
                            e1,
                            e2,
                        )));
                    }
                    {
                        this.builder.position_at_end(symbol_bb);

                        this.builder.build_return(Some(&this.compare_str(
                            Self::unchecked_get_symbol,
                            e1,
                            e2,
                        )));
                    }
                    {
                        this.builder.position_at_end(cons_bb);
                        let car1 = this.make_unchecked_car(e1);
                        let car2 = this.make_unchecked_car(e2);
                        let cdr1 = this.make_unchecked_cdr(e1);
                        let cdr2 = this.make_unchecked_cdr(e2);
                        let car = this
                            .builder
                            .build_call(eq_fn, &[car1.into(), car2.into()], "car eq")
                            .unwrap()
                            .try_as_basic_value()
                            .unwrap_left()
                            .into_int_value();
                        let cdr = this
                            .builder
                            .build_call(eq_fn, &[cdr1.into(), cdr2.into()], "cdr eq")
                            .unwrap()
                            .try_as_basic_value()
                            .unwrap_left()
                            .into_int_value();
                        this.builder
                            .build_return(Some(
                                &this.builder.build_and(car, cdr, "cons eq?").unwrap(),
                            ))
                            .unwrap();
                    }
                    {
                        self.builder.position_at_end(label_bb);
                        // let l1 = this.unchecked_get_label(e1).into_pointer_value();
                        // let l2 = this.unchecked_get_label(e2).into_pointer_value();
                        // pointer equality is undefined on basic block addrsses

                        // blockaddress(@function, %block)

                        // The ‘blockaddress’ constant computes the address of the specified basic block in the specified function.

                        // It always has an ptr addrspace(P) type, where P is the address space of the function containing %block (usually addrspace(0)).

                        // Taking the address of the entry block is illegal.

                        // This value only has defined behavior when used as an operand to the ‘indirectbr’ or for comparisons against null. Pointer equality tests between labels addresses results in undefined behavior — though, again, comparison against null is ok, and no label is equal to the null pointer. This may be passed around as an opaque pointer sized value as long as the bits are not inspected. This allows ptrtoint and arithmetic to be performed on these values so long as the original value is reconstituted before the indirectbr instruction.

                        // Finally, some targets may provide defined semantics when using the value as the operand to an inline assembly, but that is target specific.

                        // although it may be possible to use ptrtoint to compare the 2 pointers

                        // let p_eq = self.builder.build_ptr_diff(l2.get_type()., lhs_ptr, rhs_ptr, name).unwrap()
                        this.builder
                            .build_unconditional_branch(this.error_block)
                            .unwrap();
                        this.set_error("cannot compare labels", 1)
                    }
                    {
                        self.builder.position_at_end(thunk_bb);
                        this.builder
                            .build_unconditional_branch(this.error_block)
                            .unwrap();
                        this.set_error("cannot compare thunks", 1)
                    }
                    {
                        self.builder.position_at_end(lambda_bb);
                        this.builder
                            .build_unconditional_branch(this.error_block)
                            .unwrap();
                        this.set_error("cannot compare lambdas", 1)
                    }
                    {
                        self.builder.position_at_end(primitive_bb);
                        this.builder
                            .build_unconditional_branch(this.error_block)
                            .unwrap();
                        this.set_error("cannot compare primitives", 1)
                    }
                }
            },
        );
        // println!("{}", self.export_ir());
        // panic!()
    }

    fn init_primitives(&mut self) {
        // seems to problem with primitive that retunrn something meaningful not returning properly unless / possiblely some other action done on the in the primtive function
        self.make_print();
        self.make_eq_obj();
        let primitive_newline = self.create_primitive("newline", |this, _, _| {
            this.builder.build_call(
                this.functions.printf,
                &[this
                    .builder
                    .build_global_string_ptr("\n", "\n")
                    .unwrap()
                    .as_pointer_value()
                    .into()],
                "call newline",
            );
            this.builder.build_return(Some(&this.empty())).unwrap();
        });
        let primitive_cons = self.create_primitive("cons", |this, cons, _| {
            let argl = cons.get_first_param().unwrap().into_struct_value();
            let car = this.make_car(argl);
            let cdr = this.make_cadr(argl);
            let cons = this.make_cons(car, cdr);
            // this.print_object(cons);
            this.builder.build_return(Some(&cons)).unwrap();
        });
        let primitive_eq = self.create_primitive("eq", |this, cons, _| {
            let argl = cons.get_first_param().unwrap().into_struct_value();
            let e1 = this.make_car(argl);
            let e2 = this.make_cadr(argl); // doesnt do proper thing even though i have verified that argl is like (6 (6 ()))
            let eq = this.make_object(&this.compare_objects(e1, e2), TypeIndex::bool);
            this.builder.build_return(Some(&eq)).unwrap();
        });
        let primitive_set_car = self.create_primitive("set-car!", |this, set_car, _| {
            let argl = set_car.get_first_param().unwrap().into_struct_value();
            let cons = this.make_car(argl);
            let val = this.make_cadr(argl);
            this.make_set_car(cons, val);
            this.builder.build_return(Some(&this.empty())).unwrap();
        });
        let primitive_set_cdr = self.create_primitive("set-cdr!", |this, set_cdr, _| {
            let argl = set_cdr.get_first_param().unwrap().into_struct_value();
            let cons = this.make_car(argl);
            let val = this.make_cadr(argl);
            this.make_set_cdr(cons, val);
            this.builder.build_return(Some(&this.empty())).unwrap();
        });

        let primitive_not = self.create_primitive("not", |this, primitive_not, _| {
            let args = primitive_not.get_first_param().unwrap().into_struct_value();
            let arg = this.make_car(args); // when we do arrity check we can make this unchecked car
            let truthy = this.truthy(arg);
            let not_truthy = this.builder.build_not(truthy, "not").unwrap();
            this.builder
                .build_return(Some(&this.make_object(&not_truthy, TypeIndex::bool)))
                .unwrap();
        });
        let primitive_print = self.create_primitive("print", |this, set_car, _| {
            let argl = set_car.get_first_param().unwrap().into_struct_value();
            let val = this.make_car(argl);
            this.print_object(val);
            this.builder.build_return(Some(&this.empty())).unwrap();
        });

        let primitive_sub1 = self.create_primitive("-1", |this, sub1, _| {
            let argl = sub1.get_first_param().unwrap().into_struct_value();
            let val = this.make_car(argl);
            let num = this.get_number(val).into_float_value();
            let num1 = this
                .builder
                .build_float_sub(
                    num,
                    this.types
                        .types
                        .get(TypeIndex::number)
                        .into_float_type()
                        .const_float(1.0),
                    "sub 1",
                )
                .unwrap();
            let result = this.make_object(&num1, TypeIndex::number);
            // this.print_object(result);
            this.builder
                .build_return(Some(&this.make_object(&num1, TypeIndex::number)))
                .unwrap();
        });
        let primitive_add1 = self.create_primitive("+1", |this, add1, _| {
            let argl = add1.get_first_param().unwrap().into_struct_value();
            let val = this.make_car(argl);
            let num = this.get_number(val).into_float_value();
            let num1 = this
                .builder
                .build_float_add(
                    num,
                    this.types
                        .types
                        .get(TypeIndex::number)
                        .into_float_type()
                        .const_float(1.0),
                    "add 1",
                )
                .unwrap();
            let result = this.make_object(&num1, TypeIndex::number);
            // this.print_object(result);
            this.builder.build_return(Some(&result)).unwrap();
        });

        // for some reason the problem with primitives - you guessed it is only with primitives, so
        // when we make one "non" primitive it works flawlesly
        // two places were primitves are broken at caller - or declaration and given the fact the
        // printing the result before returning prints the correct results it may be by calling?
        let add1 = {
            let env = self.registers.get(Register::Env);

            let env = self
                .builder
                .build_load(self.types.object, env, "load argl")
                .unwrap();
            let prev_bb = self.builder.get_insert_block().unwrap();

            let start_label = self.context.append_basic_block(self.current, "add1");
            self.labels.insert("add1".to_string(), start_label);
            self.builder.position_at_end(start_label);
            let argl = self.registers.get(Register::Argl);
            let argl = self
                .builder
                .build_load(self.types.object, argl, "load argl")
                .unwrap();
            let value = self.make_unchecked_car(argl.into_struct_value());
            let num = self.get_number(value).into_float_value();
            let num1 = self
                .builder
                .build_float_add(
                    num,
                    self.types
                        .types
                        .get(TypeIndex::number)
                        .into_float_type()
                        .const_float(1.0),
                    "add 1",
                )
                .unwrap();
            let result = self.make_object(&num1, TypeIndex::number);

            let val = self.registers.get(Register::Val);
            self.builder.build_store(val, result);
            let continue_reg = self.registers.get(Register::Continue);
            let register = self
                .builder
                .build_load(
                    self.types.object,
                    continue_reg,
                    &format!("load register continue"),
                )
                .unwrap();
            let label = self
                .unchecked_get_label(register.into_struct_value())
                .into_pointer_value();
            self.builder
                // we need all possible labels as destinations b/c indirect br requires a destination but we dont which one at compile time so we use all of them - maybe fixed with register_to_llvm_more_opt
                .build_indirect_branch(label, &self.labels.values().copied().collect_vec());
            self.builder.position_at_end(prev_bb);
            let lambda = self.make_object(
                &self.list_to_struct(
                    self.types.types.get(TypeIndex::lambda).into_struct_type(),
                    &[
                        self.make_object(
                            &unsafe { start_label.get_address().unwrap() },
                            TypeIndex::label,
                        )
                        .into(),
                        env,
                    ],
                ),
                TypeIndex::lambda,
            );
            (self.create_symbol("+1"), lambda)
        };
        let primitives = [
            ("newline", primitive_newline),
            ("=", primitive_eq),
            ("print", primitive_print),
            ("not", primitive_not),
            ("set_cdr!", primitive_set_cdr),
            ("set_car!", primitive_set_car),
            ("cons", primitive_cons),
            // ("+1", primitive_add1),
            ("-1", primitive_sub1),
        ];
        let accesors = self.init_accessors();
        let primitive_env = primitives
            .into_iter()
            .chain(accesors)
            .map(|(name, function)| self.make_primitive_pair(name, function))
            .chain(iter::once(add1))
            .fold(
                (self.empty(), self.empty()),
                |(symbols, functions), (symbol, function)| {
                    (
                        self.make_cons(symbol, symbols),
                        self.make_cons(function, functions),
                    )
                },
            );

        let primitive_env = (
            self.make_cons(self.create_symbol("nil"), primitive_env.0),
            self.make_cons(self.empty(), primitive_env.1),
        );
        let primitive_env = self.make_cons(primitive_env.0, primitive_env.1);
        let env = self.make_cons(primitive_env, self.empty());

        self.builder
            .build_store(self.registers.get(Register::Env), env)
            .unwrap();
    }

    // TODO: maybe make primitives be a block rather that a function and instead of doing an exit + unreachable with errors we could have an error block with a phi for for error string and ret code,
    // or maybe still use functiions for primitives but try to understand trampolines
    // the advantage of this is that the optimizer wouldn't get sometimes confused by unreacahbles, the disadvantage is if we go the primitive as block way is that  we have to use indirectbr for going to the primitive
    // currentyl we use error block approach with primitives being functions, and that just means that primitives must do exit + uncreachable on their own

    pub fn set_error(&self, reason: &str, code: i32) {
        self.error_phi.add_incoming(&[(
            &self.types.error.const_named_struct(&[
                self.builder
                    .build_global_string_ptr(reason, "error exit")
                    .unwrap()
                    .as_basic_value_enum(),
                self.context.i32_type().const_int(code as u64, false).into(),
            ]),
            self.builder.get_insert_block().unwrap(),
        )]);
    }
    pub fn export_ir(&self) -> String {
        self.module.to_string()
    }

    pub fn create_primitive(
        &mut self,
        name: &str,
        code: impl FnOnce(&mut Self, FunctionValue<'ctx>, BasicBlock<'ctx>),
    ) -> FunctionValue<'ctx> {
        self.create_function(name, self.types.primitive, code)
    }

    fn print_object(&self, obj: StructValue<'ctx>) {
        let print = self.module.get_function("print-obj").unwrap();
        self.builder
            .build_call(print, &[obj.into()], "print object")
            .unwrap();
    }
    fn compare_objects(&self, obj1: StructValue<'ctx>, obj2: StructValue<'ctx>) -> IntValue<'ctx> {
        let compare = self.module.get_function("eq-obj").unwrap();
        self.builder
            .build_call(compare, &[obj1.into(), obj2.into()], "compare objects")
            .unwrap()
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value()
    }
    /// creates a function with entry and puts builder at entry
    /// then calls code
    pub fn create_function(
        &mut self,
        name: &str,
        kind: FunctionType<'ctx>,
        code: impl FnOnce(&mut Self, FunctionValue<'ctx>, BasicBlock<'ctx>),
    ) -> FunctionValue<'ctx> {
        let function = self.module.add_function(name, kind, None);
        let (prev_error, prev_error_phi, prev, prev_function) = (
            self.error_block,
            self.error_phi,
            self.builder.get_insert_block().unwrap(),
            self.current,
        );

        let entry = self.context.append_basic_block(function, "entry");
        (self.error_block, self.error_phi) = self.init_error_handler(function);

        self.builder.position_at_end(entry);
        self.current = function;
        code(self, function, entry);
        self.builder.position_at_end(prev);
        if self.error_phi.count_incoming() == 0 {
            self.error_block.remove_from_function();
            self.error_phi.as_instruction().remove_from_basic_block();
        }
        (self.current, self.error_block, self.error_phi) =
            (prev_function, prev_error, prev_error_phi);
        function
    }
    fn init_error_handler(
        &self,
        function: FunctionValue<'ctx>,
    ) -> (BasicBlock<'ctx>, PhiValue<'ctx>) {
        init_error_handler(
            function,
            self.context,
            self.builder,
            self.types.error,
            self.functions.printf,
            self.functions.exit,
        )
    }

    pub fn compile(&mut self, instructions: Vec<Instruction>) {
        instructions
            .into_iter()
            // we scan looking for the labels and create new blocks for each label
            // we do not remove the label instruction b/c when we compile each instruction
            // we need to know when to start compiling un der a new block
            .inspect(|x| {
                if let Instruction::Label(l) = x {
                    let v = self.context.append_basic_block(self.current, l);
                    self.labels.insert(l.clone(), v);
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|inst| {
                self.compile_instructions(inst);
            });
        self.builder
            .build_return(Some(&self.context.i32_type().const_zero()))
            .unwrap();
        // self.fpm.run_on(&self.current);
        let fpm = PassManager::create(());
        // TODO: more and better optimizations

        // fpm.add_function_inlining_pass();
        // fpm.add_merge_functions_pass();
        // fpm.add_global_dce_pass();
        // fpm.add_ipsccp_pass();

        // makes hard to debug llvm ir
        // fpm.add_strip_symbol_pass();

        // fpm.add_constant_merge_pass();
        // fpm.add_new_gvn_pass();
        // fpm.add_instruction_combining_pass();
        // fpm.add_reassociate_pass();
        // fpm.add_gvn_pass();
        // fpm.add_basic_alias_analysis_pass();
        // fpm.add_promote_memory_to_register_pass();
        // fpm.add_aggressive_inst_combiner_pass();
        // // // doesn't work with current goto implementation
        // fpm.add_cfg_simplification_pass();
        // fpm.add_aggressive_dce_pass();
        // fpm.add_function_inlining_pass();
        // fpm.add_strip_dead_prototypes_pass();

        fpm.run_on(self.module);
    }

    fn truthy(&self, val: StructValue<'ctx>) -> IntValue<'ctx> {
        let ty = self.extract_type(val).unwrap().into_int_value();
        let is_not_bool = self
            .builder
            .build_int_compare(
                IntPredicate::NE,
                ty,
                self.context
                    .i32_type()
                    .const_int(TypeIndex::bool as u64, false),
                "not bool check",
            )
            .unwrap();
        let value = self
            .builder
            .build_extract_value(val, 1, "get object context")
            .unwrap();
        let value = self
            .builder
            .build_load(
                self.types.types.get(TypeIndex::bool),
                value.into_pointer_value(),
                "get bool value",
            )
            .unwrap();
        self.builder
            .build_or(is_not_bool, value.into_int_value(), "non bool or true")
            .unwrap()
    }

    fn compile_instructions(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Assign(r, e) => {
                let register = self.registers.get(r);
                let expr = self.compile_expr(e);
                self.builder.build_store(register, expr).unwrap();
            }
            Instruction::Test(p) => {
                self.builder
                    .build_store(self.flag, self.compile_perform(p))
                    .unwrap();
            }
            Instruction::Branch(l) => {
                let flag = self
                    .builder
                    .build_load(self.types.object, self.flag, "load flag")
                    .unwrap();
                // TODO: hack technically (in most cases) if we have a branch, the next instruction after it will be a label so we dont need a new block for each branch but rather we should either "peek" ahead to the next instruction to obtain the label
                // or encode the label as part of the branch variant
                let next_label = self.context.append_basic_block(self.current, "next-block");
                // eprintln!("{:?}", self.labels.keys());
                self.builder.build_conditional_branch(
                    self.truthy(flag.into_struct_value()),
                    *self.labels.get(&l).unwrap(),
                    next_label,
                );
                self.builder.position_at_end(next_label);
            }
            Instruction::Goto(g) => {
                match g {
                    Goto::Label(l) => {
                        self.builder
                            .build_unconditional_branch(*self.labels.get(&l).unwrap())
                            .unwrap();
                    }
                    Goto::Register(r) => {
                        let register = self.registers.get(r);
                        let register = self
                            .builder
                            .build_load(self.types.object, register, &format!("load register {r}"))
                            .unwrap();
                        let label = self
                            .unchecked_get_label(register.into_struct_value())
                            .into_pointer_value();
                        self.builder
                            // we need all possible labels as destinations b/c indirect br requires a destination but we dont which one at compile time so we use all of them - maybe fixed with register_to_llvm_more_opt
                            .build_indirect_branch(
                                label,
                                &self.labels.values().copied().collect_vec(),
                            );
                    }
                }
                // we create new block/label b/c goto should be last instruction for a block so this "dummy" label acts as a catch all for anything afterwords
                // realy only needed b/c for label instruction we assume we should just br to the label, but if we digit goto followed by new label we would have double br
                // note wwe mahe similiar problem with branch
                let next_label = self.context.append_basic_block(self.current, "next-block");
                self.builder.position_at_end(next_label);
            }
            Instruction::Save(reg) => {
                // self.make_printf(&format!("\nsave start {reg}\n"), vec![]);
                let register = reg;
                let prev_stack = self
                    .builder
                    .build_load(self.types.stack, self.stack, "load stack")
                    .unwrap();
                let prev_stack_ptr = self
                    .builder
                    .build_alloca(self.types.stack, "previous stack")
                    .unwrap();
                self.builder
                    .build_store(prev_stack_ptr, prev_stack)
                    .unwrap();
                let new_stack = self.types.stack.const_zero();
                let reg = self.registers.get(reg);
                let reg_value = self
                    .builder
                    .build_load(self.types.object, reg, "load register")
                    .unwrap();
                let new_stack = self
                    .builder
                    .build_insert_value(new_stack, reg_value, 0, "save register")
                    .unwrap();

                let new_stack = self
                    .builder
                    .build_insert_value(new_stack, prev_stack_ptr, 1, "save previous stack")
                    .unwrap();
                // self.make_printf(&format!("save {register}\n"), vec![]);
                self.builder.build_store(self.stack, new_stack).unwrap();
            }
            Instruction::Restore(reg) => {
                // self.make_printf(&format!("\nrestore start {reg}\n"), vec![]);
                let stack = self
                    .builder
                    .build_load(self.types.stack, self.stack, "stack")
                    .unwrap();
                let old_stack = self
                    .builder
                    .build_extract_value(stack.into_struct_value(), 1, "old stack")
                    .unwrap();
                let current = self
                    .builder
                    .build_extract_value(stack.into_struct_value(), 0, "current stack")
                    .unwrap();
                self.builder
                    .build_store(self.registers.get(reg), current)
                    .unwrap();
                let old_stack = self
                    .builder
                    .build_load(
                        self.types.stack,
                        old_stack.into_pointer_value(),
                        "load previous stack",
                    )
                    .unwrap();
                // self.make_printf(&format!("restore {reg}\n"), vec![]);
                self.builder.build_store(self.stack, old_stack).unwrap();
            }
            Instruction::Perform(p) => {
                self.compile_perform(p);
            }
            Instruction::Label(l) => {
                let label = self.labels.get(&l);
                self.builder
                    .build_unconditional_branch(*label.unwrap())
                    .unwrap();
                self.builder.position_at_end(*self.labels.get(&l).unwrap());
            }
        }
    }

    fn compile_expr(&mut self, expr: Expr) -> StructValue<'ctx> {
        match expr {
            Expr::Const(c) => self.compile_const(c),
            Expr::Label(l) => {
                let label_address = unsafe { self.labels.get(&l).unwrap().get_address() }.unwrap();
                self.make_object(&label_address, TypeIndex::label)
            }
            Expr::Register(r) => {
                let reg = self.registers.get(r);
                self.builder
                    .build_load(self.types.object, reg, &format!("load register {r}"))
                    .unwrap()
                    .into_struct_value()
            }
            Expr::Op(p) => self.compile_perform(p),
        }
    }

    // lookup variable for set! and plain variable lookup
    // returns a tuple contatining the cons of the found variable and the rest of the vars in the frame
    fn lookup_variable(&self, var: StructValue<'ctx>, env: StructValue<'ctx>) -> StructValue<'ctx> {
        let lookup_entry_bb = self
            .context
            .append_basic_block(self.current, "lookup-entry");
        let lookup_bb = self.context.append_basic_block(self.current, "lookup");
        let scan_bb = self.context.append_basic_block(self.current, "scan");
        let next_env_bb = self.context.append_basic_block(self.current, "next-env");
        let check_bb = self.context.append_basic_block(self.current, "check");
        let found_bb = self.context.append_basic_block(self.current, "found");
        let scan_next_bb = self.context.append_basic_block(self.current, "scan-next");

        let env_ptr = self.builder.build_alloca(self.types.object, "env").unwrap();
        self.builder.build_store(env_ptr, env).unwrap();
        self.builder
            .build_unconditional_branch(lookup_entry_bb)
            .unwrap();

        self.builder.position_at_end(lookup_entry_bb);
        self.set_error("unbound variable\n", 1);
        let env_load = self
            .builder
            .build_load(self.types.object, env_ptr, "load env")
            .unwrap();
        self.builder.build_conditional_branch(
            self.is_hempty(env_load.into_struct_value()),
            self.error_block,
            lookup_bb,
        );

        self.builder.position_at_end(lookup_bb);
        let frame = self.make_unchecked_car(env_load.into_struct_value());
        let vars_pointer = self
            .builder
            .build_alloca(self.types.object, "vars")
            .unwrap();
        let vals_pointer = self
            .builder
            .build_alloca(self.types.object, "vals")
            .unwrap();
        self.builder
            .build_store(vars_pointer, self.make_unchecked_car(frame))
            .unwrap();
        self.builder
            .build_store(vals_pointer, self.make_unchecked_cdr(frame))
            .unwrap();
        self.builder.build_unconditional_branch(scan_bb).unwrap();

        self.builder.position_at_end(scan_bb);
        let vars = self
            .builder
            .build_load(self.types.object, vars_pointer, "vars")
            .unwrap()
            .into_struct_value();
        self.builder
            // vars might not be write theing here maybe vars pointer loaded b./c that what scan next sets
            .build_conditional_branch(self.is_hempty(vars), next_env_bb, check_bb)
            .unwrap();

        self.builder.position_at_end(next_env_bb);
        let new_env = self.make_unchecked_cdr(env_load.into_struct_value());
        self.builder.build_store(env_ptr, new_env).unwrap();
        self.builder
            .build_unconditional_branch(lookup_entry_bb)
            .unwrap();

        self.builder.position_at_end(check_bb);
        let vars_load = self
            .builder
            .build_load(self.types.object, vars_pointer, "load vars")
            .unwrap()
            .into_struct_value();
        let vars_car = self.make_unchecked_car(vars_load);
        self.builder.build_conditional_branch(
            self.compare_str(Self::unchecked_get_symbol, var, vars_car),
            found_bb,
            scan_next_bb,
        );

        self.builder.position_at_end(scan_next_bb);
        let vals_load = self
            .builder
            .build_load(self.types.object, vals_pointer, "load vals")
            .unwrap()
            .into_struct_value();
        self.builder
            .build_store(vars_pointer, self.make_unchecked_cdr(vars_load))
            .unwrap();
        self.builder
            .build_store(vals_pointer, self.make_unchecked_cdr(vals_load))
            .unwrap();
        self.builder.build_unconditional_branch(scan_bb).unwrap();

        self.builder.position_at_end(found_bb);
        self.builder
            .build_load(self.types.object, vals_pointer, "load vals")
            .unwrap()
            .into_struct_value()
    }

    fn compare_str(
        &self,
        extract_str: fn(&Self, StructValue<'ctx>) -> BasicValueEnum<'ctx>,
        s1: StructValue<'ctx>,
        s2: StructValue<'ctx>,
    ) -> IntValue<'ctx> {
        let s1 = extract_str(self, s1).into_struct_value();
        let s2 = extract_str(self, s2).into_struct_value();
        let len1 = self
            .builder
            .build_extract_value(s1, 0, "get str length")
            .unwrap()
            .into_int_value();
        let len2 = self
            .builder
            .build_extract_value(s2, 0, "get str length")
            .unwrap()
            .into_int_value();
        let s1 = self.builder.build_extract_value(s1, 1, "get str").unwrap();
        let s2 = self.builder.build_extract_value(s2, 1, "get str").unwrap();
        let str_len_matches = self
            .builder
            .build_int_compare(IntPredicate::EQ, len1, len2, "len matches")
            .unwrap();
        let str_small_size = self
            .builder
            .build_select(
                self.builder
                    .build_int_compare(IntPredicate::SLT, len1, len2, "smaller")
                    .unwrap(),
                len1,
                len2,
                "str smallaest size",
            )
            .unwrap();
        let str_cmp = self
            .builder
            .build_call(
                self.functions.strncmp,
                &[s1.into(), s2.into(), str_small_size.into()],
                "strcmp",
            )
            .unwrap()
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value();
        // strncmp returns
        // Negative value if lhs appears before rhs in lexicographical order.
        // Zero if lhs and rhs compare equal, or if count is zero.
        // Positive value if lhs appears after rhs in lexicographical order
        // so to know that the strings are the same we check that the result of strncmp is zero
        let is_same = self
            .builder
            .build_int_compare(
                IntPredicate::EQ,
                str_cmp,
                self.context.i32_type().const_zero(),
                "is same string",
            )
            .unwrap();
        self.builder
            .build_and(
                str_len_matches,
                self.builder
                    .build_int_cast(is_same, self.context.bool_type(), "")
                    .unwrap(),
                "eq?",
            )
            .unwrap()
    }

    fn make_printf(&self, string: &str, values: Vec<BasicValueEnum<'ctx>>) {
        let format = self
            .builder
            .build_global_string_ptr(string, "printf-format")
            .unwrap()
            .as_pointer_value();
        let mut values: Vec<_> = values.into_iter().map(Into::into).collect();
        values.insert(0, format.into());
        self.builder
            .build_call(self.functions.printf, &values, "printf debug")
            .unwrap();
    }

    fn compile_perform(&mut self, action: Perform) -> StructValue<'ctx> {
        // TODO: dont compile all the args before the match on operation b/c some of them could be better written or something if we had the actual types at compile time
        // similiar to the idea of combining the operastion and its args
        // TODO: each operation could be function or block(s) + phi so instead of compiling the following code each time we find a perform we would do it only once
        let args: Vec<_> = action
            .args()
            .iter()
            .cloned()
            .map(|e| self.compile_expr(e))
            .collect();
        match action.op() {
            Operation::LookupVariableValue => {
                let var = args[0];
                let env = args[1];
                self.make_unchecked_car(self.lookup_variable(var, env))
            }
            Operation::CompiledProcedureEnv => {
                let proc = args[0];
                let proc = self.unchecked_get_lambda(proc).into_struct_value();
                self.builder
                    .build_extract_value(proc, 1, "proc env")
                    .unwrap()
                    .into_struct_value()
            }
            Operation::CompiledProcedureEntry => {
                let proc = args[0];
                let proc = self.unchecked_get_lambda(proc).into_struct_value();
                self.builder
                    .build_extract_value(proc, 0, "proc entry")
                    .unwrap()
                    .into_struct_value()
            }
            Operation::DefineVariable => {
                let var = args[0];
                let val = args[1];
                let env = args[2];
                let frame = self.make_unchecked_car(env);
                // set the vars part of the frame
                self.make_unchecked_set_car(
                    frame,
                    self.make_cons(var, self.make_unchecked_car(frame)),
                );
                // set the vals part of the frame
                self.make_unchecked_set_cdr(
                    frame,
                    self.make_cons(val, self.make_unchecked_cdr(frame)),
                );
                self.empty()
            }
            Operation::ApplyPrimitiveProcedure => {
                let proc = args[0];
                let argl = args[1];
                let proc = self.unchecked_get_primitive(proc);
                self.builder
                    .build_indirect_call(
                        self.types.primitive,
                        proc.into_pointer_value(),
                        &[argl.into()],
                        "call primitive",
                    )
                    .unwrap()
                    .try_as_basic_value()
                    .unwrap_left()
                    .into_struct_value()
            }
            Operation::ExtendEnvironment => {
                let vars = args[0];
                let vals = args[1];
                let env = args[2];

                let frame = self.make_cons(
                    self.make_cons(vars, self.empty()),
                    self.make_cons(vals, self.empty()),
                );
                self.make_cons(frame, env)
            }
            Operation::Cons => {
                let car = *args.first().unwrap();
                let cdr = *args.get(1).unwrap();
                self.make_cons(car, cdr)
            }
            Operation::SetVariableValue => {
                let var = args[0];
                let new_val = args[1];
                let env = args[2];
                self.make_unchecked_set_car(self.lookup_variable(var, env), new_val);
                self.empty()
            }
            Operation::False => {
                let boolean = self
                    .builder
                    .build_not(self.truthy(*args.first().unwrap()), "not truthy")
                    .unwrap();
                self.make_object(&boolean, TypeIndex::bool)
            }
            Operation::RandomBool => {
                let bool = self
                    .builder
                    .build_call(self.functions.rand, &[], "random bool")
                    .unwrap()
                    .try_as_basic_value()
                    .unwrap_left();
                let bool = self
                    .builder
                    .build_int_signed_rem(
                        bool.into_int_value(),
                        self.context.i32_type().const_int(2, false),
                        "truncate to bool",
                    )
                    .unwrap();

                self.make_object(&bool, TypeIndex::bool)
            }
            Operation::MakeCompiledProcedure => {
                let compiled_procedure_string = self.create_symbol("compiled-procedure");
                let compiled_procedure_string =
                    self.make_object(&compiled_procedure_string, TypeIndex::symbol);
                let compiled_procedure_entry = *args.first().unwrap();
                let compiled_procedure_env = args.get(1).unwrap();
                let compiled_procedure_type = args.get(2).unwrap();
                self.make_object(
                    &self.list_to_struct(
                        self.types.types.get(TypeIndex::lambda).into_struct_type(),
                        &[
                            compiled_procedure_entry.into(),
                            (*compiled_procedure_env).into(),
                            (*compiled_procedure_type).into(),
                        ],
                    ),
                    TypeIndex::lambda,
                )
            }
            Operation::PrimitiveProcedure => {
                self.make_object(&self.is_primitive(args[0]), TypeIndex::bool)
            }
            Operation::MakeThunk => {
                let entry = args[0];
                let env = args[1];
                let thunk = self.list_to_struct(
                    self.types.types.thunk.into_struct_type(),
                    &[entry.into(), env.into()],
                );
                self.make_object(&thunk, TypeIndex::thunk)
            }
            Operation::ThunkEntry => {
                let thunk = args[0];
                let thunk = self.unchecked_get_thunk(thunk).into_struct_value();
                self.builder
                    .build_extract_value(thunk, 0, "thunk entry")
                    .unwrap()
                    .into_struct_value()
            }
            Operation::ThunkEnv => {
                let thunk = args[0];
                let thunk = self.unchecked_get_thunk(thunk).into_struct_value();
                self.builder
                    .build_extract_value(thunk, 1, "thunk env")
                    .unwrap()
                    .into_struct_value()
            }
            Operation::NotThunk => {
                let thunk = args[0];
                let is_thunk = self.is_thunk(thunk);
                let is_not_thunk = self.builder.build_not(is_thunk, "not thunk").unwrap();
                self.make_object(&is_not_thunk, TypeIndex::bool)
            }
            Operation::VariadiacProcedure => {
                let proc = args[0];
                let proc = self.unchecked_get_lambda(proc).into_struct_value();
                let lambda_type = self
                    .get_number(
                        self.builder
                            .build_extract_value(proc, 2, "proc entry")
                            .unwrap()
                            .into_struct_value(),
                    )
                    .into_float_value();
                self.make_object(
                    &self
                        .builder
                        .build_or(
                            self.builder
                                .build_float_compare(
                                    FloatPredicate::OEQ,
                                    lambda_type,
                                    self.types
                                        .types
                                        .number
                                        .into_float_type()
                                        .const_float(ZERO_VARIADIAC_ARG as f64),
                                    "is zero variadiac",
                                )
                                .unwrap(),
                            self.builder
                                .build_float_compare(
                                    FloatPredicate::OEQ,
                                    lambda_type,
                                    self.types
                                        .types
                                        .number
                                        .into_float_type()
                                        .const_float(ONE_VARIADIAC_ARG as f64),
                                    "is one variadiac",
                                )
                                .unwrap(),
                            "is variadiac",
                        )
                        .unwrap(),
                    TypeIndex::bool,
                )
            }
        }
    }

    fn list_to_struct(
        &self,
        struct_ty: StructType<'ctx>,
        things: &[BasicValueEnum<'ctx>],
    ) -> StructValue<'ctx> {
        things
            .iter()
            .enumerate()
            .fold(struct_ty.const_zero(), |strcut_val, (i, item)| {
                self.builder
                    .build_insert_value(strcut_val, *item, i as u32, "insert into struct")
                    .unwrap()
                    .into_struct_value()
            })
    }

    // TODO: make unchecked versions of *car and *cdr function to avoid dealing with br(s) added self.get_cons (which is causing problems with multiple br(s) per block in variable lookup and other places)
    // or turn car,cdr into llvm functions
    fn car(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 0, "get car")
            .unwrap()
            .into_pointer_value()
    }

    fn cdr(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 1, "get cdr")
            .unwrap()
            .into_pointer_value()
    }
    fn make_car(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.car(cons), "load car")
            .unwrap()
            .into_struct_value()
    }

    fn make_cdr(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.cdr(cons), "load cdr")
            .unwrap()
            .into_struct_value()
    }

    fn make_set_car(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder.build_store(self.car(cons), new_value).unwrap();
        self.empty()
    }

    fn make_set_cdr(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder.build_store(self.cdr(cons), new_value).unwrap();
        self.empty()
    }
    fn unchecked_car(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.unchecked_get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 0, "get car")
            .unwrap()
            .into_pointer_value()
    }

    fn unchecked_cdr(&self, cons: StructValue<'ctx>) -> PointerValue<'ctx> {
        let cons = self.unchecked_get_cons(cons).into_struct_value();
        self.builder
            .build_extract_value(cons, 1, "get cdr")
            .unwrap()
            .into_pointer_value()
    }
    fn make_unchecked_car(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.unchecked_car(cons), "load car")
            .unwrap()
            .into_struct_value()
    }

    fn make_unchecked_cdr(&self, cons: StructValue<'ctx>) -> StructValue<'ctx> {
        self.builder
            .build_load(self.types.object, self.unchecked_cdr(cons), "load cdr")
            .unwrap()
            .into_struct_value()
    }

    fn make_unchecked_set_car(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder
            .build_store(self.unchecked_car(cons), new_value)
            .unwrap();
        self.empty()
    }

    fn make_unchecked_set_cdr(
        &self,
        cons: StructValue<'ctx>,
        new_value: StructValue<'ctx>,
    ) -> StructValue<'ctx> {
        self.builder
            .build_store(self.unchecked_cdr(cons), new_value)
            .unwrap();
        self.empty()
    }

    make_accessors!(make_unchecked_cadr make_unchecked_car make_unchecked_cdr);
    make_accessors!(make_unchecked_cddr make_unchecked_cdr make_unchecked_cdr);
    make_accessors!(make_unchecked_caddr make_unchecked_car make_unchecked_cddr);

    make_accessors!(make_caar make_car make_car);
    make_accessors!(make_cadr make_car make_cdr);
    make_accessors!(make_cdar make_cdr make_car);
    make_accessors!(make_cddr make_cdr make_cdr);
    make_accessors!(make_caaar make_car make_caar);
    make_accessors!(make_caadr make_car make_cadr);
    make_accessors!(make_cadar make_car make_cdar);
    make_accessors!(make_caddr make_car make_cddr);
    make_accessors!(make_cdaar make_cdr make_caar);
    make_accessors!(make_cdadr make_cdr make_cadr);
    make_accessors!(make_cddar make_cdr make_cdar);
    make_accessors!(make_cdddr make_cdr make_cddr);
    make_accessors!(make_caaaar make_car make_caaar);
    make_accessors!(make_caaadr make_car make_caadr);
    make_accessors!(make_caadar make_car make_cadar);
    make_accessors!(make_caaddr make_car make_caddr);
    make_accessors!(make_cadaar make_car make_cdaar);
    make_accessors!(make_cadadr make_car make_cdadr);
    make_accessors!(make_caddar make_car make_cddar);
    make_accessors!(make_cadddr make_car make_cdddr);
    make_accessors!(make_cdaaar make_cdr make_caaar);
    make_accessors!(make_cdaadr make_cdr make_caadr);
    make_accessors!(make_cdadar make_cdr make_cadar);
    make_accessors!(make_cdaddr make_cdr make_caddr);
    make_accessors!(make_cddaar make_cdr make_cdaar);
    make_accessors!(make_cddadr make_cdr make_cdadr);
    make_accessors!(make_cdddar make_cdr make_cddar);
    make_accessors!(make_cddddr make_cdr make_cdddr);

    fn make_cons(&self, car: StructValue<'ctx>, cdr: StructValue<'ctx>) -> StructValue<'ctx> {
        let cons = self.types.cons.const_zero();
        let car_ptr = self
            .builder
            .build_malloc(self.types.object, "car ptr")
            .unwrap();
        let cdr_ptr = self
            .builder
            .build_malloc(self.types.object, "cdr ptr")
            .unwrap();
        self.builder.build_store(car_ptr, car).unwrap();
        self.builder.build_store(cdr_ptr, cdr).unwrap();
        let cons = self
            .builder
            .build_insert_value(cons, car_ptr, 0, "insert car - cons")
            .unwrap()
            .into_struct_value();
        let cons = self
            .builder
            .build_insert_value(cons, cdr_ptr, 1, "insert cdr - cons")
            .unwrap()
            .into_struct_value();
        self.make_object(&cons, TypeIndex::cons)
    }

    fn empty(&self) -> StructValue<'ctx> {
        self.types.object.const_zero()
    }

    fn make_object(&self, obj: &dyn BasicValue<'ctx>, index: TypeIndex) -> StructValue<'ctx> {
        let value_ptr = self
            .builder
            // we use malloc because create_alloca is stack based and can be "lost" after returning
            // from a function
            .build_malloc(self.types.types.get(index), "object value")
            .unwrap();
        self.builder
            .build_store(value_ptr, obj.as_basic_value_enum())
            .unwrap();
        let obj = self.types.object.const_zero();
        let obj = self
            .builder
            .build_insert_value(
                obj,
                self.context.i32_type().const_int(index as u64, false),
                0,
                "insert type object",
            )
            .unwrap();
        let obj = self
            .builder
            .build_insert_value(obj, value_ptr, 1, "insert value object")
            .unwrap();
        obj.into_struct_value()
    }

    fn compile_const(&mut self, constant: Const) -> StructValue<'ctx> {
        match constant {
            Const::Empty => self.empty(),
            Const::String(s) => self.create_string(&s),
            Const::Symbol(s) => self.create_symbol(&s), // TODO: intern the symbol
            Const::Number(n) => {
                let number = self.context.f64_type().const_float(n);
                self.make_object(&number, TypeIndex::number)
            }
            Const::Boolean(b) => {
                let boolean = self.context.bool_type().const_int(u64::from(b), false);
                self.make_object(&boolean, TypeIndex::bool)
            }
            Const::List(car, cdr) => {
                let cons: StructValue<'_> = self.types.cons.const_zero();
                let mut compile_and_add = |expr, name, cons, index| {
                    let expr_compiled = self.compile_expr(expr);
                    let expr = self.builder.build_malloc(self.types.object, name).unwrap();
                    self.builder.build_store(expr, expr_compiled).unwrap();
                    self.builder
                        .build_insert_value(cons, expr, index, &format!("insert {name}"))
                        .unwrap()
                };
                let cons = compile_and_add(*car, "car", cons.as_aggregate_value_enum(), 0);
                let cons = compile_and_add(*cdr, "cdr", cons, 1);
                self.make_object(&cons, TypeIndex::cons)
            }
        }
    }

    fn create_symbol(&self, s: &str) -> StructValue<'ctx> {
        self.make_object(&self.create_string_part(s), TypeIndex::symbol)
    }
    fn create_string(&self, s: &str) -> StructValue<'ctx> {
        self.make_object(&self.create_string_part(s), TypeIndex::string)
    }

    fn create_string_part(&self, s: &str) -> StructValue<'ctx> {
        let strlen = s.chars().count();
        let global_str = self
            .builder
            .build_global_string_ptr(s, s)
            .unwrap()
            .as_pointer_value();
        let obj = self.types.string.const_zero();
        let add_to_string = |string_object, name, expr, index| {
            self.builder
                .build_insert_value(string_object, expr, index, &format!("insert {name}"))
                .unwrap()
        };
        add_to_string(
            add_to_string(
                obj,
                "string length",
                self.context
                    .i32_type()
                    .const_int(strlen as u64, false)
                    .as_basic_value_enum(),
                0,
            )
            .into_struct_value(),
            "string data",
            global_str.as_basic_value_enum(),
            1,
        )
        .into_struct_value()
    }
}

fn init_error_handler<'ctx>(
    function: FunctionValue<'ctx>,
    context: &'ctx Context,
    builder: &Builder<'ctx>,
    error: StructType<'ctx>,
    printf: FunctionValue<'ctx>,
    exit: FunctionValue<'ctx>,
) -> (BasicBlock<'ctx>, PhiValue<'ctx>) {
    let error_block = context.append_basic_block(function, "error");
    builder.position_at_end(error_block);
    // error phi
    let error_phi = builder.build_phi(error, "error phi").unwrap();
    {
        let error_msg = builder
            .build_extract_value(
                error_phi.as_basic_value().into_struct_value(),
                0,
                "error_msg",
            )
            .unwrap();
        let error_code = builder
            .build_extract_value(
                error_phi.as_basic_value().into_struct_value(),
                1,
                "error_code",
            )
            .unwrap();
        builder
            .build_call(printf, &[error_msg.into()], "print")
            .unwrap();
        builder
            .build_call(exit, &[error_code.into()], "print")
            .unwrap();
        builder.build_unreachable().unwrap();
    }
    (error_block, error_phi)
}
