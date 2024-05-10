use std::fmt;

use itertools::Itertools;

use crate::interior_mut::RC;

use super::{Boolean, ModuleType, Varidiac};

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
    Stop(Option<Box<Ast2>>),
    Loop(Box<Ast2>),
    Module(RC<str>, ModuleType),
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
            Self::Quote(q) => write!(f, ";{q}"),
            Self::Loop(l) => write!(f, "(loop {l}"),
            Self::Stop(s) => write!(
                f,
                "(stop{})",
                s.as_ref()
                    .map(|s| format!(" {s}"))
                    .unwrap_or_else(|| String::new())
            ),
            Self::Module(name,_type) => write!(f,"(module {})", name),
        }
    }
}
pub fn immutable_add_to_vec<T>(mut v: Vec<T>, x: T) -> Vec<T> {
    v.push(x);
    v
}

mod impl_transformer {
    use std::fmt;

    use crate::ast::{
        ast1::Ast1,
        ast2::{immutable_add_to_vec, Ast2},
        AstTransformFrom, Varidiac,
    };

    const SPECIAL_FORMS: [&str; 12] = [
        "if", "define", "set!", "quote", "begin", "lambda", "cond", "let", "link", "stop", "loop",
        "module",
    ];
    type Error = String;

    type State = Vec<&'static str>;
    impl AstTransformFrom<Ast1> for Ast2 {
        type Error = Error;

        type State = State;

        fn transform(value: Ast1, state: Self::State) -> Result<(Self, Self::State), Self::Error> {
            match value {
                Ast1::Bool(b) => Ok((Self::Bool(b), state)),
                Ast1::Number(n) => Ok((Self::Number(n), state)),
                Ast1::String(s) => Ok((Self::String(s), state)),
                Ast1::Ident(i) => Ok((Self::Ident(i), state)),
                Ast1::Application(app) => convert_application(app, state),
                Ast1::Label(l) => Ok((Self::Label(l), state)),
                Ast1::FnParam(p) => Ok((Self::FnParam(p), state)),
            }
        }
    }
    /// 2 transformations happen during this phase:
    /// 1: all special forms are typified
    /// 2: lambdas are sngle parmaterfied curring
    fn extend_if_found(name: impl fmt::Display, env: State) -> State {
        if let Some(i) = SPECIAL_FORMS.iter().position(|&x| x == name.to_string()) {
            immutable_add_to_vec(env, SPECIAL_FORMS[i])
        } else {
            env
        }
    }

    fn convert_begin(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
        exps.into_iter()
            .try_fold((vec![], env), |(exps, env), current| {
                Ast2::transform(current, env)
                    .map(|(current, env)| (immutable_add_to_vec(exps, current), env))
            })
            .map(|(app, env)| (Ast2::Begin(app), env))
    }

    fn convert_quoted(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
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

    fn convert_set(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
        // TODO: set should only be allowed to be able to set non special forms
        if exps.len() != 2 {
            return Err("the set! form must follow (set! [var] [value])".to_string());
        }

        let Ast1::Ident(var) = exps[0].clone() else {
            return Err("the set! [var] must be a symbol".to_string());
        };
        Ast2::transform(exps[1].clone(), env).map(|(exp, env)| {
            (
                Ast2::Set(var.clone(), Box::new(exp)),
                extend_if_found(var, env),
            )
        })
    }
    fn convert_define(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
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
                Ast2::transform(exps[1].clone(), env)
                    .map(|(exp, env)| (Ast2::Define(i, Box::new(exp)), env))
            }
            _ => Err(
                "the first part of a define must be [var] or ([var] [argc] <varags>)".to_string(),
            ),
        }
    }
    fn convert_lambda(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
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
    fn convert_if(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
        if exps.len() != 3 {
            return Err(
                "the if form must follow (if [condition] [consequent] [alternative])".to_string(),
            );
        }
        Ast2::transform(exps[0].clone(), env).and_then(|(cond, env)| {
            Ast2::transform(exps[1].clone(), env).and_then(|(cons, env)| {
                Ast2::transform(exps[2].clone(), env).map(|(alt, env)| {
                    (Ast2::If(Box::new(cond), Box::new(cons), Box::new(alt)), env)
                })
            })
        })
    }

    fn convert_application(app: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
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
                    "stop" => convert_return(exps, env),
                    "loop" => convert_loop(exps, env),
                    "module" => convert_module(exps, env),
                    _ => unreachable!(),
                }
            }

            Some(fst) => {
                let fst = Ast2::transform(fst.clone(), env)?;
                let fst = (vec![fst.0], fst.1);
                app[1..]
                    .iter()
                    .cloned()
                    .try_fold(fst, |(app, env), current| {
                        Ast2::transform(current, env)
                            .map(|(current, env)| (immutable_add_to_vec(app, current), env))
                    })
                    .map(|(app, env)| (Ast2::Application(app), env))
            }
            None => Err("application must have at least one argument".to_string()),
        }
    }

    fn convert_module(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
        // TODO: requires modification of state type to include a hashmap of moduletype, vec ast,
        // and usize, so no inline modules hash to same thing
        // then if its an inline module have moduletype be inline wityh usize, increment usize and
        // parse inline module put in hashmap and put module and name as module to return
        // if its file read and parse file put in hasmap as module type file with file path
        // then return module type and name as module in ast
        todo!()
    }

    fn convert_loop(exps: Vec<Ast1>, env: State) -> Result<(Ast2, State), Error> {
        match &exps[..] {
            [body] => Ast2::transform(body.clone(), env)
                .map(|(ast, env)| (Ast2::Loop(Box::new(ast)), env)),
            _ => Err(format!(
                "loop with to many statements {}",
                exps.into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )),
        }
    }

    fn convert_return(exps: Vec<Ast1>, state: State) -> Result<(Ast2, State), Error> {
        if exps.len() > 1 {
            Err(format!(
                "Stop's can only hace at most one expression found {} expressions: {}",
                exps.len(),
                exps.into_iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ))
        } else {
            let (s, state) = match exps.first() {
                Some(s) => {
                    Ast2::transform(s.clone(), state).map(|(s, state)| (Some(Box::new(s)), state))
                }
                None => Ok((None, state)),
            }?;
            Ok((Ast2::Stop(s), state))
        }
    }
}
