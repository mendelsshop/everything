use std::fmt;

use inkwell::values::StructValue;
use itertools::Itertools;

use crate::{
    codegen::Compiler,
    interior_mut::{MUTEX, RC},
};

use super::{Boolean, Varidiac};

#[derive(Clone, PartialEq, Debug)]
pub enum ModuleType {
    Inline(Vec<Ast1>),
    Path(RC<str>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Ast1 {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Ident(RC<str>),
    Application(Vec<Ast1>),
    Label(RC<str>),
    // should simlify to ident or the like ...
    FnParam(usize),

    If(Box<Ast1>, Box<Ast1>, Box<Ast1>),
    Define(RC<str>, Box<Ast1>),
    Lambda(usize, Option<Varidiac>, Vec<Ast1>),
    Begin(Vec<Ast1>),
    Set(RC<str>, Box<Ast1>),
    Quote(Box<Ast1>),
    Stop(Option<Box<Ast1>>),
    Loop(Box<Ast1>),
    Module(RC<str>, ModuleType),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub inner: RC<MUTEX<(Ast1, Ast1, Ast1)>>,
}

// TODO: flatten trait for quotation
pub trait FlattenAst<'a, 'ctx> {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx>;
}

impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Ast1 {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
        match self {
            Self::Bool(b) => compiler.const_boolean(b),
            Self::Number(n) => compiler.const_number(n),
            Self::String(c) => compiler.const_string(&c),
            Self::Ident(i) => compiler.const_symbol(&i),
            Self::Application(a) => a.flatten(compiler),
            Self::Label(_) => todo!(),
            Self::FnParam(p) => compiler.const_symbol(&format!("'{p}'").into()),
            _ => todo!(),
        }
    }
}

impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Vec<Ast1> {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
        fn list_to_tree<'ctx>(
            list: Vec<Ast1>,
            compiler: &mut Compiler<'_, 'ctx>,
            n: usize,
        ) -> (StructValue<'ctx>, Vec<Ast1>) {
            if n == 0 {
                (compiler.hempty(), list)
            } else {
                let left_size = (n - 1) / 2;
                let (left_tree, mut non_left_tree) = list_to_tree(list, compiler, left_size);

                let this = non_left_tree.remove(0).flatten(compiler);

                let right_size = n - (left_size + 1);
                let (right_tree, remaining) = list_to_tree(non_left_tree, compiler, right_size);
                (compiler.const_cons(left_tree, this, right_tree), remaining)
            }
        }
        let n = self.len();
        let partial_tree = list_to_tree(self, compiler, n);

        partial_tree.0
    }
}

impl fmt::Display for Ast1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(f0) => write!(f, "{f0}"),
            Self::Number(f0) => write!(f, "{f0}"),
            Self::String(f0) => write!(f, "{f0}"),
            Self::Ident(f0) => write!(f, "{f0}"),
            Self::Application(f0) => {
                write!(f, "({})", f0.iter().map(ToString::to_string).join(" "))
            }
            Self::Label(f0) => write!(f, "@{f0}"),
            Self::FnParam(f0) => write!(f, "'{f0}'"),
            Self::If(cond, cons, alt) => write!(f, "(if {cond} {cons} {alt})"),
            Self::Define(v, val) => write!(f, "(define {v} {val})"),
            Self::Lambda(argc, vairdiac, body) => write!(
                f,
                "(lambda ({argc}{}) {})",
                vairdiac
                    .as_ref()
                    .map_or_else(String::new, |s| format!(" {s}")),
                body.iter().map(ToString::to_string).join(" ")
            ),
            Self::Begin(b) => write!(f, "(begin {})", b.iter().map(ToString::to_string).join(" ")),
            Self::Set(v, val) => write!(f, "(set! {v} {val})"),
            Self::Quote(q) => write!(f, ";{q}"),
            Self::Loop(l) => write!(f, "(loop {l}"),
            Self::Stop(s) => write!(
                f,
                "(stop{})",
                s.as_ref()
                    .map(|s| format!(" {s}"))
                    .unwrap_or_else(|| String::new())
            ),
            Self::Module(name, _type) => write!(f, "(module {})", name),
        }
    }
}

impl<A: Into<RC<str>>> From<A> for Ast1 {
    fn from(value: A) -> Self {
        Self::Ident(value.into())
    }
}
