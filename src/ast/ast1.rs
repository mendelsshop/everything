use std::fmt;

// use inkwell::values::StructValue;
use itertools::Itertools;

use crate::{
    // codegen::Compiler,
    interior_mut::{MUTEX, RC},
};

use super::{syntax::Syntax, Ast, AstTransformFrom, Boolean, Param, Symbol};

// Ast1 is Ast besides for Functions which are only used for macros
#[derive(Clone, PartialEq, Debug)]
pub struct Pair(pub Ast1, pub Ast1);
#[derive(Clone, PartialEq, Debug)]
pub enum Ast1 {
    Pair(Box<Pair>),
    TheEmptyList,
    String(RC<str>),
    Syntax(Box<Syntax<Ast1>>),
    Number(f64),
    Boolean(Boolean),
    Symbol(Symbol),
    Label(RC<str>),
}
#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub inner: RC<MUTEX<(Ast1, Ast1, Ast1)>>,
}

// TODO: flatten trait for quotation
// pub trait FlattenAst<'a, 'ctx> {
//     fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx>;
// }

// impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Ast1 {
//     fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
//         match self {
//             Self::Boolean(b) => compiler.const_boolean(b),
//             Self::Number(n) => compiler.const_number(n),
//             Self::String(c) => compiler.const_string(&c),
//             Self::Ident(i) => compiler.const_symbol(&i),
//             Self::Application(a) => a.flatten(compiler),
//             Self::Label(_) => todo!(),
//             Self::FnParam(p) => compiler.const_symbol(&format!("'{p}'").into()),
//         }
//     }
// }

// impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Vec<Ast1> {
//     fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
//         fn list_to_tree<'ctx>(
//             list: Vec<Ast1>,
//             compiler: &mut Compiler<'_, 'ctx>,
//             n: usize,
//         ) -> (StructValue<'ctx>, Vec<Ast1>) {
//             if n == 0 {
//                 (compiler.hempty(), list)
//             } else {
//                 let left_size = (n - 1) / 2;
//                 let (left_tree, mut non_left_tree) = list_to_tree(list, compiler, left_size);
//
//                 let this = non_left_tree.remove(0).flatten(compiler);
//
//                 let right_size = n - (left_size + 1);
//                 let (right_tree, remaining) = list_to_tree(non_left_tree, compiler, right_size);
//                 (compiler.const_cons(left_tree, this, right_tree), remaining)
//             }
//         }
//         let n = self.len();
//         let partial_tree = list_to_tree(self, compiler, n);
//
//         partial_tree.0
//     }
// }

impl fmt::Display for Ast1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pair(pair) => {
                let mut string = pair.0.to_string();
                let mut second = pair.1.clone();
                while let Self::Pair(pair) = second {
                    string = format!("{string} {}", pair.0);
                    second = pair.1;
                }
                if second != Self::TheEmptyList {
                    string = format!("{string} . {second}");
                }
                write!(f, "({string})")
            }
            Self::String(f0) => write!(f, "{f0}"),
            Self::Syntax(s) => write!(f, "#'{}", s.0.clone().syntax_to_datum()),
            Self::Number(n) => write!(f, "{n}"),
            Self::Symbol(s) => write!(f, "'{s}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Label(f0) => write!(f, "@{f0}"),
            Self::TheEmptyList => write!(f, "()"),
        }
    }
}
impl Ast1 {
    pub(crate) fn syntax_to_datum(self) -> Self {
        match self {
            Self::Syntax(s) => s.0.syntax_to_datum(),
            Self::Pair(pair) => Self::Pair(Box::new(Pair(
                pair.0.syntax_to_datum(),
                pair.1.syntax_to_datum(),
            ))),
            _ => self,
        }
    }
}

impl TryFrom<Ast> for Ast1 {
    type Error = String;

    fn try_from(value: Ast) -> Result<Self, Self::Error> {
        match value {
            Ast::Pair(pair) => Ok(Self::Pair(Box::new(Pair(
                pair.0.try_into()?,
                pair.1.try_into()?,
            )))),
            Ast::TheEmptyList => Ok(Self::TheEmptyList),
            Ast::String(s) => Ok(Self::String(s)),
            Ast::Syntax(syntax) => Ok(Self::Syntax({
                Box::new(Syntax(syntax.0.try_into()?, syntax.1, syntax.2, syntax.3))
            })),
            Ast::Number(n) => Ok(Self::Number(n)),
            Ast::Boolean(boolean) => Ok(Self::Boolean(boolean)),
            Ast::Symbol(symbol) => Ok(Self::Symbol(symbol)),
            Ast::Function(function) => Err(format!("cannot have function, found {function}")),
            Ast::Label(l) => Ok(Self::Label(l)),
        }
    }
}
impl<A: Into<RC<str>>> From<A> for Ast1 {
    fn from(value: A) -> Self {
        Self::Symbol(Symbol(value.into()))
    }
}
