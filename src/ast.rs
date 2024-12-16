pub mod scope;
pub mod syntax;
use scope::Scope;
use syntax::{Properties, SourceLocation, Syntax};

use std::{
    collections::BTreeSet,
    fmt::{self, Debug},
    rc::Rc,
};

use crate::evaluator::{Env, EnvRef, Evaluator};

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

#[derive(Clone, PartialEq, Eq, Debug)]
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
impl Eq for Lambda {}

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
    Syntax(Box<Syntax<Ast>>),
    Number(f64),
    Boolean(bool),
    Symbol(Symbol),
    Function(Function),
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Symbol(pub Rc<str>, pub usize);
impl Symbol {
    pub(crate) fn datum_to_syntax(
        self,
        scopes: Option<BTreeSet<Scope>>,
        srcloc: Option<SourceLocation>,
        properties: Option<Properties>,
    ) -> Syntax<Symbol> {
        Syntax(
            self,
            scopes.unwrap_or_default(),
            srcloc.unwrap_or_default(),
            properties.clone().unwrap_or_default(),
        )
    }
}
impl From<&str> for Ast {
    fn from(value: &str) -> Self {
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
            Self::Syntax(s) => write!(f, "#'{}", s.0),
            Self::Number(n) => write!(f, "{n}"),
            Self::Symbol(s) => write!(f, "'{s}"),
            Self::Function(function) => write!(f, "{function}"),
            Self::Boolean(b) => write!(f, "{b}"),
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
        a: Ast,
        b: Ast,
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

    pub fn is_keyword(&self) -> bool {
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

    pub(crate) fn append(self, list: Ast) -> Ast {
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
}
