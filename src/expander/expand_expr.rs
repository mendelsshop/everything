use crate::{
    ast::{scope::AdjustScope, syntax::Syntax, Ast, Pair, Symbol},
    list,
};

use super::{
    binding::{Binding, CompileTimeEnvoirnment},
    expand::rebuild,
    r#match::match_syntax,
    Expander,
};

impl Expander {
    pub fn add_core_forms(&mut self) {
        self.add_core_form("lambda".into(), Self::core_form_lambda);
        self.add_core_form("let-syntax".into(), Self::core_form_let_syntax);
        self.add_core_form("%app".into(), Self::core_form_app);
        self.add_core_form("quote".into(), Self::core_form_quote);
        self.add_core_form("quote-syntax".into(), Self::core_form_quote_syntax);
        self.add_core_form("link".into(), Self::core_form_link);
        self.add_core_form("if".into(), Self::core_form_if);
        //self.add_core_form("define".into(), Self::core_form_define);
        self.add_core_form("loop".into(), |_, _, _| todo!());
        self.add_core_form("set!".into(), Self::core_form_set);
        self.add_core_form("stop".into(), |_, _, _| todo!());
        self.add_core_form("skip".into(), |_, _, _| todo!());
        self.add_core_form("begin".into(), Self::core_form_begin);
        // TODO: will we need begin
    }

    //#[trace]
    fn core_form_lambda(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        let m = match_syntax(
            s.clone(),
            list!(
                "lambda".into(),
                list!("id".into(), "...".into(),),
                "body".into()
            ),
        )?;
        let sc = self.scope_creator.new_scope();
        let ids = m("id".into()).ok_or("internal error".to_string())?;
        let ids = ids.map_pair(|term, base| match term {
            Ast::Syntax(id) => {
                let id = id.add_scope(sc);
                Ok(Ast::Syntax(Box::new(id)))
            }
            Ast::TheEmptyList if base => Ok(Ast::TheEmptyList),
            _ => Err(format!(
                "{term} is not a symbol so it cannot be a parameter"
            )),
        })?;
        let body_env = ids.clone().foldl_pair(
            |term, base, env: Result<CompileTimeEnvoirnment, String>| match term {
                Ast::Syntax(ref id_syntax) => {
                    if let Ast::Symbol(id) = &id_syntax.0 {
                        let binding =
                            self.add_local_binding(Syntax(id.clone(), id_syntax.1.clone()));
                        env.map(|env| {
                            env.extend(binding.clone(), Ast::Symbol(self.variable.clone()))
                        })
                    } else {
                        Err(format!(
                            "{term} is not a symbol so it cannot be a parameter"
                        ))
                    }
                }
                Ast::TheEmptyList if base => env,
                _ => Err(format!(
                    "{term} is not a symbol so it cannot be a parameter"
                )),
            },
            Ok(env),
        )?;
        let exp_body = self.expand(
            m("body".into())
                .ok_or("internal error".to_string())?
                .add_scope(sc),
            body_env,
        )?;
        Ok(rebuild(
            s,
            list!(
                m("lambda".into()).ok_or("internal error".to_string())?,
                ids,
                exp_body
            ),
        ))
    }
    fn core_form_let_syntax(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        let m = match_syntax(
            s,
            list!(
                "let-syntax".into(),
                list!(list!("trans-id".into(), "trans-rhs".into()), "...".into()),
                "body".into()
            ),
        )?;
        let sc = self.scope_creator.new_scope();
        let trans_ids = m("trans-id".into())
            .ok_or("internal error".to_string())?
            .foldl(
                |current_id, ids| {
                    let mut ids = ids?;
                    match current_id {
                        Ast::Syntax(id) if matches!(id.0, Ast::Symbol(_)) => {
                            let id = id.add_scope(sc);
                            let id: Syntax<Symbol> = id.try_into()?;
                            ids.push(self.add_local_binding(id));

                            Ok(ids)
                        }
                        _ => Err(format!(
                            "{current_id} is not a symbol so it cannot be a parameter"
                        )),
                    }
                },
                Ok(vec![]),
            )??;
        let trans_vals = m("trans-rhs".into())
            .ok_or("internal error".to_string())?
            .foldl(
                |current_rhs, rhss: Result<Vec<Ast>, String>| {
                    rhss.and_then(|mut rhss| {
                        let current_rhs = self.eval_for_syntax_binding(current_rhs, env.clone())?;
                        rhss.push(current_rhs);
                        Ok(rhss)
                    })
                },
                Ok(vec![]),
            )??;
        let mut hash_map = env.0;
        hash_map.extend(trans_ids.into_iter().zip(trans_vals));
        let body_env = CompileTimeEnvoirnment(hash_map);
        self.expand(
            m("body".into()).ok_or("internal error")?.add_scope(sc),
            body_env,
        )
    }
    fn core_form_app(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        let m = match_syntax(
            s,
            //TODO: should app be a syntax object
            list!("%app".into(), "rator".into(), "rand".into(), "...".into()),
        )?;
        let rator = self.expand(
            m("rator".into()).ok_or("internal error".to_string())?,
            env.clone(),
        )?;
        let rand = m("rator".into())
            .ok_or("internal error".to_string())?
            .map(|rand| self.expand(rand, env.clone()))?;
        Ok(Ast::Pair(Box::new(Pair(
            m("%app".into()).ok_or("internal error")?,
            Ast::Pair(Box::new(Pair(rator, rand))),
        ))))
    }
    fn core_form_quote(&mut self, s: Ast, _env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        match_syntax(s.clone(), list!("quote".into(), "datum".into())).map(|_| s)
    }
    fn core_form_quote_syntax(
        &mut self,
        s: Ast,
        _env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        match_syntax(s.clone(), list!("quote-syntax".into(), "datum".into())).map(|_| s)
    }

    fn core_form_link(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        // TODO: verify that dest/src label(s) are actually labels (requires updating ast with
        // everything features)
        // TODO: expand recursivly?
        let m = match_syntax(
            s.clone(),
            list!(
                "link".into(),
                "dest-label".into(),
                "src-labels".into(),
                "...".into()
            ),
        )?;
        let link = m("link".into()).ok_or("internal error")?;
        let dest = m("dest".into()).ok_or("internal error".to_string())?;
        let src = m("src".into()).ok_or("internal error".to_string())?;
        Ok(Ast::Pair(Box::new(Pair(
            link,
            Ast::Pair(Box::new(Pair(dest, src))),
        ))))
    }
    fn core_form_if(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        // TODO: optional alt?
        let m = match_syntax(
            s.clone(),
            list!("if".into(), "cond".into(), "cons".into(), "alt".into()),
        )?;
        let r#if = m("if".into()).ok_or("internal error")?;
        let cond = m("cond".into())
            .ok_or("internal error".to_string())
            .and_then(|cond| self.expand(cond, env.clone()))?;
        let cons = m("cons".into())
            .ok_or("internal error".to_string())
            .and_then(|cons| self.expand(cons, env.clone()))?;
        let alt = m("alt".into())
            .ok_or("internal error".to_string())
            .and_then(|alt| self.expand(alt, env))?;
        Ok(list!(r#if, cond, cons, alt))
    }

    //fn core_form_define(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {}
    fn core_form_set(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        let m = match_syntax(s.clone(), list!("set!".into(), "id".into(), "rhs".into()))?;
        let set = m("set!".into()).ok_or("internal error")?;
        let id = m("id".into()).ok_or("internal error".to_string());
        id.clone()
            .and_then(|id| id.try_into())
            .and_then(|id| {
                self.resolve(&id)
                    .map_err(|_| format!("no binding for assignment {s}"))
            })
            .and_then(|id| {
                if let Binding::Variable(_) = id {
                    Ok(())
                } else {
                    Err(format!("no binding for assignment {s}"))
                }
            })?;

        let rhs = m("rhs".into())
            .ok_or("internal error".to_string())
            .and_then(|rhs| self.expand(rhs, env.clone()))?;
        Ok(list!(set, id?, rhs))
    }
    fn core_form_begin(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        let m = match_syntax(s.clone(), list!("begin".into(), "e".into(), "...+".into()))?;
        let begin = m("begin".into()).ok_or("internal error")?;
        let e = m("e".into())
            .ok_or("internal error".to_string())
            .and_then(|es| es.map(|e| self.expand(e, env.clone())))?;
        Ok(Ast::Pair(Box::new(Pair(begin, e))))
    }
}
