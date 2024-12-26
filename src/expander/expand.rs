use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

use itertools::Itertools;

use crate::{
    ast::{
        scope::{AdjustScope, Scope},
        syntax::Syntax,
        Ast, Function, Pair, Symbol,
    },
    list,
};

use super::{
    binding::{Binding, CompileTimeBinding, CompileTimeEnvoirnment},
    duplicate_check::{self, make_check_no_duplicate_table, DuplicateMap},
    expand_context::ExpandContext,
    namespace::NameSpace,
    r#match::match_syntax,
    Expander,
};

impl Expander {
    pub fn namespace_syntax_introduce<T: AdjustScope>(&self, s: T) -> T {
        s.add_scope(self.core_scope.clone())
    }
    //#[trace]
    pub fn expand(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        match s.clone() {
            Ast::Syntax(syntax) => match syntax.0 {
                Ast::Symbol(ref symbol) => {
                    self.expand_identifier(syntax.with_ref(symbol.clone()), ctx)
                }
                Ast::Pair(p) if p.0.identifier() => self.expand_id_application_form(*p, s, ctx),
                Ast::Pair(_) | Ast::TheEmptyList => self.expand_implicit("%app".into(), s, ctx),
                _ => self.expand_implicit("%datum".into(), s, ctx),
            },
            _ => self.expand_implicit("%datum".into(), s, ctx),
            // _ => Ok(rebuild(
            //     s.clone(),
            //     list!(
            //         Ast::Symbol("quote".into()).datum_to_syntax(
            //             Some(BTreeSet::from([self.core_scope.clone()])),
            //             None,
            //             None
            //         ),
            //         s
            //     ),
            // )),
        }
    }
    // constraints = s.len() > 0
    // constraints = s[0] == Ast::Syntax(Symbol)
    //#[trace]
    pub(crate) fn expand_id_application_form(
        &mut self,
        p: Pair,
        s: Ast,
        ctx: ExpandContext,
    ) -> Result<Ast, String> {
        let Ast::Syntax(ref id_syntax) = p.0 else {
            unreachable!()
        };
        let Ast::Symbol(ref id) = id_syntax.0 else {
            unreachable!();
        };
        let binding = self.resolve(&id_syntax.with_ref(id.clone()), false);
        let binding = binding.and_then(|binding| self.lookup(&binding, &ctx, id));
        match binding {
            Ok(binding) if !matches!(&binding, CompileTimeBinding::Regular(Ast::Symbol(sym)) if *sym == self.variable) => {
                self.dispatch(binding, s, ctx)
            }
            _ => self.expand_implicit("%app".into(), s, ctx),
        }
    }

    fn expand_implicit(&mut self, sym: Symbol, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let scopes = s.scope_set();
        let id = sym.clone().datum_to_syntax(scopes, None, None);
        let binding = self.resolve(&id, false);
        let transformer = binding.and_then(|binding| self.lookup(&binding, &ctx, &sym))?;
        match transformer {
            CompileTimeBinding::CoreForm(_) if ctx.only_immediate => Ok(s),
            CompileTimeBinding::Regular(Ast::Function(_)) | CompileTimeBinding::CoreForm(_) => {
                self.dispatch(transformer, s, ctx)
            }
            _ => Err(format!("no tranformer binding for {sym}")),
        }
    }

    fn lookup(
        &self,
        binding: &Binding,
        ctx: &ExpandContext,
        id: &Symbol,
    ) -> Result<CompileTimeBinding, String> {
        ctx.env.lookup(binding, &ctx.namespace, id)
    }
    pub(crate) fn apply_transformer(
        &mut self,
        m: Function,
        s: Ast,
        ctx: &ExpandContext,
    ) -> Result<Ast, String> {
        let intro_scope = self.scope_creator.new_scope();
        let intro_s = s.add_scope(intro_scope.clone());
        let uses_s = self.maybe_add_use_site_scope(intro_s, ctx);
        let transformed_s = m.apply(Ast::Pair(Box::new(Pair(uses_s, Ast::TheEmptyList))))?;
        if !matches!(transformed_s, Ast::Syntax(_)) {
            return Err(format!("transformer produced non syntax: {transformed_s}"));
        }
        let result_s = transformed_s.flip_scope(intro_scope);
        Ok(self.maybe_add_post_site_scope(result_s, ctx))
    }

    fn maybe_add_use_site_scope(&mut self, s: Ast, ctx: &ExpandContext) -> Ast {
        match &ctx.use_site_scopes {
            Some(scopes) => {
                let sc = self.scope_creator.new_scope();

                scopes.borrow_mut().insert(sc.clone());
                s.add_scope(sc)
            }
            None => s,
        }
    }
    fn maybe_add_post_site_scope(&self, s: Ast, ctx: &ExpandContext) -> Ast {
        {
            match &ctx.post_expansion_scope {
                Some(sc) => s.add_scope(sc.clone()),
                None => s,
            }
        }
    }
    fn dispatch(
        &mut self,
        t: CompileTimeBinding,
        s: Ast,
        ctx: ExpandContext,
    ) -> Result<Ast, String> {
        match t {
            CompileTimeBinding::Regular(t) => match t {
                Ast::Function(transfromer) => {
                    let apply_transformer = self.apply_transformer(transfromer, s, &ctx)?;
                    self.expand(apply_transformer, ctx)
                }
                Ast::Symbol(variable) if variable == self.variable => Ok(s),
                _ => Err(format!("illegal use of syntax: {t}")),
            },
            CompileTimeBinding::CoreForm(form) => {
                if ctx.only_immediate {
                    Ok(s)
                } else {
                    form(self, s, ctx)
                }
            }
        }
    }

    fn expand_body_loop(
        &mut self,
        mut body_ctx: ExpandContext,
        ctx: ExpandContext,
        bodys: Ast,
        done_bodys: Ast,
        val_binds: Ast,
        duplicate: DuplicateMap,
        original_syntax: Ast,
    ) -> Result<Ast, String> {
        match bodys {
            Ast::TheEmptyList => {
                self.finish_expanding_body(body_ctx, done_bodys, val_binds, original_syntax)
            }
            Ast::Pair(cons) => {
                let exp_body = self.expand(cons.0, body_ctx.clone())?;
                match self
                    .core_form_symbol(exp_body.clone())?
                    .to_string()
                    .as_str()
                {
                    "begin" => {
                        let m = match_syntax(
                            exp_body.clone(),
                            list!("begin".into(), "e".into(), "...".into()),
                        )?;
                        let e = m("e".into()).ok_or("internal error")?;
                        self.expand_body_loop(
                            body_ctx,
                            ctx,
                            e.append(cons.1),
                            done_bodys,
                            val_binds,
                            duplicate,
                            original_syntax,
                        )
                    }
                    "define-values" => todo!(),
                    "define-syntaxes" => {
                        let m = match_syntax(
                            exp_body.clone(),
                            list!(
                                "define-syntaxes".into(),
                                list!("id".into(), "...".into()),
                                "rhs".into()
                            ),
                        )?;
                        let ids = self.remove_use_site_scopes(
                            m("id".into()).ok_or("internal error")?,
                            &body_ctx,
                        );
                        let ids = ids.to_list_checked()?;

                        let id_count = ids.len();
                        let ids = ids
                            .into_iter()
                            .map(|id| id.try_into())
                            .collect::<Result<Vec<_>, _>>()?;
                        let new_duplicates = duplicate_check::check_no_duplicate_ids(
                            ids.clone(),
                            exp_body,
                            duplicate,
                        )?;
                        let keys = ids
                            .into_iter()
                            .map(|id| self.add_local_binding(id))
                            .collect_vec();

                        let vals = self.eval_for_syntaxes_binding(
                            m("rhs".into()).ok_or("internal error")?,
                            id_count,
                            ctx.clone(),
                        )?;
                        body_ctx
                            .env
                            .0
                            .extend(keys.into_iter().zip(vals.into_iter()));
                        self.expand_body_loop(
                            body_ctx,
                            ctx,
                            cons.1,
                            done_bodys,
                            val_binds,
                            new_duplicates,
                            original_syntax,
                        )
                    }
                    _ => self.expand_body_loop(
                        body_ctx,
                        ctx,
                        cons.1,
                        list!(exp_body; done_bodys),
                        val_binds,
                        duplicate,
                        original_syntax,
                    ),
                }
            }
            _ => todo!("error"),
        }
    }
    fn remove_use_site_scopes(&self, syntax: Ast, ctx: &ExpandContext) -> Ast {
        if let Some(scopes) = &ctx.use_site_scopes {
            syntax.remove_scopes(scopes.borrow().clone())
        } else {
            syntax
        }
    }
    fn finish_expanding_body(
        &self,
        body_ctx: ExpandContext,
        done_bodys: Ast,
        val_binds: Ast,
        s: Ast,
    ) -> Result<Ast, String> {
        todo!()
    }
    fn expand_body(
        &mut self,
        bodys: Ast,
        scope: Scope,
        original_syntax: Ast,
        // we need to use parts of the context to create a new context, so &mut context, might not
        // be the greatest idea here (maybe rc refcelL?)
        context: ExpandContext,
    ) -> Result<Ast, String> {
        let outside_scope = self.scope_creator.new_scope();
        let inside_scope = self.scope_creator.new_scope();
        let init_bodys = bodys.map(|body| {
            Ok(body
                .add_scope(scope.clone())
                .add_scope(outside_scope.clone())
                .add_scope(inside_scope.clone()))
        })?;
        let body_context = ExpandContext {
            use_site_scopes: Some(Rc::new(RefCell::new(BTreeSet::new()))),
            only_immediate: true,
            post_expansion_scope: Some(inside_scope),
            ..context.clone()
        };
        self.expand_body_loop(
            body_context,
            context,
            init_bodys,
            Ast::TheEmptyList,
            Ast::TheEmptyList,
            make_check_no_duplicate_table(),
            original_syntax,
        )
    }
    fn exxpand_and_eval_for_syntaxes_binding(
        &mut self,
        rhs: Ast,
        id_count: usize,
        ctx: ExpandContext,
    ) -> Result<(Vec<Ast>, Ast), String> {
        let exp_rhs = self.expand_transformer(rhs, ctx.clone())?;
        Ok((
            self.eval_for_bindings(exp_rhs.clone(), id_count, ctx.namespace)?,
            exp_rhs,
        ))
    }
    pub fn eval_for_syntaxes_binding(
        &mut self,
        rhs: Ast,
        id_count: usize,
        ctx: ExpandContext,
    ) -> Result<Vec<Ast>, String> {
        self.exxpand_and_eval_for_syntaxes_binding(rhs, id_count, ctx)
            .map(|x| x.0)
        // // let var_name = format!("problem `evaluating` macro {rhs}");
        // let expand = self.expand_transformer(rhs, ctx)?;
        // let compile = self.compile(expand)?;
        // self.expand_time_eval(compile)
    }

    fn eval_for_bindings(
        &self,
        exp_rhs: Ast,
        id_count: usize,
        namespace: NameSpace,
    ) -> Result<Vec<Ast>, String> {
        let compiled = self.compile(exp_rhs.clone(), &namespace)?;
        // TODO: some notion of values https://docs.racket-lang.org/reference/values.html
        // currently just converting to vec
        compiled
            .to_list_checked()?
            .into_iter()
            .map(|compiled| self.expand_time_eval(list!("#%expression".into(), compiled)))
            .try_collect()
            .and_then(|list: Vec<_>| {
                if id_count != list.len() {
                    Err(format!(
                        "wrong number of results ({} vs {id_count}) from {exp_rhs}",
                        list.len()
                    ))
                } else {
                    Ok(list)
                }
            })
    }
    fn expand_transformer(&mut self, rhs: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        self.expand(
            rhs,
            ExpandContext {
                env: CompileTimeEnvoirnment::new(),
                only_immediate: false,
                post_expansion_scope: None,
                ..ctx
            },
        )
    }

    // pub(crate) fn expand_app(
    //     &mut self,
    //     s: Ast,
    //     env: CompileTimeEnvoirnment,
    // ) -> Result<Ast, String> {
    //     let m = match_syntax(
    //         s.clone(),
    //         list!("rator".into(), "rand".into(), "...".into()),
    //     )?;
    //     let rator = m("rator".into()).ok_or("internal error".to_string())?;
    //     let rand = m("rand".into())
    //         .ok_or("internal error".to_string())?
    //         .map(|rand| self.expand(rand, env.clone()))?;
    //     Ok(rebuild(
    //         s,
    //         Ast::Pair(Box::new(Pair(
    //             Into::<Ast>::into("%app").datum_to_syntax(
    //                 Some(BTreeSet::from([self.core_scope.clone()])),
    //                 None,
    //                 None,
    //             ),
    //             Ast::Pair(Box::new(Pair(self.expand(rator, env)?, rand))),
    //         ))),
    //     ))
    // }
    pub(crate) fn expand_identifier(
        &mut self,
        s: Syntax<Symbol>,
        ctx: ExpandContext,
    ) -> Result<Ast, String> {
        let binding = self.resolve(&s, false);
        let id = s.0.clone();
        let s = Ast::Syntax(Box::new(s.with(Ast::Symbol(id.clone()))));
        match binding {
            Ok(binding) => self.dispatch(self.lookup(&binding, &ctx, &id)?, s, ctx),
            _ => self.expand_implicit("%top".into(), s, ctx),
        }
    }
}

pub fn rebuild(s: Ast, rator: Ast) -> Ast {
    rator.datum_to_syntax(s.scope_set(), s.syntax_src_loc(), s.properties())
}
