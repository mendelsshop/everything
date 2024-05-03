use std::{collections::HashSet, iter::empty};

use std::iter;

#[derive(Debug)]
pub enum ParseErrorType {
    EOF,
    // TODO: maybe be put the custom error in here instead of in parse error
    Custom,
    NotADigit(char),
    Mismatch(char, char),
    Fail,
    NotEnoughMatches,
    NoMatchFound,
    SatisfyMismatch(char),
    Unkown,
}

#[derive(Debug)]
pub struct ParseError<'a, E> {
    pub kind: ParseErrorType,
    pub input: &'a str,
    pub error: Option<E>,
}

pub fn run<'a, T, E>(parser: Box<Parser<T, E>>, input: &'a str) -> Result<T, ParseError<'a, E>> {
    let (result, input) = parser(input)?;
    if input.chars().count() == 0 {
        Ok(result)
    } else {
        Err(ParseError {
            // TODO: see if theres any previous errors that were not used because of something like
            // an choice or the like
            kind: ParseErrorType::Unkown,
            input,
            error: None,
        })
    }
}

#[must_use]
pub fn digit<E: 'static>() -> Box<Parser<usize, E>> {
    map(satify(|c| c.is_ascii_digit()), |d| d as usize)
}

#[must_use]
pub fn char<E>(looking_for: char) -> Box<Parser<char, E>> {
    satify(move |c| c == looking_for)
}

#[must_use]
pub fn not_char<E>(looking_for: char) -> Box<Parser<char, E>> {
    satify(move |c| c != looking_for)
}

#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn integer<E: 'static>() -> Box<Parser<usize, E>> {
    map(
        many1(any_of(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])),
        |input| input.collect::<String>().parse().unwrap(),
    )
}

pub fn satify<E>(checker: impl Fn(char) -> bool + 'static + Clone) -> Box<Parser<char, E>> {
    Box::new(move |input: &str| {
        input.chars().next().map_or(
            Err(ParseError {
                kind: ParseErrorType::EOF,
                input: "",
                error: None,
            }),
            |n| {
                if checker(n) {
                    Ok((n, input.split_at(n.len_utf8()).1))
                } else {
                    Err(ParseError {
                        kind: ParseErrorType::SatisfyMismatch(n),
                        input,
                        error: None,
                    })
                }
            },
        )
    })
}

#[must_use]
pub fn take<E>() -> Box<Parser<char, E>> {
    satify(|_| true)
}

#[must_use]
pub fn chain<T: 'static, U: 'static, E: 'static>(
    parser1: Box<Parser<T, E>>,
    parser2: Box<Parser<U, E>>,
) -> Box<Parser<(T, U), E>> {
    Box::new(move |input: &str| {
        let (res1, input) = parser1(input)?;

        let (res2, input) = parser2(input)?;

        Ok(((res1, res2), input))
    })
}

pub fn map<T: 'static, U: 'static, F: Fn(T) -> U + 'static + Clone, E: 'static>(
    parser: Box<Parser<T, E>>,
    map_fn: F,
) -> Box<Parser<U, E>> {
    Box::new(move |input| {
        let (ir, input) = parser(input)?;
        Ok((map_fn(ir), input))
    })
}

pub fn try_map<T: 'static, U: 'static, E: 'static, F: Fn(T) -> Result<U, E> + 'static + Clone>(
    parser: Box<Parser<T, E>>,
    map_fn: F,
) -> Box<Parser<U, E>> {
    Box::new(move |input| {
        let (ir, new_input) = parser(input)?;
        map_fn(ir)
            .map(|ir| (ir, new_input))
            .map_err(|e| ParseError {
                kind: ParseErrorType::Custom,
                input,
                error: Some(e),
            })
    })
}

#[must_use]
pub fn alt<T: 'static, E: 'static>(
    parser1: Box<Parser<T, E>>,
    parser2: Box<Parser<T, E>>,
) -> Box<Parser<T, E>> {
    Box::new(move |input| match parser1(input) {
        Ok((res, input)) => Ok((res, input)),
        Err(_) => parser2(input),
    })
}

#[must_use]
pub fn opt<T: 'static, E: 'static>(parser: Box<Parser<T, E>>) -> Box<Parser<Option<T>, E>> {
    Box::new(move |input| match parser(input) {
        Ok(ok) => Ok((Some(ok.0), ok.1)),
        // TODO: error should return leftover substring
        Err(e) => Ok((None, e.input)),
    })
}

#[must_use]
pub fn many<T: 'static, E: 'static>(
    parser: Box<Parser<T, E>>,
) -> Box<Parser<Option<Box<dyn Iterator<Item = T>>>, E>> {
    Box::new(move |mut input| {
        let mut init: Option<Box<dyn Iterator<Item = T>>> = None;
        while let Ok((v, new_input)) = parser(input) {
            input = new_input;

            let v = iter::once(v);
            init = match init {
                Some(old_v) => Some(Box::new(old_v.chain(v))),
                None => Some(Box::new(v)),
            };
        }
        Ok((init, input))
    })
}

#[must_use]
pub fn keep_left<T: 'static, U: 'static, E: 'static>(
    left_parser: Box<Parser<T, E>>,
    right_parser: Box<Parser<U, E>>,
) -> Box<Parser<T, E>> {
    map(chain(left_parser, right_parser), |i| i.0)
}

#[must_use]
pub fn keep_right<T: 'static, U: 'static, E: 'static>(
    left_parser: Box<Parser<T, E>>,
    right_parser: Box<Parser<U, E>>,
) -> Box<Parser<U, E>> {
    map(chain(left_parser, right_parser), |i| i.1)
}

#[must_use]
pub fn inbetween<T: 'static, U: 'static, V: 'static, E: 'static>(
    left_parser: Box<Parser<T, E>>,
    middle_parser: Box<Parser<U, E>>,
    right_parser: Box<Parser<V, E>>,
) -> Box<Parser<U, E>> {
    keep_left(keep_right(left_parser, middle_parser), right_parser)
}

#[must_use]
pub fn many1<T: 'static, E: 'static>(
    parser: Box<Parser<T, E>>,
) -> Box<Parser<Box<dyn Iterator<Item = T>>, E>> {
    let many = many(parser);
    Box::new(move |input| match many(input)? {
        (None, input) => Err(ParseError {
            kind: ParseErrorType::NotEnoughMatches,
            input,
            error: None,
        }),
        (Some(v), input) => Ok((v, input)),
    })
}

#[must_use]
pub fn fail<T, E>() -> Box<Parser<T, E>> {
    Box::new(move |input| {
        Err(ParseError {
            kind: ParseErrorType::Fail,
            input,
            error: None,
        })
    })
}

pub fn unit<T: 'static + Clone, E>(val: T) -> Box<Parser<T, E>> {
    Box::new(move |input| Ok((val.clone(), input)))
}
pub fn with_error<T: 'static + Clone, E: 'static + Clone>(
    parser: Box<Parser<T, E>>,
    error: impl FnOnce(&str) -> E + Clone + 'static,
) -> Box<Parser<T, E>> {
    Box::new(move |input| {
        parser(input).map_err(|mut e| {
            // only set the error if its not been set yet
            e.error = e.error.or(Some(error.clone()(input)));
            e
        })
    })
}
#[must_use]
pub fn seq<T: 'static, E: 'static>(
    parsers: Vec<Box<Parser<T, E>>>,
) -> Box<Parser<impl Iterator<Item = T>, E>> {
    Box::new(move |mut input| {
        let mut res: Box<dyn Iterator<Item = T>> = Box::new(empty());
        for parser in &parsers {
            let (res_part, new_input) = parser(input)?;
            input = new_input;
            res = Box::new(res.chain(iter::once(res_part)));
        }

        Ok((res, input))
    })
}

#[must_use]
pub fn choice<T: 'static, E: 'static>(parsers: Vec<Box<Parser<T, E>>>) -> Box<Parser<T, E>> {
    Box::new(move |input| {
        for parser in parsers.clone() {
            match parser(input) {
                Ok(ok) => return Ok(ok),
                Err(_) => continue,
            }
        }
        fail()(input)
    })
}

#[must_use]
pub fn not_choice<T: 'static, E: 'static>(parsers: Vec<Box<Parser<T, E>>>) -> Box<Parser<T, E>> {
    Box::new(move |input| {
        let mut res = None;
        for parser in parsers.clone() {
            res = Some(parser(input)?);
        }
        res.ok_or(ParseError {
            kind: ParseErrorType::NoMatchFound,
            input,
            error: None,
        })
    })
}

pub fn any_of<E>(chars: impl IntoIterator<Item = char>) -> Box<Parser<char, E>> {
    let p: HashSet<char> = chars.into_iter().collect();
    satify(move |c| p.contains(&c))
}

pub fn not_any_of<E>(chars: impl IntoIterator<Item = char>) -> Box<Parser<char, E>> {
    let p: HashSet<char> = chars.into_iter().collect();
    satify(move |c| !p.contains(&c))
    // not_choice(chars.into_iter().map(not_char).collect())
}

#[must_use]
pub fn string<E: 'static>(to_match: &str) -> Box<Parser<String, E>> {
    map(seq(to_match.chars().map(|c| char(c)).collect()), |chars| {
        chars.collect::<String>()
    })
}

#[must_use]
pub fn sep<T: 'static, U: 'static, E: 'static>(
    parser: Box<Parser<T, E>>,
    delimeter: Box<Parser<U, E>>,
) -> Box<Parser<Option<Box<dyn Iterator<Item = T>>>, E>> {
    let rest = many(keep_right(delimeter, parser.clone_box()));
    Box::new(move |input| {
        let (first, new_input) = match parser(input) {
            Ok(v) => v,
            Err(e) => return Ok((None, e.input)),
        };
        let first = iter::once(first);
        let (rest, input) = rest(new_input)?;
        Ok(match rest {
            None => (Some(Box::new(first)), new_input),
            Some(v) => (Some(Box::new(first.chain(v))), input),
        })
    })
}

#[must_use]
pub fn sep1<T: 'static, U: 'static, E: 'static>(
    parser: Box<Parser<T, E>>,
    delimeter: Box<Parser<U, E>>,
) -> Box<Parser<Box<dyn Iterator<Item = T>>, E>> {
    let sep = sep(parser, delimeter);
    Box::new(move |input| match sep(input)? {
        (None, input) => Err(ParseError {
            kind: ParseErrorType::NotEnoughMatches,
            input,
            error: None,
        }),
        (Some(v), input) => Ok((v, input)),
    })
}

#[must_use]
pub fn white_space<E: 'static>() -> Box<Parser<Option<Box<dyn Iterator<Item = char>>>, E>> {
    many(any_of([' ', '\n', '\t']))
}

pub trait CloneFn<T, E>: Fn(&str) -> Result<(T, &str), ParseError<'_, E>> {
    fn clone_box<'a>(&self) -> Box<dyn CloneFn<T, E> + 'a>
    where
        Self: 'a;
}

impl<T, F, E> CloneFn<T, E> for F
where
    F: Fn(&str) -> Result<(T, &str), ParseError<'_, E>> + Clone,
{
    fn clone_box<'a>(&self) -> Box<dyn CloneFn<T, E> + 'a>
    where
        Self: 'a,
    {
        Box::new(self.clone())
    }
}

impl<'a, T: 'a, E: 'a> Clone for Box<dyn 'a + CloneFn<T, E>> {
    fn clone(&self) -> Self {
        (**self).clone_box()
    }
}

pub type Parser<T, E> = dyn CloneFn<T, E>;
