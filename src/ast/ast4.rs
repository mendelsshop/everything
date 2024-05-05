use crate::{
    ast::{ast3::Ast3, map_box, map_into, Varidiac},
    interior_mut::RC,
};

use super::{Arg, AstTransformFrom, Boolean};

#[derive(Debug, Clone)]
pub enum Ast4 {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Ident(RC<str>),
    Application(Vec<Ast4>),
    Label(RC<str>),
    // should simlify to ident or the like ...
    FnParam(usize),

    // special forms
    Goto(RC<str>),
    If(Box<Ast4>, Box<Ast4>, Box<Ast4>),
    Define(RC<str>, Box<Ast4>),
    Lambda(Arg, Box<Ast4>),
    Begin(Vec<Ast4>),
    Set(RC<str>, Box<Ast4>),
    Quote(Box<Ast4>),
    Stop(Option<Box<Ast4>>),
}

impl AstTransformFrom<Ast3> for Ast4 {
    type Error = ();

    type State = ();

    fn transform(value: Ast3, state: Self::State) -> Result<(Self, Self::State), Self::Error> {
        Ok((Self::from(value), state))
    }
}

impl From<Ast3> for Ast4 {
    fn from(value: Ast3) -> Self {
        fn quote(exp: Ast3) -> Ast4 {
            match exp {
                Ast3::Bool(t) => Ast4::Bool(t),
                Ast3::Number(t) => Ast4::Number(t),
                Ast3::String(t) => Ast4::String(t),
                Ast3::Ident(t) => Ast4::Ident(t),
                Ast3::Application(t) => Ast4::Application(t.into_iter().map(quote).collect()),
                Ast3::Label(t) => Ast4::Label(t),
                Ast3::FnParam(t) => Ast4::FnParam(t),
                _ => unreachable!(),
            }
        }

        fn curryify(argc: usize, varidiac: Option<Varidiac>, body: Box<Ast3>) -> Ast4 {
            fn inner(
                argc: usize,
                varidiac: Option<Varidiac>,
                body: Box<Ast3>,
                arg_count: usize,
            ) -> Ast4 {
                if argc == 0 {
                    let body = map_into(body);
                    match varidiac {
                        Some(Varidiac::AtLeast0) => Ast4::Lambda(Arg::AtLeast0(arg_count), body),
                        Some(Varidiac::AtLeast1) => Ast4::Lambda(Arg::AtLeast1(arg_count), body),
                        None => *body,
                    }
                } else {
                    Ast4::Lambda(
                        Arg::One(arg_count),
                        Box::new(inner(argc - 1, varidiac, body, arg_count + 1)),
                    )
                }
            }
            inner(argc, varidiac, body, 0)
        }

        match value {
            Ast3::Bool(t) => Self::Bool(t),
            Ast3::Number(t) => Self::Number(t),
            Ast3::String(t) => Self::String(t),
            Ast3::Ident(t) => Self::Ident(t),
            Ast3::Application(t) => Self::Application(t.into_iter().map(Into::into).collect()),
            Ast3::Label(t) => Self::Label(t),
            Ast3::FnParam(t) => Self::FnParam(t),
            Ast3::If(cond, cons, alt) => Self::If(map_into(cond), map_into(cons), map_into(alt)),
            Ast3::Define(s, exp) => Self::Define(s, map_into(exp)),
            Ast3::Lambda(argc, varidiac, body) => {
                if argc == 0 && varidiac.is_none() {
                    Self::Lambda(Arg::Zero, map_into(body))
                } else {
                    curryify(argc, varidiac, body)
                }
            }
            Ast3::Begin(b) => Self::Begin(b.into_iter().map(Into::into).collect()),
            Ast3::Set(s, exp) => Self::Set(s, map_into(exp)),
            Ast3::Stop(s) => Self::Stop(s.map(map_into)),
            Ast3::Quote(q) => Self::Quote(map_box(q, quote)),
            Ast3::Goto(l) => Self::Goto(l),
        }
    }
}
