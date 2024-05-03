use std::fmt;
use std::fmt::Display;
use std::usize;

pub(crate) mod ast1;
pub(crate) mod ast2;
pub(crate) mod ast3;
pub(crate) mod ast4;

pub(crate) const ZERO_ARG: usize = 0;
pub(crate) const ONE_ARG: usize = 1;
pub(crate) const ZERO_VARIADIAC_ARG: usize = 2;
pub(crate) const ONE_VARIADIAC_ARG: usize = 3;

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

fn map_box<T, U>(b: Box<T>, f: impl FnOnce(T) -> U) -> Box<U> {
    Box::new(f(*b))
}

fn map_into<T, U: From<T>>(b: Box<T>) -> Box<U> {
    map_box(b, Into::into)
}
