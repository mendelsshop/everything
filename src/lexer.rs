#![allow(dead_code)]

use itertools::chain as vec_splat;
use parse_int::parse;
use std::iter;
// chars on us keyboard not used: `, , \,/,,,=
// qussiquote -> :
// unquote -> $
use crate::{
    ast::{Boolean, EverythingExpr},
    interior_mut::RC,
    pc::{
        alt, any_of, chain, char, choice, inbetween, keep_left, keep_right, many, many1, map,
        not_any_of, not_char, opt, satify, seq, string, try_map, ParseError, ParseErrorType,
        Parser,
    },
};

fn ws_or_comment() -> Box<Parser<Option<Box<dyn Iterator<Item = char>>>>> {
    map(
        many(alt(
            keep_right(char('!'), keep_left(many(not_char('\n')), opt(char('\n')))),
            map(any_of([' ', '\n', '\t']), |i| Some(opaquify(iter::once(i)))),
        )),
        |r| -> Option<Box<dyn Iterator<Item = char>>> {
            r.map(|r| opaquify(r.flatten().flatten()))
        },
    )
}

fn opaquify(f: impl Iterator<Item = char> + 'static) -> Box<dyn Iterator<Item = char>> {
    Box::new(f)
}
fn scope_list(p: Box<Parser<EverythingExpr>>) -> Box<Parser<Vec<EverythingExpr>>> {
    inbetween(
        keep_right(ws_or_comment(), char('᚜')),
        map(many(p), |r| r.map_or_else(Vec::new, Iterator::collect)),
        opt(keep_right(ws_or_comment(), char('᚛'))),
    )
}
fn scope(p: Box<Parser<EverythingExpr>>) -> Box<Parser<EverythingExpr>> {
    map(scope_list(p), |mut scope| {
        scope.insert(0, "begin".into());
        EverythingExpr::Application(scope)
    })
}
fn everythingexpr() -> Box<Parser<EverythingExpr>> {
    // needs to be its own new closure so that we don't have infinite recursion while creating the parser (so we add a level of indirection)
    map(
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
        |mut r| {
            if let Some(accesors) = r.1 {
                let new_acces =
                    |accesor: String, expr| EverythingExpr::Application(vec![accesor.into(), expr]);
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

fn application() -> Box<Parser<EverythingExpr>> {
    map(
        inbetween(
            keep_right(ws_or_comment(), any_of(call_start().iter().copied())),
            many(everythingexpr()),
            opt(keep_right(
                ws_or_comment(),
                any_of(call_end().iter().copied()),
            )),
        ),
        |r| EverythingExpr::Application(r.map_or_else(Vec::new, Iterator::collect)),
    )
}

pub fn parse_everything(input: &str) -> Result<EverythingExpr, ParseError<'_>> {
    everythingexpr()(input).map(|res| res.0)
}

pub fn everything_parse(input: &str) -> Result<Vec<EverythingExpr>, ParseError<'_>> {
    map(many(everythingexpr()), |r| r.map_or(vec![], Iterator::collect))(input).map(|res| res.0)
}

fn literal() -> Box<Parser<EverythingExpr>> {
    choice([boolean(), hexnumber(), stringdot()].to_vec())
}

fn boolean() -> Box<Parser<EverythingExpr>> {
    choice(vec![
        map(string("&"), |_| EverythingExpr::Bool(Boolean::True)),
        map(string("|"), |_| EverythingExpr::Bool(Boolean::False)),
        map(string("?"), |_| EverythingExpr::Bool(Boolean::Maybee)),
    ])
}

fn hexnumber() -> Box<Parser<EverythingExpr>> {
    let digit = any_of(['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);
    let hex_digit = choice([digit.clone(), any_of(['a', 'b', 'c', 'd', 'e', 'f'])].to_vec());
    let parese_num = |digit_type: Box<Parser<char>>| {
        try_map(
            chain(
                many(digit_type.clone()),
                opt(keep_right(char('%'), many1(digit_type))),
            ),
            |r| {
                let number = match r {
                    (None, None) => {
                        return Err(ParseError {
                            kind: ParseErrorType::Other("not a digit".into()),
                            input: "",
                        })
                    }
                    (None, Some(s)) => (String::new(), s.collect()),
                    (Some(s), None) => (s.collect(), String::new()),
                    (Some(s), Some(r)) => (s.collect(), r.collect()),
                };
                Ok(EverythingExpr::Number(
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

fn stringdot() -> Box<Parser<EverythingExpr>> {
    inbetween(
        char('.'),
        map(many(alt(escape_string(), not_char('.'))), |r| {
            EverythingExpr::String(r.map_or_else(String::new, Iterator::collect).into())
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
fn escape_string() -> Box<Parser<char>> {
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

fn stmt() -> Box<Parser<EverythingExpr>> {
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

fn mod_stmt() -> Box<Parser<EverythingExpr>> {
    keep_right(
        string("mod"),
        keep_right(
            ws_or_comment(),
            map(
                chain(
                    satify(|c| c.is_ascii_alphabetic()),
                    keep_right(
                        ws_or_comment(),
                        alt(scope_list(everythingexpr()), map(stringdot(), |s| vec![s])),
                    ),
                ),
                |(name, code)| {
                    let mut module = vec!["module".into(), name.to_string().into()];
                    module.extend(code);
                    EverythingExpr::Application(module)
                },
            ),
        ),
    )
}

fn let_stmt() -> Box<Parser<EverythingExpr>> {
    map(
        keep_right(
            string("let"),
            chain(
                keep_right(ws_or_comment(), ident_everything()),
                keep_right(keep_right(ws_or_comment(), char('=')), everythingexpr()),
            ),
        ),
        |r| EverythingExpr::Application(vec!["define".into(), r.0, r.1]),
    )
}

fn method_stmt() -> Box<Parser<EverythingExpr>> {
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
                EverythingExpr::Application(vec![number, varidiac.to_string().into()])
            });
            let scope = r.1;
            EverythingExpr::Application(vec![
                name,
                EverythingExpr::Application(vec_splat!(vec!["lambda".into(), fn_info], scope).collect()),
            ])
        },
    )
}

fn class_stmt() -> Box<Parser<EverythingExpr>> {
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
                    alt(
                        method_stmt(),
                        map(ident_everything(), |r| EverythingExpr::Application(vec![r])),
                    ),
                )),
                opt(keep_right(ws_or_comment(), char('᚛'))),
            ),
        ),
        |r| {
            let name = r.0;
            let fields = r.1.map_or_else(Vec::new, Iterator::collect);
            EverythingExpr::Application(vec_splat!(vec!["class".into(), name.into()], fields).collect())
        },
    )
}

fn if_stmt() -> Box<Parser<EverythingExpr>> {
    // TODO: allow if else if
    map(
        seq(vec![
            keep_right(string("if"), everythingexpr()),
            keep_right(
                keep_right(ws_or_comment(), string("then")),
                scope(everythingexpr()),
            ),
            keep_right(
                keep_right(ws_or_comment(), string("else")),
                scope(everythingexpr()),
            ),
        ]),
        |mut r| {
            let if_ident = "if".into();
            let cond = r.next().unwrap();
            let cons = r.next().unwrap();
            let alt = r.next().unwrap();
            EverythingExpr::Application(vec![if_ident, cond, cons, alt])
        },
    )
}

fn until_stmt() -> Box<Parser<EverythingExpr>> {
    map(
        chain(
            keep_right(string("while"), everythingexpr()),
            keep_right(
                ws_or_comment(),
                keep_right(string("do"), scope_list(everythingexpr())),
            ),
        ),
        |(cond, scope)| {
            let while_ident = "while".into();
            EverythingExpr::Application(vec_splat(vec![while_ident, cond], scope).collect())
        },
    )
}

fn go_through_stmt() -> Box<Parser<EverythingExpr>> {
    map(
        chain(
            keep_right(string("for"), keep_right(ws_or_comment(), ident_everything())), // TODO: use identifier parserl, not the full blown expression parser
            chain(
                keep_right(keep_right(ws_or_comment(), string("in")), everythingexpr()),
                scope_list(everythingexpr()),
            ),
        ),
        |(name, (iter, scope))| {
            let for_ident = "for".into();
            EverythingExpr::Application(vec_splat(vec![for_ident, name, iter], scope).collect())
        },
    )
}

fn continue_doing_stmt() -> Box<Parser<EverythingExpr>> {
    map(
        chain(string("loop"), scope_list(everythingexpr())),
        |(ident, scope)| EverythingExpr::Application(vec_splat(vec![ident.into()], scope).collect()),
    )
}

fn link_stmt() -> Box<Parser<EverythingExpr>> {
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
            let mut link = vec![link_ident, goto];
            link.extend(res.1);
            EverythingExpr::Application(link)
        },
    )
}

fn fn_stmt() -> Box<Parser<EverythingExpr>> {
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
            let map_to_everything = |c: Option<char>, mapper: fn(RC<str>) -> EverythingExpr| {
                c.as_ref()
                    .map(ToString::to_string)
                    .map(Into::into)
                    .map(mapper)
            };
            let name = map_to_everything(r.0, EverythingExpr::Ident);
            // TODO: maybe if no count given then randomly choose a count
            let param_count = r.1 .0.unwrap();
            let variadic = map_to_everything(r.1 .1 .0, EverythingExpr::Ident);
            let scope = r.1 .1 .1;
            let fn_ident = "lambda".into();
            // function can either be (lambda (n *) exprs) or (lambda (n +) exprs) or (lambda n exprs) n = arg count
            let lambda = if let Some(variadic) = variadic {
                EverythingExpr::Application(
                    vec_splat!(
                        vec![
                            fn_ident,
                            EverythingExpr::Application(vec![param_count, variadic])
                        ],
                        scope
                    )
                    .collect(),
                )
            } else {
                EverythingExpr::Application(
                    vec_splat(
                        vec![fn_ident, EverythingExpr::Application(vec![param_count])],
                        scope,
                    )
                    .collect(),
                )
            };
            if let Some(name) = name {
                let fn_ident = "define".into();
                EverythingExpr::Application(vec![fn_ident, name, lambda])
            } else {
                lambda
            }
        },
    )
}

fn ident_everything() -> Box<Parser<EverythingExpr>> {
    map(ident(), Into::into)
}

fn ident() -> Box<Parser<String>> {
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

fn terminal_everything() -> Box<Parser<EverythingExpr>> {
    alt(
        map(string("skip"), |s| EverythingExpr::Application(vec![s.into()])),
        list_expr(string("stop")),
    )
}

fn special_start() -> Box<Parser<EverythingExpr>> {
    choice(vec![
        quoted_everything(),
        label_everything(),
        param_everything(),
        unquoted_everything(),
        quassi_quoted_everything(),
    ])
}

fn list_expr(first: Box<Parser<impl Into<EverythingExpr> + 'static>>) -> Box<Parser<EverythingExpr>> {
    map(
        chain(map(first, Into::into), everythingexpr()),
        |(first, expr)| EverythingExpr::Application(vec![first, expr]),
    )
}

fn quoted_everything() -> Box<Parser<EverythingExpr>> {
    list_expr(map(string(";"), |_| "quote"))
}

fn quassi_quoted_everything() -> Box<Parser<EverythingExpr>> {
    list_expr(map(string(":"), |_| "quasiquote"))
}

fn unquoted_everything() -> Box<Parser<EverythingExpr>> {
    list_expr(map(string("$"), |_| "unquote"))
}

fn label_everything() -> Box<Parser<EverythingExpr>> {
    map(keep_right(char('@'), ident()), |res| {
        EverythingExpr::Label(res.into())
    })
}

fn param_everything() -> Box<Parser<EverythingExpr>> {
    inbetween(
        any_of(['\'', '"']),
        map(
            many1(any_of(['0', '1', '2', '3', '4', '5', '6', '7'])),
            |res| EverythingExpr::FnParam(parse(&format!("0o{}", res.collect::<String>())).unwrap()),
        ),
        opt(any_of(['\'', '"'])),
    )
}

#[cfg(test)]
mod tests {
    // TODO: remake some of thests now that >> > < are valid in any expression
    use crate::lexer::{parse_everything, Boolean, EverythingExpr};

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
        assert_eq!(test_result.unwrap(), EverythingExpr::Number(15.5625));
    }

    #[test]
    fn everything_bool() {
        let test_result = parse_everything("?");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), EverythingExpr::Bool(Boolean::Maybee));
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
        let test_result = parse_everything("'10'");
        assert!(test_result.is_ok());
        assert_eq!(test_result.unwrap(), EverythingExpr::FnParam(8));
    }

    #[test]
    fn everything_with_comment() {
        let test_result = parse_everything("!t\n (1!aaa\n 22 6 ]>");
        assert!(test_result.is_ok());
    }
}
