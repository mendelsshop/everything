use crate::interior_mut::RC;

use super::{Boolean, Varidiac};

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
    Loop(Box<Ast3>),
    Stop(Option<Box<Ast3>>),
}
// replaces labels with gotos
mod impl_transformer {
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
    use crate::{
        ast::{ast2::Ast2, map_box, AstTransformFrom, IteratorTransformer},
        codegen::multimap::MultiMap,
        interior_mut::RC,
    };

    use super::Ast3;

    type Error = String;
    impl AstTransformFrom<Ast2> for Ast3 {
        type Error = Error;

        type State = MultiMap<RC<str>, RC<str>>;

        fn transform(value: Ast2, state: Self::State) -> Result<(Self, Self::State), Self::Error> {
            let pass2_box = |expr: Box<_>, state: MultiMap<std::rc::Rc<str>, std::rc::Rc<str>>| {
                (|expr| Self::transform(expr, state))(*expr).map(|(e, s)| (Box::new(e), s))
            };
            match value {
                Ast2::Bool(t) => Ok((Ast3::Bool(t), state)),
                Ast2::Number(t) => Ok((Ast3::Number(t), state)),
                Ast2::String(t) => Ok((Ast3::String(t), state)),
                Ast2::Ident(t) => Ok((Ast3::Ident(t), state)),
                Ast2::FnParam(t) => Ok((Ast3::FnParam(t), state)),
                Ast2::Quote(q) => Ok((Ast3::Quote(map_box(q, quote)), state)),
                Ast2::Application(exprs) => exprs
                    .into_iter()
                    .transform::<Ast3>(state)
                    .transform_all()
                    .map(|(ast, state)| (Ast3::Application(ast), state)),
                Ast2::Label(l) => {
                    if let Some(l) = state.get(&l) {
                        Ok((Ast3::Goto(l.clone()), state))
                    } else if state.has_key(&l) {
                        Ok((Ast3::Label(l), state))
                    } else {
                        Err(format!("Label {l} invalid"))
                    }
                }
                Ast2::If(cond, then, alt) => {
                    let (cond, state) = pass2_box(cond, state)?;
                    let (then, state) = pass2_box(then, state)?;
                    let (alt, state) = pass2_box(alt, state)?;
                    Ok((Ast3::If(cond, then, alt), state))
                }
                Ast2::Define(i, expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Ast3::Define(i, expr), state))
                }
                Ast2::Loop(expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Ast3::Loop(expr), state))
                }
                Ast2::Lambda(pc, var, expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Ast3::Lambda(pc, var, expr), state))
                }
                Ast2::Begin(exprs) => exprs
                    .into_iter()
                    .transform::<Ast3>(state)
                    .transform_all()
                    .map(|(ast, state)| (Ast3::Begin(ast), state)),
                Ast2::Set(i, expr) => {
                    pass2_box(expr, state).map(|(expr, state)| (Ast3::Set(i, expr), state))
                }
                Ast2::Stop(s) => {
                    let (s, state) = match s {
                        Some(s) => pass2_box(s, state).map(|(s, state)| (Some(s), state)),
                        None => Ok((None, state)),
                    }?;
                    Ok((Self::Stop(s), state))
                }
            }
        }
    }
}
