pub mod scope;
pub mod syntax;
use scope::Scope;
use syntax::Syntax;

//pub mod ast1;
//pub mod ast2;
//pub mod ast3;
//pub mod ast4;

use std::{
    collections::BTreeSet,
    fmt::{self, Debug},
    iter,
    rc::Rc,
};

use crate::evaluator::{Env, EnvRef, Evaluator};

#[macro_export]
macro_rules! list {
    () => {$crate::ast::Ast::TheEmptyList};
    ($car:expr $(,)?) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, $crate::ast::Ast::TheEmptyList)))
    };
    ($car:expr ; $cdr:expr) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, $cdr)))
    };
    ($car:expr, $($cdr:tt)+) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, list!($($cdr)+))))
    };
}

pub type AnalyzedResult = Result<Box<dyn AnalyzeFn>, String>;

pub trait AnalyzeFn: Fn(EnvRef) -> Result<Ast, String> {
    fn clone_box<'a>(&self) -> Box<dyn 'a + AnalyzeFn>
    where
        Self: 'a;
}

impl<F> AnalyzeFn for F
where
    F: Fn(EnvRef) -> Result<Ast, String> + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn 'a + AnalyzeFn>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a> Clone for Box<dyn 'a + AnalyzeFn> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Function {
    Lambda(Lambda),
    Primitive(Primitive),
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lambda(l) => write!(f, "(lambda {} {})", l.2, l.0),
            Self::Primitive(_) => write!(f, "(primitive-procedure)"),
        }
    }
}

impl Function {
    pub(crate) fn apply(&self, args: Ast) -> Result<Ast, String> {
        match self {
            Self::Lambda(Lambda(body, env, params)) => {
                let env = Env::extend_envoirnment(env.clone(), *params.clone(), args)?;
                Evaluator::eval_sequence(*body.clone(), env)
            }
            Self::Primitive(p) => p(args),
        }
    }
}

#[derive(Clone)]
pub struct Lambda(pub Box<Ast>, pub EnvRef, pub Box<Ast>);

impl PartialEq for Lambda {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(lambda {} {}", self.2, self.0)
    }
}

pub type Primitive = fn(Ast) -> Result<Ast, String>;

#[derive(Clone, PartialEq, Debug)]
pub struct Pair(pub Ast, pub Ast);

impl Pair {
    pub fn map(&self, mut f: impl FnMut(Ast) -> Result<Ast, String>) -> Result<Ast, String> {
        let car = f(self.0.clone())?;
        let cdr = self.1.map(f)?;
        Ok(Ast::Pair(Box::new(Self(car, cdr))))
    }
    #[must_use]
    pub fn list(&self) -> bool {
        self.1.list()
    }
    #[must_use]
    pub fn size(&self) -> usize {
        1 + self.1.size()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Ast {
    Pair(Box<Pair>),
    TheEmptyList,
    String(RC<str>),
    Syntax(Box<Syntax<Ast>>),
    Number(f64),
    Boolean(Boolean),
    Symbol(Symbol),
    Function(Function),
    Label(RC<str>),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub Rc<str>, pub usize);
impl From<&str> for Ast {
    fn from(value: &str) -> Self {
        Self::Symbol(value.into())
    }
}
impl From<String> for Ast {
    fn from(value: String) -> Self {
        Self::Symbol(value.into())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0, self.1)
    }
}

impl From<Rc<str>> for Symbol {
    fn from(value: Rc<str>) -> Self {
        Self(value, 0)
    }
}

impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self(value.into(), 0)
    }
}

impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(value.into(), 0)
    }
}
#[must_use] pub fn bound_identifier(a: Ast, b: Ast) -> bool {
    matches!((a, b), (Ast::Syntax(a), Ast::Syntax(b)) if a == b)
}

impl fmt::Display for Ast {
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
            Self::Syntax(s) => write!(f, "#'{}", s.0),
            Self::Number(n) => write!(f, "{n}"),
            Self::Symbol(s) => write!(f, "'{s}"),
            Self::Function(function) => write!(f, "{function}"),
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Label(f0) => write!(f, "@{f0}"),
            Self::TheEmptyList => write!(f, "()"),
        }
    }
}

impl Ast {
    #[must_use]
    pub fn size(&self) -> usize {
        match self {
            Self::Pair(p) => p.size(),
            _ => 0,
        }
    }
    pub fn map(&self, f: impl FnMut(Self) -> Result<Self, String>) -> Result<Self, String> {
        match self {
            Self::Pair(p) => {
                let this = &p;
                let mut f = f;
                let car = f(this.0.clone())?;
                let cdr = this.1.map(f)?;
                Ok(Self::Pair(Box::new(Pair(car, cdr))))
            }
            Self::TheEmptyList => Ok(Self::TheEmptyList),
            bad => Err(format!("cannot map {bad}")),
        }
    }
    pub fn map_pair<E>(self, mut f: impl FnMut(Self, bool) -> Result<Self, E>) -> Result<Self, E> {
        {
            match self {
                Self::Pair(p) => {
                    let Pair(car, cdr) = *p;
                    let car = f(car, false)?;
                    let cdr = cdr.map_pair(f)?;
                    Ok(Self::Pair(Box::new(Pair(car, cdr))))
                }
                other_term => f(other_term, true),
            }
        }
    }

    pub fn foldl_pair<A>(self, mut f: impl FnMut(Self, bool, A) -> A, init: A) -> A {
        match self {
            Self::Pair(p) => {
                let Pair(car, cdr) = *p;
                let car = f(car, false, init);

                cdr.foldl_pair(f, car)
            }
            other_term => f(other_term, true, init),
        }
    }

    pub fn foldl<A>(self, mut f: impl FnMut(Self, A) -> A, init: A) -> Result<A, String> {
        self.foldl_pair(
            |term, base, init: Result<A, String>| {
                if !base {
                    init.map(|init| f(term, init))
                } else {
                    match term {
                        Self::TheEmptyList => init,
                        _other => Err(String::new()),
                    }
                }
            },
            Ok(init),
        )
    }

    #[must_use] pub const fn is_keyword(&self) -> bool {
        // https://docs.racket-lang.org/guide/keywords.html
        false
    }

    // pub fn to_synax_list(self) -> Self {
    //     match self {
    //         Self::List(l) => Self::List(l.into_iter().map(Self::to_synax_list).collect()),
    //         Self::Syntax(s) => s.0.to_synax_list(),
    //         _ => self,
    //     }
    // }
    #[must_use]
    pub fn list(&self) -> bool {
        matches!(self,  Self::Pair(p) if p.list() ) || *self == Self::TheEmptyList
    }

    pub(crate) fn scope_set(&self) -> Option<BTreeSet<Scope>> {
        match self {
            Self::Syntax(s) => Some(s.1.clone()),
            _ => None,
        }
    }
}
use std::fmt::Display;
use std::usize;

use crate::interior_mut::RC;

pub trait AstTransformFrom<T>: Sized {
    type Error;
    type State;

    fn transform(value: T, state: Self::State) -> Result<(Self, Self::State), Self::Error>;
}

#[allow(missing_debug_implementations)]
pub struct Transformer<I, T, U>
where
    I: Iterator<Item = T>,
    U: AstTransformFrom<T>,
{
    iter: I,
    // Has to be option result so that we can take ownership of it mem::take
    state: Option<Result<U::State, U::Error>>,
}

impl<I, T, U> Iterator for Transformer<I, T, U>
where
    I: Iterator<Item = T>,
    U: AstTransformFrom<T>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().and_then(|ast| {
            self.state
                .take()?
                .ok()
                .and_then(|state| match U::transform(ast, state) {
                    Ok((ast, state)) => {
                        self.state = Some(Ok(state));
                        Some(ast)
                    }
                    Err(e) => {
                        self.state = Some(Err(e));
                        None
                    }
                })
        })
    }
}

impl<I, T, U> Transformer<I, T, U>
where
    I: Iterator<Item = T>,
    U: AstTransformFrom<T>,
{
    pub const fn new(iter: I, state: U::State) -> Self {
        Self {
            iter,
            state: Some(Ok(state)),
        }
    }

    pub fn transform_all<B: FromIterator<U>>(mut self) -> Result<(B, U::State), U::Error> {
        let values: B = iter::from_fn(|| self.next()).collect();
        self.state.unwrap().map(|state| (values, state))
    }
}
pub trait IteratorTransformer<T>: Iterator<Item = T> {
    fn transform<U: AstTransformFrom<T>>(self, state: U::State) -> Transformer<Self, T, U>
    where
        Self: Sized,
    {
        Transformer::new(self, state)
    }
}

impl<I, T> IteratorTransformer<T> for I where I: Iterator<Item = T> {}

pub(crate) const ZERO_ARG: usize = 0;
pub(crate) const ONE_ARG: usize = 1;
pub(crate) const ZERO_VARIADIAC_ARG: usize = 2;
pub(crate) const ONE_VARIADIAC_ARG: usize = 3;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ModuleType {
    Inline(usize),
    Path(RC<str>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boolean {
    /// |
    False = 0,
    /// &
    True = 1,
    /// ?
    Maybe,
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::False => write!(f, "false"),
            Self::True => write!(f, "true"),
            Self::Maybe => write!(f, "maybe"),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

fn map_box<T, U>(b: Box<T>, f: impl FnOnce(T) -> U) -> Box<U> {
    Box::new(f(*b))
}

fn map_into<T, U: From<T>>(b: Box<T>) -> Box<U> {
    map_box(b, Into::into)
}
