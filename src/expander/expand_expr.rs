use crate::{
    ast::{
        scope::{AdjustScope, Scope},
        syntax::Syntax,
        Ast, Pair, Symbol,
    },
    expander::{
        duplicate_check::{self, check_no_duplicate_ids},
        expand,
    },
    list,
};

use super::{
    binding::{CompileTimeBinding, CompileTimeEnvoirnment},
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

    fn make_lambda_expander(
        &mut self,
        s: Ast,
        formals: Ast,
        bodys: Ast,
        ctx: ExpandContext,
    ) -> Result<(Ast, Ast), String> {
        let sc = self.scope_creator.new_scope();
        let ids = self.parse_and_flatten_formals(formals.clone(), sc.clone())?;
        check_no_duplicate_ids(
            ids.clone(),
            s.clone(),
            duplicate_check::make_check_no_duplicate_table(),
        )?;
        let variable = Ast::Symbol(self.variable.clone());
        let keys = ids.into_iter().map(|id| self.add_local_binding(id));
        let mut body_ctx = ctx;
        body_ctx
            .env
            .0
            .extend(keys.map(|key| (key, variable.clone())));
        let exp_body = self.expand_body(bodys, sc.clone(), s, body_ctx)?;
        Ok((formals.add_scope(sc), exp_body))
    }
    //#[trace]
    fn core_form_lambda(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = match_syntax(
            s.clone(),
            list!(
                "lambda".into(),
                "formals".into(),
                "body".into(),
                "...+".into()
            ),
        )?;
        let (formals, body) = self.make_lambda_expander(
            s.clone(),
            m("formals".into()).ok_or("internal error")?,
            m("bodys".into()).ok_or("internal error")?,
            ctx,
        )?;
        Ok(rebuild(
            s,
            list!(
                m("lambda".into()).ok_or("internal error".to_string())?,
                formals,
                body
            ),
        ))
    }
    fn core_form_case_lambda(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = match_syntax(
            s.clone(),
            list!(
                "case-lambda".into(),
                list!("formals".into(), "body".into(), "...+".into()),
                "...".into()
            ),
        )?;
        let cm = match_syntax(
            s.clone(),
            list!("case-lambda".into(), "clause".into(), "...".into()),
        )?;
        let iter = m("formals".into())
            .ok_or("internal error")?
            .to_list_checked()?
            .into_iter()
            .zip(
                m("body".into())
                    .ok_or("internal error")?
                    .to_list_checked()?,
            )
            .zip(
                cm("clause".into())
                    .ok_or("internal error")?
                    .to_list_checked()?,
            )
            .try_rfold(Ast::TheEmptyList, |rest, ((formals, bodys), clause)| {
                let (formals, body) =
                    self.make_lambda_expander(s.clone(), formals, bodys, ctx.clone())?;
                Ok::<Ast, String>(list!(rebuild(clause, list!(formals, body));rest))
            })?;
        Ok(rebuild(
            s,
            list!(m("case-lambda".into()).ok_or("internal error")?; iter),
        ))
    }
    fn parse_and_flatten_formals(
        &self,
        formals: Ast,
        sc: Scope,
    ) -> Result<Vec<Syntax<Symbol>>, String> {
        fn parse_and_flatten_formals_loop(
            formals: Ast,
            all_formals: Ast,
            sc: Scope,
            formals_list: &mut Vec<Syntax<Symbol>>,
        ) -> Result<(), String> {
            match formals {
                Ast::Syntax(s) => match s.0 {
                    Ast::Symbol(ref sym) => Ok(formals_list.push(s.clone().with(sym.clone()))),
                    Ast::TheEmptyList => Ok(()),
                    p @ Ast::Pair(_) => {
                        parse_and_flatten_formals_loop(p, all_formals, sc, formals_list)
                    }
                    p => Err(format!("not an identifier: {p}")),
                },
                Ast::TheEmptyList => Ok(()),
                Ast::Pair(p) => {
                    let Pair(car, cdr) = *p;
                    let s: Syntax<Symbol> = car
                        .clone()
                        .try_into()
                        .map_err(|_| format!("not and identifier {car}"))?;
                    let s = s.add_scope(sc.clone());
                    formals_list.push(s);
                    parse_and_flatten_formals_loop(cdr, all_formals, sc, formals_list)
                }
                _ => Err(format!("bad arguement sequence: {all_formals}")),
            }
        }
        let mut formal_list = vec![];
        parse_and_flatten_formals_loop(formals.clone(), formals, sc, &mut formal_list)
            .map(|_| formal_list)
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
