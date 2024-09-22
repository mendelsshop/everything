#![allow(dead_code)]

use parse_int::parse;
use std::iter;
// chars on us keyboard not used: `, , \,/,,,=
// qussiquote -> :
// unquote -> $
use crate::{
    ast::{Ast, Boolean, Pair},
    list,
    pc::{
        alt, any_of, chain, char, choice, inbetween, keep_left, keep_right, many, many1, map,
        not_any_of, not_char, opt, run, satify, seq, string, try_map, with_error, ParseError,
        Parser,
    },
};
#[derive(Debug, Clone)]
pub enum Error {
    InvalidNumber,
    InvalidModuleName,
    MissingThenBlock,
    MissingElseBlock,
}
fn ws_or_comment() -> Box<Parser<Option<Box<dyn DoubleEndedIterator<Item = char>>>, Error>> {
    map(
        many(alt(
            keep_right(char('!'), keep_left(many(not_char('\n')), opt(char('\n')))),
            map(any_of([' ', '\n', '\t']), |i| Some(opaquify(iter::once(i)))),
        )),
        |r| -> Option<Box<dyn DoubleEndedIterator<Item = char>>> {
            r.map(|r| opaquify(r.flatten().flatten()))
        },
    )
}

fn opaquify(
    f: impl DoubleEndedIterator<Item = char> + 'static,
) -> Box<dyn DoubleEndedIterator<Item = char>> {
    Box::new(f)
}
fn scope_list(p: Box<Parser<Ast, Error>>) -> Box<Parser<Vec<Ast>, Error>> {
    inbetween(
        keep_right(ws_or_comment(), char('᚜')),
        map(many(p), |r| r.map_or_else(Vec::new, Iterator::collect)),
        opt(keep_right(ws_or_comment(), char('᚛'))),
    )
}
fn scope(p: Box<Parser<Ast, Error>>) -> Box<Parser<Ast, Error>> {
    map(scope_list(p), |scope| {
        list!(
            "begin".into(),
            scope.into_iter().rfold(Ast::TheEmptyList, cons)
        )
    })
}

macro_rules! to_list {
    ($e:expr) => {
        $e.into_iter().rfold(Ast::TheEmptyList, cons)
    };
}
fn everythingexpr() -> Box<Parser<Ast, Error>> {
    // needs to be its own new closure so that we don't have infinite recursion while creating the parser (so we add a level of indirection)
    map(
        keep_left(
            chain(
                Box::new(|input| {
                    keep_right(
                        ws_or_comment(),
                        choice(
                            [
                                literal(),
                                stmt(),
                                terminal_everything(),
                                ident_everything(),
                                application(),
                                special_start(),
                                scope(everythingexpr()),
                            ]
                            .to_vec(),
                        ),
                    )(input)
                }),
                many(choice(vec![
                    // match >> before > so >> doesn't become >, >
                    string(">>"),
                    string(">"),
                    string("<"),
                    keep_right(
                        char('^'),
                        choice(vec![string("car"), string("cdr"), string("cgr")]),
                    ),
                ])),
            ),
            ws_or_comment(),
        ),
        |mut r| {
            if let Some(accesors) = r.1 {
                let new_acces = |accesor: String, expr| list![accesor.into(), expr];
                for mut accesor in accesors {
                    if accesor == ">>" {
                        accesor.clear();
                        accesor += "print";
                    } else if accesor == ">" {
                        // TODO: make printline function just calls print + newline
                        accesor.clear();
                        accesor += "println";
                    }
                    // if it says to not print we just ignore it
                    if accesor == "<" {
                        continue;
                    }

                    r.0 = new_acces(accesor, r.0);
                }
            }
            r.0
        },
    )
}

fn application() -> Box<Parser<Ast, Error>> {
    // TODO: dot tail
    map(
        inbetween(
            keep_right(ws_or_comment(), any_of(call_start().iter().copied())),
            many(everythingexpr()),
            opt(keep_right(
                ws_or_comment(),
                any_of(call_end().iter().copied()),
            )),
        ),
        |r| r.map_or(Ast::TheEmptyList, |app| app.rfold(Ast::TheEmptyList, cons)),
    )
}
fn cons(list: Ast, current: Ast) -> Ast {
    Ast::Pair(Box::new(Pair(current, list)))
}
pub fn parse_everything(input: &str) -> Result<Ast, ParseError<'_, Error>> {
    everythingexpr()(input).map(|res| res.0)
}

pub fn everything_parse(input: &str) -> Result<Vec<Ast>, ParseError<'_, Error>> {
    run(
        map(many(everythingexpr()), |r| {
            r.map_or(vec![], Iterator::collect)
        }),
        input,
    )
}

fn literal() -> Box<Parser<Ast, Error>> {
    choice([boolean(), hexnumber(), stringdot()].to_vec())
}

fn boolean() -> Box<Parser<Ast, Error>> {
    choice(vec![
        map(string("&"), |_| Ast::Boolean(Boolean::True)),
        map(string("|"), |_| Ast::Boolean(Boolean::False)),
        map(string("?"), |_| Ast::Boolean(Boolean::Maybe)),
    ])
}

fn hexnumber() -> Box<Parser<Ast, Error>> {
    let digit = any_of(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let hex_digit = choice([digit.clone(), any_of(['a', 'b', 'c', 'd', 'e', 'f'])].to_vec());
    let parese_num = |digit_type: Box<Parser<char, Error>>| {
        try_map(
            chain(
                many(digit_type.clone()),
                opt(keep_right(char('%'), many1(digit_type))),
            ),
            |r| {
                let number = match r {
                    (None, None) => return Err(Error::InvalidNumber),
                    (None, Some(s)) => (String::new(), s.collect()),
                    (Some(s), None) => (s.collect(), String::new()),
                    (Some(s), Some(r)) => (s.collect(), r.collect()),
                };
                Ok(Ast::Number(
                    format!("0x{}.{}", number.0, number.1)
                        .parse::<hexponent::FloatLiteral>()
                        .unwrap()
                        .into(),
                ))
            },
        )
    };
    alt(
        keep_right(string("0x"), parese_num(hex_digit)),
        parese_num(digit),
    )
}

fn stringdot() -> Box<Parser<Ast, Error>> {
    inbetween(
        char('.'),
        map(many(alt(escape_string(), not_char('.'))), |r| {
            Ast::String(r.map_or_else(String::new, Iterator::collect).into())
        }),
        opt(char('.')),
    )
}
/// | Escape sequence | Description |
/// |:-:|:-:|
/// | `\n` | newline |
/// | `\t` | tab |
/// | `\r` | carriage return |
/// | `\b` | backspace |
/// | `\f` | form feed |
/// | `\a` | alert |
/// | `\v` | vertical tab |
/// | `\e` | escape |
/// | `\\` | backslash |
/// | ```\` ``` | single quote |
/// | ```\x{hex}``` | hexadecimal value in ascii representation |
/// | `\u{hex}` | Unicode character |\
fn escape_string() -> Box<Parser<char, Error>> {
    keep_right(
        // TODO: this more escape characters
        char('\\'),
        choice(vec![
            map(char('a'), |_| '\x07'),
            map(char('n'), |_| '\n'),
            char('.'),
            char('\\'),
        ]),
    )
}

fn stmt() -> Box<Parser<Ast, Error>> {
    choice(
        [
            mod_stmt(),
            if_stmt(),
            until_stmt(),
            go_through_stmt(),
            continue_doing_stmt(),
            fn_stmt(),
            link_stmt(),
            let_stmt(),
            class_stmt(),
        ]
        .to_vec(),
    )
}

fn mod_stmt() -> Box<Parser<Ast, Error>> {
    keep_right(
        string("mod"),
        keep_right(
            ws_or_comment(),
            map(
                chain(
                    with_error(satify(|c| c.is_ascii_alphabetic()), |_| {
                        Error::InvalidModuleName
                    }),
                    keep_right(
                        ws_or_comment(),
                        alt(scope_list(everythingexpr()), map(stringdot(), |s| vec![s])),
                    ),
                ),
                |(name, code)| {
                    list!(
                        "module".into(),
                        name.to_string().as_str().into(),
                        code.into_iter().rfold(Ast::TheEmptyList, cons)
                    )
                },
            ),
        ),
    )
}

fn let_stmt() -> Box<Parser<Ast, Error>> {
    map(
        keep_right(
            string("let"),
            chain(
                keep_right(ws_or_comment(), ident_everything()),
                keep_right(keep_right(ws_or_comment(), char('=')), everythingexpr()),
            ),
        ),
        |r| list!("define".into(), r.0, r.1),
    )
}

fn method_stmt() -> Box<Parser<Ast, Error>> {
    map(
        chain(
            keep_right(
                keep_right(
                    ws_or_comment(),
                    keep_right(
                        string("public"),
                        keep_right(ws_or_comment(), string("object")),
                    ),
                ),
                keep_right(
                    ws_or_comment(),
                    inbetween(
                        char('('),
                        chain(
                            keep_right(ws_or_comment(), ident()),
                            chain(
                                keep_right(ws_or_comment(), hexnumber()),
                                opt(keep_right(ws_or_comment(), alt(char('+'), char('*')))),
                            ),
                        ),
                        char(')'),
                    ),
                ),
            ),
            keep_right(
                keep_right(
                    ws_or_comment(),
                    keep_right(
                        string("throws"),
                        keep_right(ws_or_comment(), string("Exception")),
                    ),
                ),
                scope_list(everythingexpr()),
            ),
        ),
        |r| {
            let name = r.0 .0.into();
            let number = r.0 .1 .0;
            let varidiac = r.0 .1 .1;
            let fn_info = varidiac.map_or(number.clone(), |varidiac| {
                list![number, varidiac.to_string().into()]
            });
            let scope = r.1;
            list![name, list!["lambda".into(), fn_info;to_list!( scope)],]
        },
    )
}

fn class_stmt() -> Box<Parser<Ast, Error>> {
    // lisp version of class will be
    // (class name (method1 (lambda ...)) (filed1 value) (filed2) ...)
    map(
        chain(
            keep_right(
                keep_right(ws_or_comment(), string("class")),
                keep_right(ws_or_comment(), ident()),
            ),
            inbetween(
                keep_right(ws_or_comment(), char('᚜')),
                many(keep_right(
                    ws_or_comment(),
                    alt(method_stmt(), map(ident_everything(), |r| list![r])),
                )),
                opt(keep_right(ws_or_comment(), char('᚛'))),
            ),
        ),
        |r| {
            let name = r.0;
            let fields = r.1.map_or_else(Vec::new, Iterator::collect);
            list!["class".into(), name.into(); to_list!(fields)]
        },
    )
}

fn if_stmt() -> Box<Parser<Ast, Error>> {
    // TODO: allow if else if
    map(
        seq(vec![
            keep_right(string("if"), everythingexpr()),
            keep_right(
                keep_right(ws_or_comment(), string("then")),
                with_error(scope(everythingexpr()), |_| Error::MissingThenBlock),
            ),
            keep_right(
                keep_right(ws_or_comment(), string("else")),
                with_error(scope(everythingexpr()), |_| Error::MissingElseBlock),
            ),
        ]),
        |mut r| {
            let if_ident = "if".into();
            let cond = r.next().unwrap();
            let cons = r.next().unwrap();
            let alt = r.next().unwrap();
            list![if_ident, cond, cons, alt]
        },
    )
}

fn until_stmt() -> Box<Parser<Ast, Error>> {
    map(
        chain(
            keep_right(string("while"), everythingexpr()),
            keep_right(
                ws_or_comment(),
                keep_right(string("do"), scope_list(everythingexpr())),
            ),
        ),
        |(cond, scope)| {
            // TODO: should it be (while cond expr ...) or (while cond (begin ...))
            let while_ident = "while".into();
            list!(while_ident, cond;to_list!( scope))
        },
    )
}

fn go_through_stmt() -> Box<Parser<Ast, Error>> {
    map(
        chain(
            keep_right(
                string("for"),
                keep_right(ws_or_comment(), ident_everything()),
            ), // TODO: use identifier parserl, not the full blown expression parser
            chain(
                keep_right(keep_right(ws_or_comment(), string("in")), everythingexpr()),
                scope_list(everythingexpr()),
            ),
        ),
        |(name, (iter, scope))| {
            // TODO: should it be (for name iter expr ...) or (for name iter (begin ...))
            let for_ident = "for".into();
            list![for_ident, name, iter;to_list! (scope)]
        },
    )
}

fn continue_doing_stmt() -> Box<Parser<Ast, Error>> {
    map(
        chain(string("loop"), scope_list(everythingexpr())),
        |(ident, scope)| list![ident.into(); to_list! (scope)],
    )
}

fn link_stmt() -> Box<Parser<Ast, Error>> {
    map(
        chain(
            keep_right(
                string("ln"),
                // makeing sure that there is atleast two labels
                keep_right(ws_or_comment(), label_everything()),
            ),
            many1(keep_right(ws_or_comment(), label_everything())),
        ),
        |res| {
            let link_ident = "link".into();
            let goto = res.0;
            list!(link_ident, goto; to_list!(res.1))
        },
    )
}

fn fn_stmt() -> Box<Parser<Ast, Error>> {
    // fanction - through away, name - keep char | everythingexpr
    // optinal param count (base10) - keep -> optinal everythingexpr | usize
    // optinal varidac keep scope > optional char | varidac
    // scope keep everythingexpr

    // (chain (keep right "fanction" name(char)), (chain, (opt number) (chain (opt varidiac), scope))
    map(
        chain(
            keep_right(
                string("fn"),
                opt(keep_right(
                    ws_or_comment(),
                    satify(unic_emoji_char::is_emoji_presentation),
                )),
            ),
            chain(
                opt(keep_right(ws_or_comment(), hexnumber())),
                chain(
                    opt(keep_right(ws_or_comment(), any_of(['*', '+']))),
                    scope_list(everythingexpr()),
                ),
            ),
        ),
        |r| {
            let map_to_everything = |c: Option<char>, mapper: fn(String) -> Ast| {
                c.as_ref().map(ToString::to_string).map(mapper)
            };
            let name = map_to_everything(r.0, Ast::from);
            // TODO: maybe if no count given then randomly choose a count
            let param_count = r.1 .0.unwrap();
            let variadic = map_to_everything(r.1 .1 .0, Ast::from);
            let scope = r.1 .1 .1;
            let fn_ident = "lambda".into();
            // function can either be (lambda (n *) exprs) or (lambda (n +) exprs) or (lambda n exprs) n = arg count
            let lambda = if let Some(variadic) = variadic {
                list!(fn_ident, param_count; to_list!( scope))
            } else {
                list!(fn_ident, list![param_count]; to_list!( scope))
            };
            if let Some(name) = name {
                let define_ident = "define".into();
                list![define_ident, name, lambda]
            } else {
                lambda
            }
        },
    )
}

fn ident_everything() -> Box<Parser<Ast, Error>> {
    map(ident(), Into::into)
}

fn ident() -> Box<Parser<String, Error>> {
    map(
        many1(not_any_of(
            call_start()
                .iter()
                .chain(special_char())
                .chain(call_end())
                .copied(),
        )),
        std::iter::Iterator::collect,
    )
}

const fn special_char() -> &'static [char] {
    &[
        '@', '\'', '"', ';', '\n', '\t', '<', '>', '^', ' ', '᚜', '᚛', '.', '!', '$', ':', '%',
    ]
}

const fn call_start() -> &'static [char] {
    &[
        '(', '༺', '༼', '⁅', '⁽', '₍', '⌈', '⌊', '❨', '❪', '❬', '❮', '❰', '❲', '❴', '⟅', '⟦', '⟨',
        '⟪', '⟬', '⟮', '⦃', '⦅', '⦇', '⦉', '⦋', '⦍', '⦏', '⦑', '⦓', '⦕', '⦗', '⧘', '⧚', '⸢', '⸤',
        '⸦', '⸨', '\u{2e55}', '\u{2e57}', '\u{2e59}', '\u{2e5b}', '〈', '《', '「', '『', '【',
        '〔', '〖', '〘', '〚', '﹙', '﹛', '﹝', '（', '［', '｛', '｟', '｢', '{', '[',
    ]
}

const fn call_end() -> &'static [char] {
    &[
        ')', '༻', '༽', '⁆', '⁾', '₎', '⌉', '⌋', '❩', '❫', '❭', '❯', '❱', '❳', '❵', '⟆', '⟧', '⟩',
        '⟫', '⟭', '⟯', '⦄', '⦆', '⦈', '⦊', '⦌', '⦎', '⦐', '⦒', '⦔', '⦖', '⦘', '⧙', '⧛', '⸣', '⸥',
        '⸧', '⸩', '\u{2e56}', '\u{2e58}', '\u{2e5a}', '\u{2e5c}', '〉', '》', '」', '』', '】',
        '〕', '〗', '〙', '〛', '﹚', '﹜', '﹞', '）', '］', '｝', '｠', '｣', '}', ']',
    ]
}

fn terminal_everything() -> Box<Parser<Ast, Error>> {
    alt(
        map(string("skip"), |s| list![s.into()]),
        list_expr(string("stop")),
    )
}

fn special_start() -> Box<Parser<Ast, Error>> {
    choice(vec![
        quoted_everything(),
        label_everything(),
        param_everything(),
        unquoted_everything(),
        quassi_quoted_everything(),
    ])
}

fn list_expr(first: Box<Parser<impl Into<Ast> + 'static, Error>>) -> Box<Parser<Ast, Error>> {
    map(
        chain(map(first, Into::into), everythingexpr()),
        |(first, expr)| list![first, expr],
    )
}

fn quoted_everything() -> Box<Parser<Ast, Error>> {
    list_expr(map(string(";"), |_| "quote"))
}

fn quassi_quoted_everything() -> Box<Parser<Ast, Error>> {
    list_expr(map(string(":"), |_| "quasiquote"))
}

fn unquoted_everything() -> Box<Parser<Ast, Error>> {
    list_expr(map(string("$"), |_| "unquote"))
}

fn label_everything() -> Box<Parser<Ast, Error>> {
    map(keep_right(char('@'), ident()), |res| Ast::Label(res.into()))
}

fn param_everything() -> Box<Parser<Ast, Error>> {
    inbetween(
        any_of(['\'', '"']),
        map(
            many1(any_of(['0', '1', '2', '3', '4', '5', '6', '7'])),
            |res| {
                list!(
                    "param".into(),
                    Ast::Number(parse(&format!("0o{}", res.collect::<String>())).unwrap())
                )
            },
        ),
        opt(any_of(['\'', '"'])),
    )
}

#[cfg(test)]
mod tests {
    // TODO: remake some of thests now that >> > < are valid in any expression
    use crate::{
        ast::Ast,
        lexer::{parse_everything, Boolean},
        list,
    };

    #[test]
    pub fn everything() {
        println!("{:?}", parse_everything("if 1 do ᚜1 unless 1 than ᚜1 2᚛ else ᚜1᚛ 2᚛ otherwise ᚜if 1 do ᚜1 2᚛ otherwise ᚜until 1 then ᚜1 2᚛᚛᚛"));
    }

    #[test]
    pub fn everything_no_end() {
        println!("{:?}", parse_everything("if 1 do ᚜1 unless 1 than ᚜1 2 else ᚜1 2 otherwise ᚜if 1 do ᚜1 2 otherwise ᚜until 1 then ᚜1 2"));
    }

    // #[test]
    // pub fn everything_if() {
    //     let test_result = parse_everything("if ? do ᚜2 6 6᚛  otherwise ᚜4᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         EverythingExpr::If(Box::new(If::new(
    //             EverythingExpr::Bool(Boolean::Maybee),
    //             vec![
    //                 EverythingExpr::Number(2.0),
    //                 EverythingExpr::Number(6.0),
    //                 EverythingExpr::Number(6.0)
    //             ],
    //             vec![EverythingExpr::Number(4.0)]
    //         )))
    //     );
    // }

    // #[test]
    // fn everything_unless() {
    //     let test_result = parse_everything("unless & than ᚜4᚛ else ᚜.t.᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         EverythingExpr::Unless(Box::new(Unless::new(
    //             EverythingExpr::Bool(Boolean::True),
    //             vec![EverythingExpr::Number(4.0)],
    //             vec![EverythingExpr::String("t".into())]
    //         )))
    //     );
    // }

    // #[test]
    // fn everything_until() {
    //     let test_result = parse_everything("until | then ᚜ ab/᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         EverythingExpr::Until(Box::new(Until::new(
    //             EverythingExpr::Bool(Boolean::False),
    //             vec!["ab/".into()]
    //         )))
    //     );
    // }

    // #[test]
    // fn everything_go_through() {
    //     let test_result = parse_everything("go-through a of (tree 5 6 7)< ᚜ .ab/.᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         EverythingExpr::GoThrough(Box::new(GoThrough::new(
    //             "a".into(),
    //             EverythingExpr::Application((vec![
    //                 "tree".into(),
    //                 EverythingExpr::Number(5.0),
    //                 EverythingExpr::Number(6.0),
    //                 EverythingExpr::Number(7.0)
    //             ],)),
    //             vec![EverythingExpr::String("ab/".into())]
    //         )))
    //     );
    // }

    // #[test]
    // fn everything_continue_doing() {
    //     let test_result = parse_everything("continue-doing ᚜ lg` ᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         EverythingExpr::ContiueDoing(vec!["lg`".into()])
    //     );
    // }

    // #[test]
    // fn everything_fn() {
    //     let test_result = parse_everything("fanction 🚗  1 * ᚜ l ᚛");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         EverythingExpr::Fanction(Fanction::new(
    //             Some('🚗'),
    //             1,
    //             Some(Varidiac::AtLeast0),
    //             vec!["l".into()]
    //         ))
    //     );
    // }

    #[test]
    fn everything_ident() {
        let test_result = parse_everything("a===a");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), "a===a".into());
    }

    #[test]
    fn everything_number() {
        let test_result = parse_everything("0xf%9");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), Ast::Number(15.5625));
    }

    #[test]
    fn everything_bool() {
        let test_result = parse_everything("?");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), Ast::Boolean(Boolean::Maybe));
    }

    // #[test]
    // fn everything_application() {
    //     let test_result = parse_everything("{mul 5 0x10 ]>>");
    //     assert!(test_result.is_ok());
    //     assert_eq!(
    //         test_result.unwrap(),
    //         EverythingExpr::Application((vec![
    //             "mul".into(),
    //             EverythingExpr::Number(5.0),
    //             EverythingExpr::Number(16.0)
    //         ],))
    //     );
    // }

    #[test]
    fn everything_acces_param() {
        // TODO: param form
        let test_result = parse_everything("'10'");
        assert!(test_result.is_ok());
        assert_eq!(
            test_result.unwrap(),
            list!("param".into(), Ast::Number(8.0))
        );
    }

    #[test]
    fn everything_with_comment() {
        let test_result = parse_everything("!t\n (1!aaa\n 22 6 ]>");
        assert!(test_result.is_ok());
    }
}
