use crate::{
    ast::{syntax::Syntax, Ast, Pair, Symbol},
    evaluator::Evaluator,
    list,
};

use super::{binding::Binding, r#match::match_syntax, Expander};

impl Expander {
    pub fn compile(&mut self, s: Ast) -> Result<Ast, String> {
        let Ast::Syntax(syntax) = s.clone() else {
            panic!()
        };
        match syntax.0 {
            Ast::Pair(_) => {
                let core_sym = self
                    .core_form_symbol(s.clone())
                    .map_err(|_| format!("not a core form {s}"))?;
                match core_sym.to_string().as_str() {
                    "lambda" => {
                        // TODO: 0 arg lambda is currently (lambda expr) after expander
                        let m =
                            match_syntax(s, list!("lambda".into(), "id".into(), "body".into()))?;
                        let id = m("id".into()).ok_or("internal error")?;
                        let body = m("body".into()).ok_or("internal error")?;
                        Ok(list!(
                            "lambda".into(),
                            self.local_symbol(id).map(Ast::Symbol)?,
                            self.compile(body)?
                        ))
                    }
                    "%app" => {
                        let m = match_syntax(
                            s,
                            // TODO: should this (%app) be a syntax object
                            // TODO FIX: the problem seems to be that rest is not a list
                            Ast::Pair(Box::new(Pair("%app".into(), "rest".into()))),
                        )?;
                        let l = m("rest".into()).ok_or("internal error")?.list();
                        m("rest".into())
                            .ok_or("internal error")?
                            .map(|s| self.compile(s))
                    }
                    "quote" => {
                        let m = match_syntax(s, list!("quote".into(), "datum".into()))?;
                        m("datum".into())
                            .ok_or("internal error".to_string())
                            .map(Ast::syntax_to_datum)
                            .map(|datum| list!("quote".into(), datum))
                    }
                    "quote-syntax" => {
                        let m = match_syntax(s, list!("quote-syntax".into(), "datum".into()))?;
                        m("datum".into())
                            .ok_or("internal error".to_string())
                            .map(|datum| list!("quote".into(), datum))
                    }
                    "set!" => {
                        let m = match_syntax(
                            s,
                            list!("set!".into(), "id".into(), "rhs".into()),
                        )?;
                        let id = m("id".into())
                            .ok_or("internal error".to_string())
                            .and_then(|id| self.compile(id))?;
                        let rhs = m("rhs".into())
                            .ok_or("internal error".to_string())
                            .and_then(|rhs| self.compile(rhs))?;
                        Ok(list!("set!".into(), id, rhs))
                    }
                    "link" => {
                        // TODO: verify that dest/src label(s) are actually labels (requires updating ast with
                        // everything features)
                        let m = match_syntax(
                            s,
                            list!(
                                "link".into(),
                                "dest-label".into(),
                                "src-labels".into(),
                                "...".into()
                            ),
                        )?;
                        let dest = m("dest".into()).ok_or("internal error".to_string())?;
                        let src = m("src".into()).ok_or("internal error".to_string())?;
                        Ok(Ast::Pair(Box::new(Pair(
                            "link".into(),
                            Ast::Pair(Box::new(Pair(dest, src))),
                        ))))
                    }
                    "if" => {
                        // TODO: optional alt?
                        let m = match_syntax(
                            s,
                            list!("if".into(), "cond".into(), "cons".into(), "alt".into()),
                        )?;
                        let cond = m("cond".into())
                            .ok_or("internal error".to_string())
                            .and_then(|cond| self.compile(cond))?;
                        let cons = m("cons".into())
                            .ok_or("internal error".to_string())
                            .and_then(|cons| self.compile(cons))?;
                        let alt = m("alt".into())
                            .ok_or("internal error".to_string())
                            .and_then(|alt| self.compile(alt))?;
                        Ok(list!("if".into(), cond, cons, alt))
                    }
                    "begin" => {
                        let m = match_syntax(
                            s,
                            list!("begin".into(), "e".into(), "...+".into()),
                        )?;
                        let e = m("e".into())
                            .ok_or("internal error".to_string())
                            .and_then(|es| es.map(|e| self.compile(e)))?;
                        Ok(Ast::Pair(Box::new(Pair("begin".into(), e))))
                    }
                    //"define" => {}
                    "stop" | "skip" | "loop" => todo!(),
                    _ => Err(format!("unrecognized core form {core_sym}")),
                }
            }
            Ast::Symbol(s1) => {
                let b = self.resolve(&Syntax(s1, syntax.1))?;
                match b {
                    Binding::Variable(b) => Ok(Ast::Symbol(key_to_symbol(b.clone()))),
                    Binding::CoreBinding(s) => self
                        .core_primitives
                        .get(s)
                        .ok_or(format!("missing core bindig for primitive {s}"))
                        .cloned(),
                }
            }
            _ => Err(format!("bad syntax after expansion {s}")),
        }
    }

    fn local_symbol(&self, id: Ast) -> Result<Symbol, String> {
        let Ast::Syntax(ref s) = id else {
            return Err(format!("expected symbol found {id}"));
        };
        let Ast::Symbol(ref id) = s.0 else {
            return Err(format!("expected symbol found {id}"));
        };
        let b = self.resolve(&Syntax(id.clone(), s.1.clone()))?;
        let Binding::Variable(s) = b else {
            return Err(format!("bad binding {b}"));
        };
        Ok(key_to_symbol(s.clone()))
    }

    pub fn expand_time_eval(&self, compiled: Ast) -> Result<Ast, String> {
        Evaluator::eval(compiled, self.expand_env.clone())
    }
    pub fn run_time_eval(&self, compiled: Ast) -> Result<Ast, String> {
        Evaluator::eval(compiled, self.run_time_env.clone())
    }
}

const fn key_to_symbol(key: Symbol) -> Symbol {
    key
}
