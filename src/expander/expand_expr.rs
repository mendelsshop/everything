use crate::{
    ast::{scope::AdjustScope, syntax::Syntax, Ast, Pair, Symbol},
    expander::expand,
    list,
};

use super::{
    binding::{self, CompileTimeBinding, CompileTimeEnvoirnment},
    expand::rebuild,
    expand_context::ExpandContext,
    r#match::match_syntax,
    Expander,
};

impl Expander {
    pub fn add_core_forms(&mut self) {
        self.add_core_form("lambda".into(), Self::core_form_lambda);
        self.add_core_form("case-lambda".into(), Self::core_form_case_lambda);
        self.add_core_form("let-values".into(), Self::core_form_let_values);
        self.add_core_form("letrec-values".into(), Self::core_form_letrec_values);
        self.add_core_form(
            "letrec-syntaxes+values".into(),
            Self::core_form_letrec_syntaxes_and_values,
        );
        self.add_core_form("#%datum".into(), Self::core_form_datum);
        self.add_core_form("#%app".into(), Self::core_form_app);
        self.add_core_form("quote".into(), Self::core_form_quote);
        self.add_core_form("quote-syntax".into(), Self::core_form_quote_syntax);
        self.add_core_form("if".into(), Self::core_form_if);
        self.add_core_form(
            "with-continuation-mark".into(),
            Self::core_form_with_continuation_mark,
        );
        self.add_core_form("begin".into(), Self::core_form_begin);
        self.add_core_form("begin0".into(), Self::core_form_begin0);
        self.add_core_form("set!".into(), Self::core_form_set);
    }

    //#[trace]
    fn core_form_lambda(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
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
                let id = id.add_scope(sc.clone());
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
                        let binding = self.add_local_binding(id_syntax.with_ref(id.clone()));
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
            Ok(ctx),
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
    fn core_form_case_lambda(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_let_values(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_letrec_values(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_letrec_syntaxes_and_values(
        &mut self,
        s: Ast,
        ctx: ExpandContext,
    ) -> Result<Ast, String> {
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
                            let id = id.add_scope(sc.clone());
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
                        let current_rhs = self.eval_for_syntax_binding(current_rhs, ctx.clone())?;
                        rhss.push(current_rhs);
                        Ok(rhss)
                    })
                },
                Ok(vec![]),
            )??;
        let mut hash_map = ctx.0;
        hash_map.extend(trans_ids.into_iter().zip(trans_vals));
        let body_env = CompileTimeEnvoirnment(hash_map);
        self.expand(
            m("body".into()).ok_or("internal error")?.add_scope(sc),
            body_env,
        )
    }
    fn core_form_datum(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_app(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = match_syntax(
            s,
            //TODO: should app be a syntax object
            list!("#%app".into(), "rator".into(), "rand".into(), "...".into()),
        )?;
        let rator = self.expand(
            m("rator".into()).ok_or("internal error".to_string())?,
            ctx.clone(),
        )?;
        let rand = m("rator".into())
            .ok_or("internal error".to_string())?
            .map(|rand| self.expand(rand, ctx.clone()))?;
        Ok(Ast::Pair(Box::new(Pair(
            m("#%app".into()).ok_or("internal error")?,
            Ast::Pair(Box::new(Pair(rator, rand))),
        ))))
    }
    fn core_form_quote(&mut self, s: Ast, _ctx: ExpandContext) -> Result<Ast, String> {
        match_syntax(s.clone(), list!("quote".into(), "datum".into())).map(|_| s)
    }
    fn core_form_quote_syntax(&mut self, s: Ast, _ctx: ExpandContext) -> Result<Ast, String> {
        match_syntax(s.clone(), list!("quote-syntax".into(), "datum".into())).map(|_| s)
    }
    fn core_form_if(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_with_continuation_mark(
        &mut self,
        s: Ast,
        ctx: ExpandContext,
    ) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_begin(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_begin0(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }
    fn core_form_set(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = match_syntax(s.clone(), list!("set!".into(), "id".into(), "rhs".into()))?;
        let id = m("id".into()).ok_or("internal error")?;
        let binding = self
            .resolve(&id.clone().try_into()?, false)
            .map_err(|_| format!("no binding for assignment: {s}"))?;
        let t = ctx.env.lookup(&binding, &ctx.namespace, &s)?;
        if !matches!(t, CompileTimeBinding::Regular(Ast::Symbol(s)) if s == self.variable) {
            return Err(format!("cannot assign to syntax: {s}"));
        }
        let set = m("set!".into()).ok_or("internal error")?;
        let rhs = m("rhs".into()).ok_or("internal error")?;
        Ok(expand::rebuild(s, list!(set, id, rhs)))
    }
}
