use std::fmt;

use inkwell::values::StructValue;
use itertools::Itertools;

use crate::{
    codegen::Compiler,
    interior_mut::{MUTEX, RC},
};

use super::Boolean;

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
        }
    }
}

impl<A: Into<RC<str>>> From<A> for Ast1 {
    fn from(value: A) -> Self {
        Self::Ident(value.into())
    }
}
