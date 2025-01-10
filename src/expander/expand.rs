use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

use itertools::Itertools;

use crate::{
    ast::{
        scope::{AdjustScope, Scope},
        syntax::Syntax,
        Ast, Function, Pair, Symbol,
    },
    evaluator::Values,
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
                Ast::Pair(_) | Ast::TheEmptyList => self.expand_implicit("#%app".into(), s, ctx),
                _ => self.expand_implicit("#%datum".into(), s, ctx),
            },
            _ => self.expand_implicit("#%datum".into(), s, ctx),
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
            _ => self.expand_implicit("#%app".into(), s, ctx),
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
        let transformed_s = m.apply_single(Ast::Pair(Box::new(Pair(uses_s, Ast::TheEmptyList))))?;
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

    // unlike the racket version done_bodys and val_binds are in the correct order, as it is
    // probably more effecient from a rust perspective anyway
    fn expand_body_loop(
        &mut self,
        mut body_ctx: ExpandContext,
        ctx: ExpandContext,
        mut bodys: impl Iterator<Item = Ast>,
        mut done_bodys: Vec<Ast>,
        mut val_binds: Vec<(Vec<Syntax<Symbol>>, Ast)>,
        duplicate: DuplicateMap,
        original_syntax: Ast,
    ) -> Result<Ast, String> {
        match bodys.next() {
            None => self.finish_expanding_body(body_ctx, done_bodys, val_binds, original_syntax),
            Some(body) => {
                let exp_body = self.expand(body, body_ctx.clone())?;
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
                            e.to_list_checked()?.into_iter().chain(bodys),
                            done_bodys,
                            val_binds,
                            duplicate,
                            original_syntax,
                        )
                    }
                    "define-values" => {
                        let m = match_syntax(
                            exp_body.clone(),
                            list!(
                                "define-values".into(),
                                list!("id".into(), "...".into()),
                                "rhs".into()
                            ),
                        )?;
                        let ids = self.remove_use_site_scopes(
                            m("id".into()).ok_or("internal error")?,
                            &body_ctx,
                        );
                        let ids = to_id_list(ids)?;
                        let new_duplicates = duplicate_check::check_no_duplicate_ids(
                            ids.clone(),
                            exp_body,
                            duplicate,
                        )?;
                        let keys = ids
                            .clone()
                            .into_iter()
                            .map(|id| self.add_local_binding(id))
                            .collect_vec();

                        body_ctx.env.0.extend(
                            keys.into_iter()
                                .map(|key| (key, Ast::Symbol(self.variable.clone()))),
                        );

                        val_binds.append(&mut self.no_binds(done_bodys));
                        val_binds.push((ids, m("rhs".into()).ok_or("internal error")?));
                        self.expand_body_loop(
                            body_ctx,
                            ctx,
                            bodys,
                            vec![],
                            val_binds,
                            new_duplicates,
                            original_syntax,
                        )
                    }
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
                            bodys,
                            done_bodys,
                            val_binds,
                            new_duplicates,
                            original_syntax,
                        )
                    }
                    _ => {
                        done_bodys.push(exp_body);
                        self.expand_body_loop(
                            body_ctx,
                            ctx,
                            bodys,
                            done_bodys,
                            val_binds,
                            duplicate,
                            original_syntax,
                        )
                    }
                }
            }
        }
    }
    fn core_datum_to_syntax(&self, expr: Ast) -> Ast {
        expr.datum_to_syntax(
            Some(self.core_syntax.1.clone()),
            Some(self.core_syntax.2.clone()),
            Some(self.core_syntax.3.clone()),
        )
    }
    fn remove_use_site_scopes(&self, syntax: Ast, ctx: &ExpandContext) -> Ast {
        if let Some(scopes) = &ctx.use_site_scopes {
            syntax.remove_scopes(scopes.borrow().clone())
        } else {
            syntax
        }
    }
    fn finish_expanding_body(
        &mut self,
        body_ctx: ExpandContext,
        mut done_bodys: Vec<Ast>,
        val_binds: Vec<(Vec<Syntax<Symbol>>, Ast)>,
        s: Ast,
    ) -> Result<Ast, String> {
        if done_bodys.is_empty() {
            return Err(format!(
                "begin (possibly implicit): the last form is not an expression {s}"
            ));
        }
        let finish_ctx = ExpandContext {
            use_site_scopes: None,
            only_immediate: false,
            post_expansion_scope: None,
            ..body_ctx
        };
        let finish_bodys = if done_bodys.len() == 1 {
            self.expand(done_bodys.remove(0), finish_ctx.clone())?
        } else {
            list!(
                self.core_datum_to_syntax("begin".into());
                done_bodys
                    .into_iter()
                    .try_rfold(Ast::TheEmptyList, |exprs, expr| {
                        self.expand(expr, finish_ctx.clone())
                            .map(|expr| list!(expr; exprs))
                })?)
            .datum_to_syntax(None, None, None)
        };
        if val_binds.is_empty() {
            Ok(finish_bodys)
        } else {
            Ok(list!(
                self.core_datum_to_syntax("letrec-values".into()),
                val_binds
                    .into_iter()
                    .try_rfold(Ast::TheEmptyList, |exprs, (ids, values)| {
                        self.expand(values, finish_ctx.clone())
                            .map(|expr| list!(list!(
                                ids.into_iter().rfold(Ast::TheEmptyList, |ids, id|
                                    list!(
                                        Ast::Syntax(Box::new(id.clone().with(Ast::Symbol(id.0))));
                                        ids)
                                ).datum_to_syntax(None, None, None),
                                expr); exprs))
                    })?;
                finish_bodys
            )
            .datum_to_syntax(None, None, None))
        }
    }
    fn no_binds(&self, done_bodys: Vec<Ast>) -> Vec<(Vec<Syntax<Symbol>>, Ast)> {
        done_bodys
            .into_iter()
            .map(|body| {
                (
                    vec![],
                    list!(
                        self.core_datum_to_syntax("begin".into()),
                        body,
                        list!(
                            self.core_datum_to_syntax("#%app".into()),
                            self.core_datum_to_syntax("values".into())
                        )
                    ),
                )
            })
            .collect_vec()
    }
    pub fn expand_body(
        &mut self,
        bodys: Ast,
        scope: Scope,
        original_syntax: Ast,
        context: ExpandContext,
    ) -> Result<Ast, String> {
        let outside_scope = self.scope_creator.new_scope();
        let inside_scope = self.scope_creator.new_scope();
        let init_bodys = bodys
            .map(|body| {
                Ok(body
                    .add_scope(scope.clone())
                    .add_scope(outside_scope.clone())
                    .add_scope(inside_scope.clone()))
            })?
            .to_list_checked()?;
        let body_context = ExpandContext {
            use_site_scopes: Some(Rc::new(RefCell::new(BTreeSet::new()))),
            only_immediate: true,
            post_expansion_scope: Some(inside_scope),
            ..context.clone()
        };
        self.expand_body_loop(
            body_context,
            context,
            init_bodys.into_iter(),
            vec![],
            vec![],
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
    }

    fn eval_for_bindings(
        &self,
        exp_rhs: Ast,
        id_count: usize,
        namespace: NameSpace,
    ) -> Result<Vec<Ast>, String> {
        let compiled = self.compile(exp_rhs.clone(), &namespace)?;
        self.expand_time_eval(list!("#%expression".into(), compiled))
            .and_then(|values| {
                let list = match values {
                    Values::Many(vec) => vec,
                    Values::Single(ast) => vec![ast],
                };
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

pub fn to_id_list(ids: Ast) -> Result<Vec<Syntax<Symbol>>, String> {
    let ids = ids.to_list_checked()?;
    let ids = ids
        .into_iter()
        .map(|id| id.try_into())
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ids)
}

pub fn rebuild(s: Ast, rator: Ast) -> Ast {
    rator.datum_to_syntax(s.scope_set(), s.syntax_src_loc(), s.properties())
}
