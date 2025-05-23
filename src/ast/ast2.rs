use std::{fmt, rc::Rc};

use itertools::Itertools;

use super::{ast1::Label, Ast, ModuleType, Param};

#[derive(Debug, Clone)]
pub enum Ast2 {
    Basic(Ast),

    // special forms
    If(Box<Ast2>, Box<Ast2>, Box<Ast2>),
    DefineValues(Vec<Rc<str>>, Box<Ast2>),
    LetValues(Vec<(Vec<Rc<str>>, Ast2)>, Box<Ast2>),
    LetRecValues(Vec<(Vec<Rc<str>>, Ast2)>, Box<Ast2>),
    Lambda(Param, Box<Ast2>),
    Application(Box<Ast2>, Vec<Ast2>),
    Expression(Box<Ast2>),
    Begin(Vec<Ast2>),
    Begin0(Vec<Ast2>),
    Set(Rc<str>, Box<Ast2>),
    Quote(Ast),
    Stop(Option<Box<Ast2>>),
    Skip,
    Loop(Box<Ast2>),
    Module(Rc<str>, ModuleType),
    Goto(Label),
}
impl fmt::Display for Ast2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Goto(g) => write!(f, "(goto {g})"),
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
            Self::Set(v, val) => write!(f, "(set-bang {v} {val})"),
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
// replaces labels with gotos
mod impl_transformer {
    use crate::{
        ast::{
            ast1::{Ast1, Label},
            Ast, AstTransformFrom, Function, IteratorTransformer, Pair,
        },
        multimap::MultiMap,
    };

    use super::Ast2;

    type Error = String;

    impl AstTransformFrom<Ast1> for Ast2 {
        type Error = Error;

        type State = MultiMap<Label, Label>;

        fn transform(value: Ast1, state: Self::State) -> Result<(Self, Self::State), Self::Error> {
            let pass2_box = |expr: Box<_>, state: MultiMap<Label, Label>| {
                (|expr| Self::transform(expr, state))(*expr).map(|(e, s)| (Box::new(e), s))
            };
            match value {
                Ast1::Module(name, kind) => Ok((Self::Module(name, kind), state)),
                Ast1::Quote(q) => Ok((Self::Quote(q), state)),
                Ast1::Application(f, exprs) => {
                    let (f, state) = pass2_box(f, state)?;
                    exprs
                        .into_iter()
                        .transform::<Self>(state)
                        .transform_all()
                        .map(|(ast, state)| (Self::Application(f, ast), state))
                }
                Ast1::Basic(Ast::Label(l)) => {
                    if let Some(l) = state.get(&Label(l.clone())) {
                        Ok((Self::Goto(l.clone()), state))
                    } else if state.has_key(&Label(l.clone())) {
                        Ok((Self::Basic(Ast::Label(l)), state))
                    } else {
                        Err(format!("Label {l} invalid"))
                    }
                }
                Ast1::Basic(b) => convert_basic(b).map(|b| (Self::Basic(b), state)),
                Ast1::If(cond, then, alt) => {
                    let (cond, state) = pass2_box(cond, state)?;
                    let (then, state) = pass2_box(then, state)?;
                    let (alt, state) = pass2_box(alt, state)?;
                    Ok((Self::If(cond, then, alt), state))
                }
                Ast1::DefineValues(i, expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Self::DefineValues(i, expr), state))
                }
                Ast1::Loop(expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Self::Loop(expr), state))
                }
                Ast1::Lambda(pc, expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Self::Lambda(pc, expr), state))
                }
                Ast1::Begin(exprs) => exprs
                    .into_iter()
                    .transform::<Self>(state)
                    .transform_all()
                    .map(|(ast, state)| (Self::Begin(ast), state)),
                Ast1::Begin0(exprs) => exprs
                    .into_iter()
                    .transform::<Self>(state)
                    .transform_all()
                    .map(|(ast, state)| (Self::Begin0(ast), state)),
                Ast1::Set(i, expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Self::Set(i, expr), state))
                }
                Ast1::Stop(s) => {
                    let (s, state) = match s {
                        Some(s) => pass2_box(s, state).map(|(s, state)| (Some(s), state)),
                        None => Ok((None, state)),
                    }?;
                    Ok((Self::Stop(s), state))
                }
                Ast1::LetValues(items, ast1) => {
                    let (body, state) = pass2_box(ast1, state)?;
                    let (items, state) = items
                        .into_iter()
                        .transform::<(Vec<_>, Self)>(state)
                        .transform_all()?;
                    Ok((Self::LetValues(items, body), state))
                }
                Ast1::LetRecValues(items, ast1) => {
                    let (body, state) = pass2_box(ast1, state)?;
                    let (items, state) = items
                        .into_iter()
                        .transform::<(Vec<_>, Self)>(state)
                        .transform_all()?;
                    Ok((Self::LetRecValues(items, body), state))
                }
                Ast1::Expression(ast1) => {
                    pass2_box(ast1, state).map(|(ast1, s)| (Self::Expression(ast1), s))
                }
                Ast1::Skip => Ok((Self::Skip, state)),
            }
        }
    }

    fn convert_basic(b: Ast) -> Result<Ast, String> {
        match b {
            Ast::Pair(p) => {
                let a = convert_basic(p.0)?;
                let b = convert_basic(p.1)?;
                Ok(Ast::Pair(Box::new(Pair(a, b))))
            }
            Ast::Symbol(_)
            | Ast::Label(_)
            | Ast::Boolean(_)
            | Ast::TheEmptyList
            | Ast::String(_)
            | Ast::Number(_) => Ok(b),
            Ast::Syntax(mut syntax) => {
                let a = convert_basic(syntax.0.take())?;
                Ok(Ast::Syntax(Box::new(syntax.with(a))))
            }
            Ast::Function(Function::Primitive(p)) => Ok(p.name.into()),
            Ast::Function(Function::Lambda(_)) => unreachable!(),
        }
    }
}
