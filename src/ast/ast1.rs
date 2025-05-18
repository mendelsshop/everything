use std::{collections::HashMap, fmt, hash::Hash};

use itertools::Itertools;

use crate::interior_mut::{MUTEX, RC};

use super::{Ast, ModuleType, Param};

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

// TODO: maybe: do this as part of expander's compile, this would mean we would still need
// functionss in the ast
// and evaluator would need to work with this
// and we would need a way to get back regular for going back into expander
// so functions would still take produce Ast but evaluator would reduce Ast1 -> Ast as evaling b/c
// pair and syntax would still be Ast and theoritcally and of the other constants (number,strings,
// ....) only special forms cannot be Ast
#[derive(Debug, Clone, PartialEq)]
// TODO: #%expression form, begin0
pub enum Ast1 {
    // Maybe all not speical forms should be just Be
    // coreesponds to data
    Basic(Ast),

    // special forms
    If(Box<Ast1>, Box<Ast1>, Box<Ast1>),
    DefineValues(Vec<RC<str>>, Box<Ast1>),
    LetValues(Vec<(Vec<RC<str>>, Ast1)>, Box<Ast1>),
    LetRecValues(Vec<(Vec<RC<str>>, Ast1)>, Box<Ast1>),
    Lambda(Param, Box<Ast1>),
    Application(Box<Ast1>, Vec<Ast1>),
    Expression(Box<Ast1>),
    Begin(Vec<Ast1>),
    Begin0(Vec<Ast1>),
    Set(RC<str>, Box<Ast1>),
    Quote(Ast),
    Stop(Option<Box<Ast1>>),
    Skip,
    Loop(Box<Ast1>),
    Module(RC<str>, ModuleType),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label(pub RC<str>);

impl fmt::Display for Ast1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TODO: maybe datum to syntax it
            Self::Basic(s) => write!(f, "{s}"),
            Self::Application(f0, a0) => {
                write!(f, "({f0} {})", a0.iter().map(ToString::to_string).join(" "))
            }

            Self::If(cond, cons, alt) => write!(f, "(if {cond} {cons} {alt})"),
            Self::DefineValues(v, val) => todo!(),
            // write!(f, "(define {v} {val})"),
            Self::Lambda(param, body) => write!(f, "(lambda ({param} {body})",),
            Self::Begin(b) => write!(f, "(begin {})", b.iter().map(ToString::to_string).join(" ")),
            Self::Begin0(b) => write!(
                f,
                "(begin0 {})",
                b.iter().map(ToString::to_string).join(" ")
            ),
            Self::Set(v, val) => write!(f, "(set! {v} {val})"),
            Self::Quote(q) => write!(f, ";{q}"),
            Self::Loop(l) => write!(f, "(loop {l}"),
            Self::Stop(s) => write!(
                f,
                "(stop{})",
                s.as_ref().map_or_else(String::new, |s| format!(" {s}"))
            ),
            Self::Skip => write!(f, "skip"),
            Self::LetRecValues(r, b) => write!(f, "(letrec-values (...) {b})"),
            Self::LetValues(r, b) => write!(f, "(let-values (...) {b})"),
            Self::Expression(e) => write!(f, "(#%expression {e})"),
            Self::Module(name, _type) => write!(f, "(module {name})"),
        }
    }
}
pub fn immutable_add_to_vec<T>(mut v: Vec<T>, x: T) -> Vec<T> {
    v.push(x);
    v
}
pub fn immutable_add_to_hashmap<K: Hash + Eq, V>(
    mut h: HashMap<K, V>,
    k: K,
    v: V,
) -> HashMap<K, V> {
    h.insert(k, v);
    h
}
