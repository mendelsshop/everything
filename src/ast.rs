pub mod scope;
pub mod syntax;
use scope::Scope;
use syntax::{Properties, SourceLocation, Syntax};

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

use crate::evaluator::{Env, EnvRef, Evaluator, Values};
#[macro_export]
macro_rules! matches_to {
    ($e:expr => $s:path) => {
        match $e {
            $s(e) => Some(e),
            _ => None,
        }
    };
    ($e:expr => $s:path | this) => {
        match $e {
            $s(v) => Ok(v),
            e => Err(e),
        }
    };
    ($e:expr => $s:path | $r:expr) => {
        match $e {
            $s(e) => Ok(e),
            _ => Err($r),
        }
    };
}
#[macro_export]
macro_rules! list {
    () => {$crate::ast::Ast::TheEmptyList};
    ($car:expr $(,)?) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, $crate::ast::Ast::TheEmptyList)))
    };



    ($car:expr; $cdr:expr) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, $cdr)))
    };

    ($car:expr, $($cdr:tt)+) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, list!($($cdr)+))))
    };

    (quote $($datum:tt)*) =>  {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair("quote".into(), list!($($datum)+))))
    };
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Function {
    Lambda(Lambda),
    Primitive(Primitive),
}
#[macro_export]
/// s-expression like quasi-quoter
/// \# for unquote
/// ; for dot
/// do mbe limitations each item in list must be delimeted by , unless its followed by ; or the
/// last thing in a list
macro_rules! sexpr {
    (@list) => {$crate::ast::Ast::TheEmptyList};
    (()) => {$crate::ast::Ast::TheEmptyList};
    ([]) => {$crate::ast::Ast::TheEmptyList};
    ($i:ident) => {
        $crate::ast::Ast::from(stringify!($i))
    };
    ($l:literal) => {
        $crate::ast::Ast::from($l)
    };
    (#($e:expr)) => {
        $e
    };

    (@list . $($cdr:tt)+) => {
       sexpr!($($cdr)+)
    };
    (@list $car:ident $($cdr:tt)*) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(stringify!($car).into(), sexpr!(@list $($cdr)*))))
    };
    (@list ($($car:tt)*) $($cdr:tt)*) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(sexpr!(($($car)*)), sexpr!(@list $($cdr)*))))
    };
    (@list[ $($car:tt)* ] $($cdr:tt)*) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(sexpr!([ $($car)* ]), sexpr!(@list $($cdr)*))))
    };
    (@list $car:literal $($cdr:tt)*) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car.into(), sexpr!(@list $($cdr)*))))
    };
    (@list #($car:expr) $($cdr:tt)*) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list $($cdr)*))))
    };

    (($car:ident $($cdr:tt)*)) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(stringify!($car).into(), sexpr!(@list $($cdr)*))))
    };
    (($car:literal $($cdr:tt)*)) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car.into(), sexpr!(@list $($cdr)*))))
    };
    ((($($car:tt)*) $($cdr:tt)*)) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(sexpr!(($($car)*)), sexpr!(@list $($cdr)*))))
    };
    (([ $($car:tt)* ] $($cdr:tt)*)) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(sexpr!([ $($car)* ]), sexpr!(@list $($cdr)*))))
    };
    ((#($car:expr) $($cdr:tt)*)) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list $($cdr)*))))
    };

    ([ $car:ident $($cdr:tt)* ]) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(stringify!($car).into(), sexpr!(@list $($cdr)*))))
    };
    ([ $car:literal $($cdr:tt)* ]) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car.into(), sexpr!(@list $($cdr)*))))
    };
    ([ ($($car:tt)+) $($cdr:tt)* ]) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(sexpr!(($($car)+)), sexpr!(@list $($cdr)*))))
    };
    ([ [ $($car:tt)+ ] $($cdr:tt)* ]) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair(sexpr!([ $($car)+ ]), sexpr!(@list $($cdr)*))))
    };
    ([ #($car:expr) $($cdr:tt)* ]) => {
        $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list $($cdr)*))))
    };

    // ((#$car:expr, $($cdr:tt)*)) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list ,$($cdr)*))))
    // };
    // ((#$car:expr; $($cdr:tt)*)) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list ;$($cdr)*))))
    // };
    // ((#$car:expr)) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, $crate::ast::Ast::TheEmptyList)))
    // };
    //
    // ([ #$car:expr, $($cdr:tt)* ]) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list ,$($cdr)*))))
    // };
    // ([ #$car:expr; $($cdr:tt)* ]) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list ;$($cdr)*))))
    // };
    // ([ #$car:expr ]) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, $crate::ast::Ast::TheEmptyList)))
    // };
    //
    // (@list #($car:expr) $($cdr:tt)*) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list ,$($cdr)*))))
    // };
    // (@list; #($car:expr) $($cdr:tt)*) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, sexpr!(@list ;$($cdr)*))))
    // };
    // (@list, #($car:expr)) => {
    //     $crate::ast::Ast::Pair(Box::new($crate::ast::Pair($car, $crate::ast::Ast::TheEmptyList)))
    // };

}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lambda(l) => write!(f, "(lambda {} {})", l.param, l.body),
            Self::Primitive(_) => write!(f, "(primitive-procedure)"),
        }
    }
}

// TODO: how does function application work if we are currying all the functions, espically for
// apply transformer
// from looking at compilation code:
// primitives are fully applied
// lambdas are curried
// variadiacs are fully applied
impl Function {
    pub(crate) fn apply(&self, args: Ast) -> Result<Values, String> {
        match self {
            Self::Lambda(Lambda { body, env, param }) => match param {
                Param::Zero => {
                    if args == Ast::TheEmptyList {
                        Evaluator::eval_sequence(*body.clone(), env.clone())
                    } else {
                        Err("empty lambda must be applied to no arguements".to_string())
                    }
                }
                Param::One(n) => {
                    let Pair(arg, args) =
                        *matches_to!(args => Ast::Pair).ok_or("expected at least one arguement")?;
                    let curried = Evaluator::eval_sequence(
                        *body.clone(),
                        Env::new_lambda_env(env.clone(), Symbol(n.clone()), arg),
                    )?;
                    if args == Ast::TheEmptyList {
                        Ok(curried)
                    } else {
                        let curried = curried
                            .into_single()
                            .map_err(|_| "arity error expected one curried value".to_string())?;
                        let curried = matches_to!(curried => Ast::Function)
                            .ok_or("expected function to be curried")?;
                        curried.apply(args)
                    }
                }
                Param::AtLeast1(n) => {
                    if args == Ast::TheEmptyList {
                        Err("+ requires at least one argument".to_string())
                    } else {
                        Evaluator::eval_sequence(
                            *body.clone(),
                            Env::new_lambda_env(env.clone(), Symbol(n.clone()), args),
                        )
                    }
                }
                Param::AtLeast0(n) => Evaluator::eval_sequence(
                    *body.clone(),
                    Env::new_lambda_env(env.clone(), Symbol(n.clone()), args),
                ),
            },
            Self::Primitive(p) => p(args),
        }
    }

    pub fn apply_single(&self, args: Ast) -> Result<Ast, String> {
        self.apply(args).and_then(|values| {
            values
                .into_single()
                .map_err(|_| "arity error expected one value".to_string())
        })
    }
}

#[derive(Clone)]
pub struct Lambda {
    pub body: Box<Ast>,
    pub env: EnvRef,
    pub param: Param,
}

impl PartialEq for Lambda {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl Eq for Lambda {}

impl fmt::Debug for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(lambda {} {}", self.param, self.body)
    }
}

pub type Primitive = fn(Ast) -> Result<Values, String>;

#[derive(Clone, PartialEq, Debug)]
pub struct Pair(pub Ast, pub Ast);

impl Pair {
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
pub struct Symbol(pub Rc<str>);
impl Symbol {
    pub(crate) fn datum_to_syntax(
        self,
        scopes: Option<BTreeSet<Scope>>,
        srcloc: Option<SourceLocation>,
        properties: Option<Properties>,
    ) -> Syntax<Self> {
        Syntax(
            self,
            scopes.unwrap_or_default(),
            srcloc.unwrap_or_default(),
            properties.unwrap_or_default(),
        )
    }
}
impl TryFrom<Ast> for Symbol {
    type Error = String;

    fn try_from(value: Ast) -> Result<Self, Self::Error> {
        let Ast::Symbol(s) = value else {
            return Err("not a symbol".to_string());
        };
        Ok(s)
    }
}
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
impl From<usize> for Ast {
    fn from(value: usize) -> Self {
        Self::Number(value as f64)
    }
}
impl From<f64> for Ast {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Rc<str>> for Symbol {
    fn from(value: Rc<str>) -> Self {
        Self(value)
    }
}
impl From<String> for Symbol {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}
impl From<&str> for Symbol {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

#[must_use]
pub fn bound_identifier(a: Ast, b: Ast) -> bool {
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
            Self::Syntax(s) => write!(f, "#'{}", s.0.clone().syntax_to_datum()),
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
    pub fn map2(
        a: Self,
        b: Self,
        mut f: impl FnMut(Self, Self) -> Result<Self, String>,
    ) -> Result<Self, String> {
        match (a, b) {
            (Self::Pair(p), Self::Pair(p1)) => {
                let car = f(p.0.clone(), p1.0.clone())?;
                let cdr = Self::map2(p.1, p1.1, f)?;
                Ok(Self::Pair(Box::new(Pair(car, cdr))))
            }
            (Self::TheEmptyList, Self::TheEmptyList) => Ok(Self::TheEmptyList),
            bad => Err(format!("cannot map {} and {}", bad.0, bad.1)),
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
                if base {
                    match term {
                        Self::TheEmptyList => init,
                        _other => Err(String::new()),
                    }
                } else {
                    init.map(|init| f(term, init))
                }
            },
            Ok(init),
        )
    }
    // TODO: have Vec<Ast> -> Ast
    #[must_use]
    pub fn to_list(self) -> Vec<Self> {
        self.foldl_pair(
            |term, base, mut init| {
                if base && term == Self::TheEmptyList {
                } else {
                    init.push(term);
                }
                init
            },
            Vec::new(),
        )
    }
    pub fn map_to_list_checked<T>(
        self,
        mut f: impl FnMut(Self) -> Result<T, String>,
    ) -> Result<Vec<T>, Option<String>> {
        {
            let init = Vec::new();
            self.foldl_pair(
                |term, base, init: Result<Vec<T>, Option<String>>| {
                    if base {
                        match term {
                            Self::TheEmptyList => init,
                            _other => Err(None),
                        }
                    } else {
                        init.and_then(|mut init| {
                            f(term)
                                .map(|x| {
                                    init.push(x);
                                    init
                                })
                                .map_err(Some)
                        })
                    }
                },
                Ok(init),
            )
        }
    }
    pub fn to_list_checked(self) -> Result<Vec<Self>, String> {
        self.foldl(
            |term, mut init| {
                init.push(term);
                init
            },
            Vec::new(),
        )
    }

    #[must_use]
    pub const fn is_keyword(&self) -> bool {
        // TODO: https://docs.racket-lang.org/guide/keywords.html
        false
    }

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
    pub(crate) fn properties(&self) -> Option<Properties> {
        match self {
            Self::Syntax(s) => Some(s.3.clone()),
            _ => None,
        }
    }
    pub(crate) fn syntax_src_loc(&self) -> Option<syntax::SourceLocation> {
        match self {
            Self::Syntax(s) => Some(s.2.clone()),
            _ => None,
        }
    }
    #[must_use]
    pub fn append(self, list: Self) -> Self {
        fn inner(list1: Ast, list2: Ast, f: impl FnOnce(Ast) -> Ast) -> Ast {
            match list1 {
                Ast::Pair(pair) => {
                    let Pair(x, xs) = *pair;
                    inner(xs, list2, |acc| f(list!(x;acc)))
                }
                Ast::TheEmptyList => f(list2),
                _ => list1,
            }
        }
        inner(self, list, |x| x)
    }

    #[must_use]
    pub fn to_synax_list(self) -> Self {
        match self {
            Self::Pair(l) => Self::Pair(Box::new(Pair(l.0, l.1.to_synax_list()))),
            Self::Syntax(s) => s.0.to_synax_list(),
            _ => self,
        }
    }

    pub fn fold_to_syntax_list<T, E>(
        self,
        f: &mut impl FnMut(Self, T) -> Result<T, E>,
        init: T,
    ) -> Result<T, E> {
        match self {
            Self::Pair(l) => {
                let Pair(car, cdr) = *l;
                cdr.fold_to_syntax_list(f, init)
                    .and_then(|init| f(car, init))
            }
            Self::Syntax(s) => s.0.fold_to_syntax_list(f, init),
            _ => Ok(init),
        }
    }
    pub fn map_to_syntax_list<E>(
        self,
        mut f: impl FnMut(Self) -> Result<Self, E>,
    ) -> Result<Self, E> {
        match self {
            Self::Pair(l) => Ok(Self::Pair(Box::new(Pair(
                f(l.0)?,
                l.1.map_to_syntax_list(f)?,
            )))),
            Self::Syntax(s) => s.0.map_to_syntax_list(f),
            _ => Ok(self),
        }
    }
    // single level unsyntax object
    #[must_use]
    pub fn unsyntax(self) -> Self {
        match self {
            Self::Syntax(s) => s.0,
            s => s,
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

#[derive(Debug, Clone)]
pub enum Param {
    // All the variants besides Zero have a number, so even after auto currying the compiler still
    // knows the arguement index
    Zero,
    One(Rc<str>),
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 1 arg)
    AtLeast1(Rc<str>),
    /// denotes that besides the usual arg count function will take extra args
    /// in form of tree (requires at least 0 args)
    AtLeast0(Rc<str>),
}

impl From<Param> for usize {
    fn from(value: Param) -> Self {
        match value {
            Param::Zero => ZERO_ARG,
            Param::One(_) => ONE_ARG,
            Param::AtLeast1(_) => ONE_VARIADIAC_ARG,
            Param::AtLeast0(_) => ZERO_VARIADIAC_ARG,
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
impl fmt::Display for Param {
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
