use crate::{
    ast::{
        ast1::{Ast1, Label},
        Param,
    },
    error::Error,
    matches_to,
};
use std::{mem, rc::Rc};

use matcher_proc_macro::match_syntax;

use crate::{
    ast::{syntax::Syntax, Ast, Pair, Symbol},
    evaluator::{Evaluator, Values},
};

use super::{binding::Binding, namespace::NameSpace, Expander};

impl Expander {
    // self is only used for envoirment

    pub fn compile(&self, s: Ast, ns: &NameSpace) -> Result<Ast1, Error> {
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
                            (lambda formals body)
                        )(s)?;
                        let formals = self.process_formals(m.formals)?;

                        Ok(Ast1::Lambda(formals, Box::new(compile(m.body)?)))
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
                        if let Ast::Pair(p) = m.rest {
                            let func = compile(p.0)?;
                            let app =
                                p.1.map_to_list_checked(compile)
                                    .map_err(|e| e.unwrap_or("not a list".to_string().into()))?;
                            Ok(Ast1::Application(Box::new(func), app))
                        } else {
                            Err("bad syntax after expansion compile: expexted at least one thing in an app".to_string().into())
                        }
                    }
                    "if" => {
                        let m = match_syntax!(
                            (r#if test then r#else)
                        )(s)?;
                        Ok(Ast1::If(
                            Box::new(compile(m.test)?),
                            Box::new(compile(m.then)?),
                            Box::new(compile(m.r#else)?),
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
                    "begin" => {
                        let m = match_syntax!( (begin e ..+))(s)?;
                        let stmts =
                            m.e.map_to_list_checked(compile)
                                .map_err(|e| e.unwrap_or("not a list".into()))?;
                        Ok(Ast1::Begin(stmts))
                    }
                    "begin0" => {
                        let m = match_syntax!( (begin0 e ..+))(s)?;
                        let stmts =
                            m.e.map_to_list_checked(compile)
                                .map_err(|e| e.unwrap_or("not a list".into()))?;
                        Ok(Ast1::Begin0(stmts))
                    }
                    "#%expression" => {
                        // let m = match_syntax!( (#%expression value))(s)?;
                        let m = match_syntax!( (expression value))(s)?;
                        Ok(Ast1::Expression(Box::new(compile(m.value)?)))
                    }
                    "set!" => {
                        // TODO: match_syntax!( (set! id value))
                        let m = match_syntax!( (set id value))(s)?;
                        if let Ast1::Basic(Ast::Symbol(id)) =
                            // maybe just use compile_identeifier
                            compile(m.id)?
                        {
                            Ok(Ast1::Set(id.0, Box::new(compile(m.value)?)))
                        } else {
                            Err("set requires an identifier".into())
                        }
                    }
                    "let-values" | "letrec-values" => self.compile_let(core_sym, s, ns),
                    "quote" => {
                        let m = match_syntax!( (quote datum))(s)?;
                        Ok(Ast1::Quote(m.datum.syntax_to_datum()))
                    }
                    "quote-syntax" => {
                        let m = match_syntax!((quote_syntax datum))(s)?;
                        Ok(Ast1::Quote(m.datum))
                    }
                    // TODO: will links be eliminated
                    "link" => {
                        // TODO: verify that dest/src label(s) are actually labels (requires updating ast with
                        // everything features)
                        // cases if dont quote them
                        let m = match_syntax!(
                            (
                                link
                                dest_label
                                src_labels
                                ...
                            )
                        )(s)?;
                        let filter_label = |l| {
                            matches_to!(l => Ast::Label |format!("not a label: {l}") ).map(Label)
                        };
                        let dest = m
                            .dest_label
                            .map_to_list_checked(filter_label)
                            .map_err(|e| e.unwrap_or("not a list of labels".to_string()))?;
                        let src = filter_label(m.src_labels)?;
                        Ok(Ast1::Link(src, dest))
                    }
                    "module" => todo!(),
                    "stop" => todo!(),
                    "skip" => todo!(),
                    "loop" => todo!(),
                    _ => Err(format!("unrecognized core form {core_sym}").into()),
                }
            }
            Ast::Symbol(ref s1) => Self::compile_identifier(&syntax.with_ref(s1.clone()), ns),
            _ => Err(format!("bad syntax after expansion {s} compile").into()),
        }
    }

    fn process_formals(&self, formals: Ast) -> Result<Param, Error> {
        if let Ok(m) = match_syntax!((id))(formals.clone()) {
            let id = m.id.try_into()?;
            Ok(Param::One(Self::local_symbol(&id)?.0))
        } else if let Ok(m) = match_syntax!((id variadic))(formals.clone()) {
            let id = m.id.try_into()?;
            // TODO: make sure variadic symbol is + or *
            let varidiac = m.variadic.unsyntax();
            let Ast::Symbol(varidiac) = varidiac else {
                Err(format!("invalid lambda formals variadiac {varidiac}"))?
            };
            let symbol = Self::local_symbol(&id)?.0;
            match varidiac.0.to_string().as_str() {
                "*" => Ok(Param::AtLeast0(symbol)),

                "+" => Ok(Param::AtLeast1(symbol)),
                _ => Err(format!("invalid lambda formals variadiac {varidiac}").into()),
            }
        } else if match_syntax!(())(formals).is_ok() {
            Ok(Param::Zero)
        } else {
            Err("invalid lambda formals".to_string().into())
        }
    }
    fn loop_formals(&self, formals: Ast) -> Result<Ast, Error> {
        match formals {
            Ast::Syntax(mut s) => {
                let mut a = Ast::TheEmptyList;
                mem::swap(&mut s.0, &mut a);
                match a {
                    Ast::Symbol(sym) => Self::local_symbol(&s.with(sym)).map(Ast::Symbol),
                    a @ (Ast::Pair(_) | Ast::TheEmptyList) => self.loop_formals(a),
                    formals => Err(format!("bad parameter: {formals}").into()),
                }
            }
            Ast::Pair(p) => Ok(Ast::Pair(Box::new(Pair(
                self.loop_formals(p.0)?,
                self.loop_formals(p.1)?,
            )))),
            Ast::TheEmptyList => Ok(Ast::TheEmptyList),
            _ => Err(format!("bad parameter: {formals}").into()),
        }
    }
    // fn compile_lambda(&self, formals: Ast, body: Ast, ns: &NameSpace) -> Result<Ast, Error> {
    //     Ok(list!(self.loop_formals(formals)?, self.compile(body, ns)?))
    // }

    fn compile_let(&self, core_sym: Rc<str>, s: Ast, ns: &NameSpace) -> Result<Ast1, Error> {
        let rec = &*core_sym == "letrec-values";
        let m = match_syntax!(
            (
                let_values

                    (((id ...) rhs) ...)
                    body

            )
        )(s)?;
        let idss = m.id;
        let ctor = if rec {
            Ast1::LetRecValues
        } else {
            Ast1::LetValues
        };
        Ast::map2_to_list(idss, m.rhs, |ids, rhs| {
            ids.map_to_list_checked(|id| Self::local_symbol(&id.try_into()?).map(|i| i.0))
                .map_err(|e| e.unwrap_or("not a list".into()))
                .and_then(|ids| self.compile(rhs.clone(), ns).map(|rhs| (ids, rhs)))
        })
        .and_then(|signature| {
            self.compile(m.body, ns)
                .map(|body| ctor(signature, Box::new(body)))
        })
    }
    fn local_symbol(id: &Syntax<Symbol>) -> Result<Symbol, Error> {
        let b = Self::resolve(id, false)?;
        let Binding::Local(s) = b else {
            return Err(format!("bad binding {b}").into());
        };
        Ok(key_to_symbol(s))
    }

    pub fn expand_time_eval(&self, compiled: Ast1) -> Result<Values, Error> {
        Evaluator::eval(compiled, self.expand_time_env.clone())
    }
    pub fn run_time_eval(&self, compiled: Ast1) -> Result<Values, Error> {
        Evaluator::eval(compiled, self.run_time_env.clone())
    }
    pub fn expand_time_eval_single(&self, compiled: Ast1) -> Result<Ast, Error> {
        Evaluator::eval_single_value(compiled, self.expand_time_env.clone())
    }
    pub fn run_time_eval_single(&self, compiled: Ast1) -> Result<Ast, Error> {
        Evaluator::eval_single_value(compiled, self.run_time_env.clone())
    }
    fn compile_identifier(s: &Syntax<Symbol>, ns: &NameSpace) -> Result<Ast1, Error> {
        let with = s;
        let b = Self::resolve(with, false)?;
        match b {
            Binding::Local(b) => Ok(Ast1::Basic(Ast::Symbol(key_to_symbol(b)))),
            Binding::TopLevel(s) => ns
                .variables
                .get(&s.clone().into())
                .ok_or(format!("missing core bindig for primitive {s}").into())
                .cloned()
                .map(Ast1::Basic),
        }
    }
}

const fn key_to_symbol(key: Symbol) -> Symbol {
    key
}
