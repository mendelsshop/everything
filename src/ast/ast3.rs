use crate::{codegen::multimap::MultiMap, interior_mut::RC};

use super::{ast2::Ast2, map_box, Boolean, Varidiac};
use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Ast3 {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Ident(RC<str>),
    Application(Vec<Ast3>),
    Label(RC<str>),
    // should simlify to ident or the like ...
    FnParam(usize),

    // special forms
    Goto(RC<str>),
    If(Box<Ast3>, Box<Ast3>, Box<Ast3>),
    Define(RC<str>, Box<Ast3>),
    Lambda(usize, Option<Varidiac>, Box<Ast3>),
    Begin(Vec<Ast3>),
    Set(RC<str>, Box<Ast3>),
    Quote(Box<Ast3>),
}
fn quote(exp: Ast2) -> Ast3 {
    match exp {
        Ast2::Bool(t) => Ast3::Bool(t),
        Ast2::Number(t) => Ast3::Number(t),
        Ast2::String(t) => Ast3::String(t),
        Ast2::Ident(t) => Ast3::Ident(t),
        Ast2::Application(t) => Ast3::Application(t.into_iter().map(quote).collect()),
        Ast2::Label(t) => Ast3::Label(t),
        Ast2::FnParam(t) => Ast3::FnParam(t),
        _ => unreachable!(),
    }
}
type Error = String;
// replaces labels with gotos
pub fn pass2(expr: Ast2, links: &MultiMap<RC<str>, RC<str>>) -> Result<Ast3, Error> {
    let pass2 = |expr| pass2(expr, links);
    let pass2_box = |expr: Box<_>| pass2(*expr).map(Box::new);
    match expr {
        Ast2::Bool(t) => Ok(Ast3::Bool(t)),
        Ast2::Number(t) => Ok(Ast3::Number(t)),
        Ast2::String(t) => Ok(Ast3::String(t)),
        Ast2::Ident(t) => Ok(Ast3::Ident(t)),
        Ast2::FnParam(t) => Ok(Ast3::FnParam(t)),
        Ast2::Quote(q) => Ok(Ast3::Quote(map_box(q, quote))),
        Ast2::Application(exprs) => exprs
            .into_iter()
            .map(pass2)
            .try_collect()
            .map(Ast3::Application),
        Ast2::Label(l) => {
            if let Some(l) = links.get(&l) {
                Ok(Ast3::Goto(l.clone()))
            } else if links.has_key(&l) {
                Ok(Ast3::Label(l))
            } else {
                Err(format!("Label {l} invalid"))
            }
        }
        Ast2::If(cond, then, alt) => Ok(Ast3::If(
            pass2_box(cond)?,
            pass2_box(then)?,
            pass2_box(alt)?,
        )),
        Ast2::Define(i, expr) => Ok(Ast3::Define(i, pass2_box(expr)?)),
        Ast2::Lambda(pc, var, expr) => Ok(Ast3::Lambda(pc, var, pass2_box(expr)?)),
        Ast2::Begin(exprs) => exprs.into_iter().map(pass2).try_collect().map(Ast3::Begin),
        Ast2::Set(i, expr) => Ok(Ast3::Set(i, pass2_box(expr)?)),
    }
}
