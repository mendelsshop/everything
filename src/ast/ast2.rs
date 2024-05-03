use std::fmt;

use itertools::Itertools;

use crate::interior_mut::RC;

use super::{ast1::Ast1, Boolean, Varidiac};

#[derive(Debug, Clone)]
pub enum Ast2 {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Ident(RC<str>),
    Application(Vec<Ast2>),
    Label(RC<str>),
    // should simlify to ident or the like ...
    FnParam(usize),

    // special forms
    If(Box<Ast2>, Box<Ast2>, Box<Ast2>),
    Define(RC<str>, Box<Ast2>),
    Lambda(usize, Option<Varidiac>, Box<Ast2>),
    Begin(Vec<Ast2>),
    Set(RC<str>, Box<Ast2>),
    Quote(Box<Ast2>),
}

impl fmt::Display for Ast2 {
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
                "(lambda ({argc}{}) {body})",
                vairdiac
                    .as_ref()
                    .map_or_else(String::new, |s| format!(" {s}"))
            ),
            Self::Begin(b) => write!(f, "(begin {})", b.iter().map(ToString::to_string).join(" ")),
            Self::Set(v, val) => write!(f, "(set! {v} {val})"),
            Self::Quote(q) => write!(f, "'{q}"),
        }
    }
}
pub fn immutable_add_to_vec<T>(mut v: Vec<T>, x: T) -> Vec<T> {
    v.push(x);
    v
}

type Error = String;
/// 2 transformations happen during this phase:
/// 1: all special forms are typified
/// 2: lambdas are sngle parmaterfied curring
pub fn pass1(value: (Ast1, Vec<&str>)) -> Result<(Ast2, Vec<&str>), Error> {
    const SPECIAL_FORMS: [&str; 9] = [
        "if", "define", "set!", "quote", "begin", "lambda", "cond", "let", "link",
    ];
    fn extend_if_found(name: impl fmt::Display, env: Vec<&str>) -> Vec<&str> {
        if let Some(i) = SPECIAL_FORMS.iter().position(|&x| x == name.to_string()) {
            immutable_add_to_vec(env, SPECIAL_FORMS[i])
        } else {
            env
        }
    }
    let env = value.1;

    fn convert_begin(exps: Vec<Ast1>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        exps.into_iter()
            .try_fold((vec![], env), |(exps, env), current| {
                pass1((current, env))
                    .map(|(current, env)| (immutable_add_to_vec(exps, current), env))
            })
            .map(|(app, env)| (Ast2::Begin(app), env))
    }

    fn convert_quoted(exps: Vec<Ast1>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        if exps.len() != 1 {
            return Err("quoted expression can only contain single expression".to_string());
        }
        fn quote(exp: Ast1) -> Ast2 {
            match exp {
                Ast1::Bool(t) => Ast2::Bool(t),
                Ast1::Number(t) => Ast2::Number(t),
                Ast1::String(t) => Ast2::String(t),
                Ast1::Ident(t) => Ast2::Ident(t),
                Ast1::Application(t) => Ast2::Application(t.into_iter().map(quote).collect()),
                Ast1::Label(t) => Ast2::Label(t),
                Ast1::FnParam(t) => Ast2::FnParam(t),
            }
        }
        Ok((Ast2::Quote(Box::new(quote(exps[0].clone()))), env))
    }

    fn convert_set(exps: Vec<Ast1>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        // TODO: set should only be allowed to be able to set non special forms
        if exps.len() != 2 {
            return Err("the set! form must follow (set! [var] [value])".to_string());
        }

        let Ast1::Ident(var) = exps[0].clone() else {
            return Err("the set! [var] must be a symbol".to_string());
        };
        pass1((exps[1].clone(), env)).map(|(exp, env)| {
            (
                Ast2::Set(var.clone(), Box::new(exp)),
                extend_if_found(var, env),
            )
        })
    }
    fn convert_define(exps: Vec<Ast1>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        if exps.len() < 2 {
            return Err("the define form must follow (define [var] [value]) or (define ([var] [argc] <vararg>) exp+ )".to_string());
        }
        match exps[0].clone() {
            Ast1::Application(a) => {
                if a.len() < 2 || a.len() > 3 {
                    return Err(
                        "the define form signature must follow ([var] [argc] <vararg>)".to_string(),
                    );
                }
                let Ast1::Ident(i) = a[0].clone() else {
                    return Err("the define form [var] must be a symbol".to_string());
                };
                let env = extend_if_found(i.clone(), env);
                convert_lambda(
                    vec![Ast1::Application(a[1..].to_vec())]
                        .into_iter()
                        .chain(exps[1..].to_vec())
                        .collect(),
                    env,
                )
                .map(|(exp, env)| (Ast2::Define(i, Box::new(exp)), env))
            }
            Ast1::Ident(i) => {
                if exps.len() != 2 {
                    return Err(
                        "the define form (define [var] [value]) must follow not have anything else"
                            .to_string(),
                    );
                }
                let env = extend_if_found(i.clone(), env);
                pass1((exps[1].clone(), env))
                    .map(|(exp, env)| (Ast2::Define(i, Box::new(exp)), env))
            }
            _ => Err(
                "the first part of a define must be [var] or ([var] [argc] <varags>)".to_string(),
            ),
        }
    }
    fn convert_lambda(exps: Vec<Ast1>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        if exps.len() < 2 {
            return Err(
                "the lambda form must follow (lambda ([argc] <vararg>) exp+ ) ".to_string(),
            );
        }
        let (argc, vararg) = if let Ast1::Application(app) = &exps[0] {
            match app.as_slice() {
                [Ast1::Number(n), Ast1::Ident(s)] if ["+".into(), "*".into()].contains(s) => (
                    *n,
                    if s.to_string().as_str() == "*" {
                        Some(Varidiac::AtLeast0)
                    } else {
                        Some(Varidiac::AtLeast1)
                    },
                ),

                [Ast1::Number(n)] => (*n, None),
                _ => todo!("self function should return result so self can error"),
            }
        } else {
            return Err("paramters in lambda does not take form ([argc] <varargs>) ".to_string());
        };
        let (body, _) = convert_begin(exps[1..].to_vec(), env.clone())?;
        Ok((Ast2::Lambda(argc as usize, vararg, Box::new(body)), env))
    }
    fn convert_if(exps: Vec<Ast1>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        if exps.len() != 3 {
            return Err(
                "the if form must follow (if [condition] [consequent] [alternative])".to_string(),
            );
        }
        pass1((exps[0].clone(), env)).and_then(|(cond, env)| {
            pass1((exps[1].clone(), env)).and_then(|(cons, env)| {
                pass1((exps[2].clone(), env)).map(|(alt, env)| {
                    (Ast2::If(Box::new(cond), Box::new(cons), Box::new(alt)), env)
                })
            })
        })
    }

    fn convert_application(app: Vec<Ast1>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        match app.first() {
            Some(Ast1::Ident(i))
                if !env.contains(&i.to_string().as_str())
                    && SPECIAL_FORMS.contains(&i.to_string().as_str()) =>
            {
                // TODO: have constraints on where some special forms can be used/rededinfed to help with the approximation of where some special forms are redefined when using lazyness
                let exps = app[1..].to_vec();
                match i.to_string().as_str() {
                    "lambda" => convert_lambda(exps, env),
                    "define" => convert_define(exps, env),
                    "set!" => convert_set(exps, env),
                    "begin" => convert_begin(exps, env),
                    "if" => convert_if(exps, env),
                    "quote" => convert_quoted(exps, env),
                    _ => unreachable!(),
                }
            }

            Some(fst) => {
                let fst = pass1((fst.clone(), env))?;
                let fst = (vec![fst.0], fst.1);
                app[1..]
                    .iter()
                    .cloned()
                    .try_fold(fst, |(app, env), current| {
                        pass1((current, env))
                            .map(|(current, env)| (immutable_add_to_vec(app, current), env))
                    })
                    .map(|(app, env)| (Ast2::Application(app), env))
            }
            None => Err("application must have at least one argument".to_string()),
        }
    }
    match value.0 {
        Ast1::Bool(b) => Ok((Ast2::Bool(b), env)),
        Ast1::Number(n) => Ok((Ast2::Number(n), env)),
        Ast1::String(s) => Ok((Ast2::String(s), env)),
        Ast1::Ident(i) => Ok((Ast2::Ident(i), env)),
        Ast1::Application(app) => convert_application(app, env),
        Ast1::Label(l) => Ok((Ast2::Label(l), env)),
        Ast1::FnParam(p) => Ok((Ast2::FnParam(p), env)),
    }
}
