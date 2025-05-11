use std::{mem, rc::Rc};

use matcher::match_syntax;

use crate::{
    ast::{syntax::Syntax, Ast, Pair, Symbol},
    evaluator::{Evaluator, Values},
    list, sexpr,
};

use super::{binding::Binding, namespace::NameSpace, Expander};

impl Expander {
    // self is only used for envoirment
    pub fn compile(&self, s: Ast, ns: &NameSpace) -> Result<Ast, String> {
        let compile = |s| self.compile(s, ns);
        let Ast::Syntax(syntax) = s.clone() else {
            panic!()
        };
        match syntax.0 {
            Ast::Pair(_) => {
                let core_sym = Self::core_form_symbol(s.clone())
                    .map_err(|_| format!("not a core form {s}"))?;
                match core_sym.to_string().as_str() {
                    "lambda" => {
                        // TODO: 0 arg lambda is currently (lambda expr) after expander
                        let m = match_syntax!(
                            (lambda id body)
                        )(s)?;
                        let id = m.id.try_into()?;
                        Ok(
                            // do we need local to symbol if its (lambda n ...) where n is a number
                            sexpr!((lambda #(Self::local_symbol(&id).map(Ast::Symbol)?))),
                        )
                    }
                    // "case-lambda" => {
                    //     let m = match_syntax!( (case_lambda (formals body) ...))(s)?;
                    //     Ast::map2(m.formals, m.body, |formals, body| {
                    //         self.compile_lambda(formals, body, ns)
                    //     })
                    //     .map(|cases| sexpr!(("case-lambda". #(cases))))
                    // }
                    "#%app" => {
                        let m = match_syntax!((app.rest))(s)?;
                        m.rest.map(compile)
                    }
                    "if" => {
                        let m = match_syntax!(
                            (r#if test then r#else)
                        )(s)?;
                        Ok(list!(
                            "if".into(),
                            compile(m.test)?,
                            compile(m.then)?,
                            compile(m.r#else)?,
                        ))
                    }

                    // "with-continuation-mark" => {
                    //     let m = match_syntax!(
                    //         // TODO: should this match with-continuation-mark as opposed to if?
                    //     (with_continuation_mark key val body)
                    //     )(s)?;
                    //     Ok(list!(
                    //         "with-continuation-mark".into(),
                    //         compile(m.key)?,
                    //         compile(m.val)?,
                    //         compile(m.body)?,
                    //     ))
                    // }
                    // maybe begin0 is if its gen-symed (at a sybmol level)
                    "begin" | "begin0" => {
                        let m = match_syntax!( (begin e ..+))(s)?;
                        m.e.map(compile)
                            .map(|e| list!(Ast::Symbol(core_sym.into()); e ))
                    }
                    "set!" => {
                        // TODO: match_syntax!( (set! id value))
                        let m = match_syntax!( (set id value))(s)?;
                        Ok(list!("set!".into(), compile(m.id)?, compile(m.value)?,))
                    }
                    "let-values" | "letrec-values" => self.compile_let(core_sym, s, ns),
                    "quote" => {
                        let m = match_syntax!( (quote datum))(s)?;
                        Ok(list!("quote".into(), m.datum.syntax_to_datum()))
                    }
                    "quote-syntax" => {
                        let m = match_syntax!((quote_syntax datum))(s)?;
                        Ok(sexpr!((quote #(m.datum))))
                    }
                    "link" => {
                        // TODO: verify that dest/src label(s) are actually labels (requires updating ast with
                        // everything features)
                        let m = match_syntax!(

                            (
                                link
                                dest_label
                                src_labels
                                ...
                            )
                        )(s)?;
                        let dest = m.dest_label;
                        let src = m.src_labels;
                        Ok(Ast::Pair(Box::new(Pair(
                            "link".into(),
                            Ast::Pair(Box::new(Pair(dest, src))),
                        ))))
                    }
                    "stop" | "skip" | "loop" => todo!(),
                    _ => Err(format!("unrecognized core form {core_sym}")),
                }
            }
            Ast::Symbol(ref s1) => {
                let with = syntax.with_ref(s1.clone());
                let b = Self::resolve(&with, false).inspect_err(|e| {
                    dbg!(format!("{e}"));
                })?;
                match b {
                    Binding::Local(b) => Ok(Ast::Symbol(key_to_symbol(b))),
                    Binding::TopLevel(s) => ns
                        .variables
                        .get(&s.clone().into())
                        .ok_or(format!("missing core bindig for primitive {s}"))
                        .cloned(),
                }
            }
            _ => Err(format!("bad syntax after expansion {s} compile")),
        }
    }
    fn loop_formals(&self, formals: Ast) -> Result<Ast, String> {
        match formals {
            Ast::Syntax(mut s) => {
                let mut a = Ast::TheEmptyList;
                mem::swap(&mut s.0, &mut a);
                match a {
                    Ast::Symbol(sym) => Self::local_symbol(&s.with(sym)).map(Ast::Symbol),
                    a @ (Ast::Pair(_) | Ast::TheEmptyList) => self.loop_formals(a),
                    formals => Err(format!("bad parameter: {formals}")),
                }
            }
            Ast::Pair(p) => Ok(Ast::Pair(Box::new(Pair(
                self.loop_formals(p.0)?,
                self.loop_formals(p.1)?,
            )))),
            Ast::TheEmptyList => Ok(Ast::TheEmptyList),
            _ => Err(format!("bad parameter: {formals}")),
        }
    }
    fn compile_lambda(&self, formals: Ast, body: Ast, ns: &NameSpace) -> Result<Ast, String> {
        Ok(list!(self.loop_formals(formals)?, self.compile(body, ns)?))
    }

    fn compile_let(&self, core_sym: Rc<str>, s: Ast, ns: &NameSpace) -> Result<Ast, String> {
        let rec = &*core_sym == "letrec-values";
        let m = match_syntax!(
            (
                let_values
                (
                    (
                        (id ... rhs)
                        ...
                    )
                    body
                )
            )
        )(s)?;
        let idss = m.id;
        Ast::map2(idss, m.rhs, |ids, rhs| {
            ids.map(|id| Self::local_symbol(&id.try_into()?).map(Ast::Symbol))
                .and_then(|ids| self.compile(rhs.clone(), ns).map(|rhs| list!(ids, rhs)))
        })
        .and_then(|signature| {
            self.compile(m.body, ns)
                .map(|body| list!(Ast::Symbol(core_sym.into()), signature, body))
        })
    }
    fn local_symbol(id: &Syntax<Symbol>) -> Result<Symbol, String> {
        let b = Self::resolve(id, false).inspect_err(|e| {
            dbg!(format!("{e}"));
        })?;
        let Binding::Local(s) = b else {
            return Err(format!("bad binding {b}"));
        };
        Ok(key_to_symbol(s))
    }

    pub fn expand_time_eval(&self, compiled: Ast) -> Result<Values, String> {
        Evaluator::eval(compiled, self.expand_time_env.clone())
    }
    pub fn run_time_eval(&self, compiled: Ast) -> Result<Values, String> {
        Evaluator::eval(compiled, self.run_time_env.clone())
    }
    pub fn expand_time_eval_single(&self, compiled: Ast) -> Result<Ast, String> {
        Evaluator::eval_single_value(compiled, self.expand_time_env.clone())
    }
    pub fn run_time_eval_single(&self, compiled: Ast) -> Result<Ast, String> {
        Evaluator::eval_single_value(compiled, self.run_time_env.clone())
    }
}

const fn key_to_symbol(key: Symbol) -> Symbol {
    key
}
