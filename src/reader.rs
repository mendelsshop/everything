use crate::ast::{Ast, Pair, Symbol};

use std::iter::Peekable;

pub struct Reader(String);

impl Reader {
    pub const fn new() -> Self {
        Self(String::new())
    }
    pub fn new_with_input(input: &impl ToString) -> Self {
        Self(input.to_string())
    }
}

#[derive(Debug)]
pub struct OwnedChars {
    pub(crate) string: String,
    pub(crate) position: usize,
}

impl Iterator for OwnedChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        let c = self.string[self.position..].chars().next()?;
        self.position += c.len_utf8();
        Some(c)
    }
}
impl DoubleEndedIterator for OwnedChars {
    fn next_back(&mut self) -> Option<Self::Item> {
        let c = self.string[self.position..].chars().next_back()?;
        self.position -= c.len_utf8();
        Some(c)
    }
}

pub trait OwnedCharsExt {
    fn chars(self) -> OwnedChars;
}

impl OwnedCharsExt for String {
    fn chars(self) -> OwnedChars {
        OwnedChars {
            string: self,
            position: 0,
        }
    }
}

// pub type Input = Peekable<OwnedChars>;
pub type Input = Peekable<OwnedChars>;

pub type ReaderInnerResult = Result<(Ast, Input), (String, Input)>;

impl Reader {
    // we have empty continuations for if we run out of input, but we can recover if we get more
    // input
    pub fn read(&mut self) -> Result<Ast, String> {
        let input = <String as Clone>::clone(&self.0).chars().peekable();
        match Self::read_inner(input, &mut || None) {
            Ok((ast, rest)) => {
                self.0 = rest.collect();
                Ok(ast)
            }
            Err((reason, rest)) => {
                self.0 = rest.collect();
                Err(reason)
            }
        }
    }
    pub fn read_with_continue(
        &mut self,
        mut empty_continuation: impl FnMut() -> String,
    ) -> Result<Ast, String> {
        let input = <String as Clone>::clone(&self.0).chars().peekable();
        match Self::read_inner(input, &mut || Some(empty_continuation())) {
            Ok((ast, rest)) => {
                self.0 = rest.collect();
                Ok(ast)
            }
            Err((reason, rest)) => {
                self.0 = rest.collect();
                Err(reason)
            }
        }
    }
    pub(crate) fn read_inner(
        input: Input,
        empty_continuation: &mut impl FnMut() -> Option<String>,
    ) -> ReaderInnerResult {
        let mut input = Self::read_whitespace_and_comments(input).1;
        match input.peek() {
            // TODO: quote
            Some(bracket @ ('(' | '[')) => {
                let bracket = *bracket;
                input.next();
                Self::read_list(input, bracket, empty_continuation)
            }
            Some(')' | ']') => {
                input.next();
                Err(("unfinished pair".to_string(), input))
            }
            Some('#') => {
                input.next();
                match input.next() {
                    Some('t') => Ok((Ast::Boolean(true), input)),
                    Some('f') => Ok((Ast::Boolean(false), input)),
                    Some('%') => {
                        input.next_back();
                        input.next_back();
                        Self::read_symbol(input)
                    }
                    Some('\'') => todo!("syntax object syntax"),
                    Some(s) => {
                        input.next_back();
                        Err((
                            format!("# must be followed by t, f, % or ', found {s}"),
                            input,
                        ))
                    }
                    None => Err(("# must be followed by t, f, % or '".to_string(), input)),
                }
            }
            Some('\'') => {
                input.next();
                Self::read_inner(input, empty_continuation).map(|(quoted, input)| {
                    (
                        Ast::Pair(Box::new(Pair(
                            Ast::Symbol("quote".into()),
                            Ast::Pair(Box::new(Pair(quoted, Ast::TheEmptyList))),
                        ))),
                        input,
                    )
                })
            }
            Some(n) if n.is_ascii_digit() => Self::read_number(input),
            Some(_) => Self::read_symbol(input),
            None => empty_continuation()
                .ok_or((String::from("empty input"), input))
                .and_then(|input| {
                    let input = input.chars().peekable();
                    Self::read_inner(input, empty_continuation)
                }),
        }
    }

    pub(crate) fn read_whitespace_and_comments(mut input: Input) -> (bool, Input) {
        let mut found = false;
        while let Some(c) = input.peek() {
            match c {
                ';' => {
                    found = true;
                    // we do find to skip untill we find newline, this is essentially
                    // what skip while does, but skip while returns a new type and we
                    // cannot do impl trait in type alias so this does not work for with
                    // my input type
                    input.find(|c| *c != '\n');
                }
                c if c.is_whitespace() => {
                    found = true;
                    input.next();
                }
                _ => break,
            }
        }
        (found, input)
    }

    // parse symbol if not followed by space paren or comment
    // invariant Some('.') | Some(c) if c.is_ascci_digit() = input.peek()
    // TODO: if number is immediatly followed by symbol combine into one symbol
    pub(crate) fn read_number(input: Input) -> ReaderInnerResult {
        let (first, mut input) = Self::read_digit(input);
        let (second, input) = {
            if input.peek().copied().filter(|c| *c == '.').is_some() {
                input.next();
                let (digits, input) = Self::read_digit(input);
                (format!(".{digits}"), input)
            } else {
                (String::new(), input)
            }
        };
        let (last, input) = Self::read_symbol_inner(input);
        match (first.as_str(), second.as_str(), last.as_str()) {
            ("", "." | "", "") => Err(("invalid syntax dangling dot".to_owned(), input)),
            (_, _, "") => match format!("{first}{second}").parse::<f64>() {
                Ok(n) => Ok((Ast::Number(n), input)),
                Err(e) => Err((e.to_string(), input)),
            },

            (first, second, last) => Ok((
                Ast::Symbol(Symbol(format!("{first}{second}{last}").into())),
                input,
            )),
        }
    }
    pub(crate) fn read_digit(mut input: Input) -> (String, Input) {
        let mut number = String::new();
        while let Some(n) = input.peek().copied().filter(char::is_ascii_digit) {
            input.next();
            number.push(n);
        }
        (number, input)
    }
    // constraints input.next() == Some(c) if c != whitespace or comment or paren
    pub(crate) fn read_symbol(input: Input) -> ReaderInnerResult {
        let (symbol, input) = Self::read_symbol_inner(input);
        Ok((Ast::Symbol(Symbol(symbol.into())), input))
    }

    pub(crate) fn read_list(
        mut input: Input,
        bracket: char,
        empty_continuation: &mut impl FnMut() -> Option<String>,
    ) -> ReaderInnerResult {
        input = Self::read_whitespace_and_comments(input).1;
        match input.peek() {
            // TODO: dot tailed list and pair instead of list
            Some(end_bracket @ (')' | ']')) => {
                let expected_end_bracket = if bracket == '(' { ')' } else { ']' };
                let end_bracket = *end_bracket;
                input.next();
                if end_bracket == expected_end_bracket {
                    Ok((Ast::TheEmptyList, input))
                } else {
                    Err((format!("unfinished pair expected {expected_end_bracket} to finish the pair but found {end_bracket}"), input))
                }
            }
            Some('.') => {
                let item: Ast;
                input.next();
                (item, input) = Self::read_inner(input, empty_continuation)?;
                input = Self::read_end_parenthesis(input, empty_continuation)?;
                Ok((item, input))
            }
            Some(_) => {
                let item: Ast;
                (item, input) = Self::read_inner(input, empty_continuation)?;
                let item2: Ast;
                (item2, input) = Self::read_list(input, bracket, empty_continuation)?;
                Ok((Ast::Pair(Box::new(Pair(item, item2))), input))
            }
            None => {
                input = empty_continuation()
                    .ok_or(("unfinished list".to_string(), input))
                    .map(|input| input.chars().peekable())?;

                Self::read_list(input, bracket, empty_continuation)
            }
        }
    }

    pub(crate) fn read_end_parenthesis(
        mut input: Input,
        empty_continuation: &mut impl FnMut() -> Option<String>,
    ) -> Result<Input, (String, Input)> {
        input = Self::read_whitespace_and_comments(input).1;
        match input.next() {
            Some(')') => Ok(input),
            Some(char) => Err((
                format!("invalid termination character of dotted list {char}"),
                input,
            )),
            None => {
                input = empty_continuation()
                    .ok_or(("unfinished list".to_string(), input))
                    .map(|input| input.chars().peekable())?;
                Self::read_end_parenthesis(input, empty_continuation)
            }
        }
    }

    pub(crate) fn read_symbol_inner(mut input: Input) -> (String, Input) {
        let mut str = String::new();
        while let Some(char) = input.peek().copied() {
            if char.is_whitespace() || ['(', '[', ']', ')', ';', '"', '\''].contains(&char) {
                break;
            }
            input.next();
            str.push(char);
        }
        (str, input)
    }
}

#[cfg(test)]
mod reader_tests {
    use crate::{
        ast::{Ast, Pair},
        list,
        reader::Reader,
    };

    #[test]
    pub fn read_test_number() {
        let mut reader = Reader("42".to_string());
        assert_eq!(reader.read(), Ok(Ast::Number(42.)));
    }
    #[test]
    pub fn read_test_float() {
        let mut reader = Reader("36.4".to_string());
        assert_eq!(reader.read(), Ok(Ast::Number(36.4)));
    }
    #[test]
    pub fn read_test_symbol() {
        let mut reader = Reader("foo".to_string());
        assert_eq!(reader.read(), Ok(Ast::Symbol("foo".into())));
    }
    #[test]
    pub fn read_test_number_symbol() {
        let mut reader = Reader("1foo".to_string());
        assert_eq!(reader.read(), Ok(Ast::Symbol("1foo".into())));
    }
    #[test]
    pub fn read_test_float_symbol() {
        let mut reader = Reader("1.5foo".to_string());
        assert_eq!(reader.read(), Ok(Ast::Symbol("1.5foo".into())));
    }
    #[test]
    pub fn read_test_quote_symbol() {
        let mut reader = Reader("'foo".to_string());
        assert_eq!(
            reader.read(),
            Ok(list!(
                Ast::Symbol("quote".into()),
                Ast::Symbol("foo".into())
            ))
        );
    }
    #[test]
    pub fn read_test_quote_empty() {
        let mut reader = Reader("'".to_string());
        assert!(reader.read().is_err(),);
    }

    #[test]
    pub fn read_test_list() {
        let mut reader = Reader("(foo 1)".to_string());
        assert_eq!(
            reader.read(),
            Ok(list!(Ast::Symbol("foo".into()), Ast::Number(1.)))
        );
    }
    #[test]
    pub fn read_test_nested_list() {
        let mut reader = Reader("(bar (def 1) 23)".to_string());
        assert_eq!(
            reader.read(),
            Ok(list!(
                Ast::Symbol("bar".into()),
                list!(Ast::Symbol("def".into()), Ast::Number(1.)),
                Ast::Number(23.),
            ))
        );
    }
    #[test]
    pub fn read_test_list_unpaired() {
        let mut reader = Reader("( foo bar".to_string());
        assert!(reader.read().is_err(),);
    }
    #[test]
    pub fn read_test_pair() {
        let mut reader = Reader("(foo . 1)".to_string());
        assert_eq!(
            reader.read(),
            Ok(Ast::Pair(Box::new(Pair(
                Ast::Symbol("foo".into()),
                Ast::Number(1.)
            ))))
        );
    }
    #[test]
    pub fn read_test_pair_unpaired() {
        let mut reader = Reader("( foo . bar".to_string());
        assert!(reader.read().is_err(),);
    }
    #[test]
    pub fn read_test_pair_missing_cdr() {
        let mut reader = Reader("( foo . )".to_string());
        assert!(reader.read().is_err(),);
    }
    #[test]
    pub fn read_test_pair_missing_cdr_unpaired() {
        let mut reader = Reader("( foo .".to_string());
        assert!(reader.read().is_err(),);
    }
    #[test]
    pub fn read_test_quote_list() {
        let mut reader = Reader("'(1 foo)".to_string());
        assert_eq!(
            reader.read(),
            Ok(list!(
                Ast::Symbol("quote".into()),
                list!(Ast::Number(1.), Ast::Symbol("foo".into()),)
            ))
        );
    }
    #[test]
    pub fn read_test_list_quote_unpaired() {
        let mut reader = Reader("'( ab bar".to_string());
        assert!(reader.read().is_err(),);
    }
    #[test]
    pub fn read_test_quote_pair() {
        let mut reader = Reader("'(365 . abc)".to_string());
        assert_eq!(
            reader.read(),
            Ok(list!(
                Ast::Symbol("quote".into()),
                Ast::Pair(Box::new(
                    Pair(Ast::Number(365.), Ast::Symbol("abc".into()),)
                ))
            ))
        );
    }
}
