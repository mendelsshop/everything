use std::fmt::Display;
use std::usize;
use std::{collections::HashMap, fmt};

use inkwell::values::StructValue;
use itertools::Itertools;

use crate::codegen::multimap::MultiMap;
use crate::{
    codegen::Compiler,
    interior_mut::{MUTEX, RC},
};

pub(crate) const ZERO_ARG: usize = 0;
pub(crate) const ONE_ARG: usize = 1;
pub(crate) const ZERO_VARIADIAC_ARG: usize = 2;
pub(crate) const ONE_VARIADIAC_ARG: usize = 3;
#[derive(Clone, Debug, PartialEq)]
pub struct Tree {
    pub inner: RC<MUTEX<(EverythingExpr, EverythingExpr, EverythingExpr)>>,
}

// TODO: flatten trait for quotation
pub trait FlattenAst<'a, 'ctx> {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx>;
}

#[derive(Clone, PartialEq, Debug)]
pub enum EverythingExpr {
    Bool(Boolean),
    Number(f64),
    String(RC<str>),
    Ident(RC<str>),
    Application(Vec<EverythingExpr>),
    Label(RC<str>),
    // should simlify to ident or the like ...
    FnParam(usize),
}
impl<'a, 'ctx> FlattenAst<'a, 'ctx> for EverythingExpr {
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

impl<'a, 'ctx> FlattenAst<'a, 'ctx> for Vec<EverythingExpr> {
    fn flatten(self, compiler: &mut Compiler<'a, 'ctx>) -> StructValue<'ctx> {
        fn list_to_tree<'ctx>(
            list: Vec<EverythingExpr>,
            compiler: &mut Compiler<'_, 'ctx>,
            n: usize,
        ) -> (StructValue<'ctx>, Vec<EverythingExpr>) {
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

impl fmt::Display for EverythingExpr {
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

impl<A: Into<RC<str>>> From<A> for EverythingExpr {
    fn from(value: A) -> Self {
        Self::Ident(value.into())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boolean {
    /// |
    False = 0,
    /// &
    True = 1,
    /// ?
    Maybee,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Maybee => write!(f, "maybe"),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Varidiac {
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 1 arg)
    AtLeast1,
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 0 args)
    AtLeast0,
}

#[derive(Debug, Copy, Clone)]
pub enum Arg {
    // All the variants besides Zero have a number, so even after auto currying the compiler still
    // knows the arguement index
    Zero,
    One(usize),
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 1 arg)
    AtLeast1(usize),
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 0 args)
    AtLeast0(usize),
}

impl From<Arg> for usize {
    fn from(value: Arg) -> Self {
        match value {
            Arg::Zero => ZERO_ARG,
            Arg::One(_) => ONE_ARG,
            Arg::AtLeast1(_) => ONE_VARIADIAC_ARG,
            Arg::AtLeast0(_) => ZERO_VARIADIAC_ARG,
        }
    }
}

impl fmt::Display for Varidiac {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AtLeast1 => "+",
                Self::AtLeast0 => "*",
            }
        )
    }
}
impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::AtLeast1(_) => "+",
                Self::AtLeast0(_) => "*",
                Self::Zero => "0",
                Self::One(_) => "1",
            }
        )
    }
}

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

type Error = String;

pub fn immutable_add_to_vec<T>(mut v: Vec<T>, x: T) -> Vec<T> {
    v.push(x);
    v
}

/// 2 transformations happen during this phase:
/// 1: all special forms are typified
/// 2: lambdas are sngle parmaterfied curring
pub fn pass1(value: (EverythingExpr, Vec<&str>)) -> Result<(Ast2, Vec<&str>), Error> {
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

    fn convert_begin(
        exps: Vec<EverythingExpr>,
        env: Vec<&str>,
    ) -> Result<(Ast2, Vec<&str>), Error> {
        exps.into_iter()
            .try_fold((vec![], env), |(exps, env), current| {
                pass1((current, env))
                    .map(|(current, env)| (immutable_add_to_vec(exps, current), env))
            })
            .map(|(app, env)| (Ast2::Begin(app), env))
    }

    fn convert_quoted(
        exps: Vec<EverythingExpr>,
        env: Vec<&str>,
    ) -> Result<(Ast2, Vec<&str>), Error> {
        if exps.len() != 1 {
            return Err("quoted expression can only contain single expression".to_string());
        }
        fn quote(exp: EverythingExpr) -> Ast2 {
            match exp {
                EverythingExpr::Bool(t) => Ast2::Bool(t),
                EverythingExpr::Number(t) => Ast2::Number(t),
                EverythingExpr::String(t) => Ast2::String(t),
                EverythingExpr::Ident(t) => Ast2::Ident(t),
                EverythingExpr::Application(t) => {
                    Ast2::Application(t.into_iter().map(quote).collect())
                }
                EverythingExpr::Label(t) => Ast2::Label(t),
                EverythingExpr::FnParam(t) => Ast2::FnParam(t),
            }
        }
        Ok((Ast2::Quote(Box::new(quote(exps[0].clone()))), env))
    }

    fn convert_set(exps: Vec<EverythingExpr>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
        // TODO: set should only be allowed to be able to set non special forms
        if exps.len() != 2 {
            return Err("the set! form must follow (set! [var] [value])".to_string());
        }

        let EverythingExpr::Ident(var) = exps[0].clone() else {
            return Err("the set! [var] must be a symbol".to_string());
        };
        pass1((exps[1].clone(), env)).map(|(exp, env)| {
            (
                Ast2::Set(var.clone(), Box::new(exp)),
                extend_if_found(var, env),
            )
        })
    }
    fn convert_define(
        exps: Vec<EverythingExpr>,
        env: Vec<&str>,
    ) -> Result<(Ast2, Vec<&str>), Error> {
        if exps.len() < 2 {
            return Err("the define form must follow (define [var] [value]) or (define ([var] [argc] <vararg>) exp+ )".to_string());
        }
        match exps[0].clone() {
            EverythingExpr::Application(a) => {
                if a.len() < 2 || a.len() > 3 {
                    return Err(
                        "the define form signature must follow ([var] [argc] <vararg>)".to_string(),
                    );
                }
                let EverythingExpr::Ident(i) = a[0].clone() else {
                    return Err("the define form [var] must be a symbol".to_string());
                };
                let env = extend_if_found(i.clone(), env);
                convert_lambda(
                    vec![EverythingExpr::Application(a[1..].to_vec())]
                        .into_iter()
                        .chain(exps[1..].to_vec())
                        .collect(),
                    env,
                )
                .map(|(exp, env)| (Ast2::Define(i, Box::new(exp)), env))
            }
            EverythingExpr::Ident(i) => {
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
    fn convert_lambda(
        exps: Vec<EverythingExpr>,
        env: Vec<&str>,
    ) -> Result<(Ast2, Vec<&str>), Error> {
        if exps.len() < 2 {
            return Err(
                "the lambda form must follow (lambda ([argc] <vararg>) exp+ ) ".to_string(),
            );
        }
        let (argc, vararg) = if let EverythingExpr::Application(app) = &exps[0] {
            match app.as_slice() {
                [EverythingExpr::Number(n), EverythingExpr::Ident(s)]
                    if ["+".into(), "*".into()].contains(s) =>
                {
                    (
                        *n,
                        if s.to_string().as_str() == "*" {
                            Some(Varidiac::AtLeast0)
                        } else {
                            Some(Varidiac::AtLeast1)
                        },
                    )
                }

                [EverythingExpr::Number(n)] => (*n, None),
                _ => todo!("self function should return result so self can error"),
            }
        } else {
            return Err("paramters in lambda does not take form ([argc] <varargs>) ".to_string());
        };
        let (body, _) = convert_begin(exps[1..].to_vec(), env.clone())?;
        Ok((Ast2::Lambda(argc as usize, vararg, Box::new(body)), env))
    }
    fn convert_if(exps: Vec<EverythingExpr>, env: Vec<&str>) -> Result<(Ast2, Vec<&str>), Error> {
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

    fn convert_application(
        app: Vec<EverythingExpr>,
        env: Vec<&str>,
    ) -> Result<(Ast2, Vec<&str>), Error> {
        match app.first() {
            Some(EverythingExpr::Ident(i))
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
        EverythingExpr::Bool(b) => Ok((Ast2::Bool(b), env)),
        EverythingExpr::Number(n) => Ok((Ast2::Number(n), env)),
        EverythingExpr::String(s) => Ok((Ast2::String(s), env)),
        EverythingExpr::Ident(i) => Ok((Ast2::Ident(i), env)),
        EverythingExpr::Application(app) => convert_application(app, env),
        EverythingExpr::Label(l) => Ok((Ast2::Label(l), env)),
        EverythingExpr::FnParam(p) => Ok((Ast2::FnParam(p), env)),
    }
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
            Ast3::Quote(q) => Self::Quote(map_box(q, quote)),
            Ast3::Goto(l) => Self::Goto(l),
        }
    }
}

fn map_box<T, U>(b: Box<T>, f: impl FnOnce(T) -> U) -> Box<U> {
    Box::new(f(*b))
}

fn map_into<T, U: From<T>>(b: Box<T>) -> Box<U> {
    map_box(b, Into::into)
}
