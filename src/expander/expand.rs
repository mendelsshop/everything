use crate::{
    ast::ast1::Ast1,
    error::{
        BeginNonExpression, Error, IllegalUseOfSyntax, MissingTransformer, NonSyntaxTransformer,
    },
};
use std::{
    cell::RefCell,
    collections::{BTreeSet, VecDeque},
    rc::Rc,
};

use itertools::Itertools;
use matcher_proc_macro::match_syntax;

use crate::{
    ast::{
        scope::{AdjustScope, Scope},
        syntax::Syntax,
        Ast, Function, Pair, Symbol,
    },
    evaluator::Values,
    list, UniqueNumberManager,
};

use super::{
    binding::{Binding, CompileTimeBinding, CompileTimeEnvoirnment},
    duplicate_check::{self, make_check_no_duplicate_table, DuplicateMap},
    expand_context::ExpandContext,
    namespace::NameSpace,
    Expander,
};

impl Expander {
    pub fn namespace_syntax_introduce<T: AdjustScope>(&self, s: T) -> T {
        s.add_scope(self.core_scope.clone())
    }
    pub fn expand(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, Error> {
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
    pub(crate) fn expand_id_application_form(
        &mut self,
        p: Pair,
        s: Ast,
        ctx: ExpandContext,
    ) -> Result<Ast, Error> {
        let Ast::Syntax(ref id_syntax) = p.0 else {
            unreachable!()
        };
        let Ast::Symbol(ref id) = id_syntax.0 else {
            unreachable!();
        };
        let binding = Self::resolve(&id_syntax.with_ref(id.clone()), false);
        let binding = binding.and_then(|binding| self.lookup(&binding, &ctx, id));
        match binding {
            Ok(binding) if !matches!(&binding, CompileTimeBinding::Regular(Ast::Symbol(sym)) if *sym == self.variable) => {
                self.dispatch(binding, s, ctx)
            }
            _ => self.expand_implicit("#%app".into(), s, ctx),
        }
    }

    fn expand_implicit(&mut self, sym: Symbol, s: Ast, ctx: ExpandContext) -> Result<Ast, Error> {
        let scopes = s.scope_set();
        let id = sym.clone().datum_to_syntax(scopes, None, None);
        let binding = Self::resolve(&id, false);
        let transformer = binding.and_then(|binding| self.lookup(&binding, &ctx, &sym))?;
        match transformer {
            CompileTimeBinding::CoreForm(_) if ctx.only_immediate => Ok(s),
            CompileTimeBinding::Regular(Ast::Function(_)) | CompileTimeBinding::CoreForm(_) => {
                let scope_set = s.scope_set();
                let syntax_src_loc = s.syntax_src_loc();
                self.dispatch(
                    transformer,
                    list!(Ast::Symbol(sym); s).datum_to_syntax(scope_set, syntax_src_loc, None),
                    ctx,
                )
            }
            _ => Err(Error::MissingTransformer(MissingTransformer(sym))),
        }
    }

    fn lookup(
        &self,
        binding: &Binding,
        ctx: &ExpandContext,
        id: &Symbol,
    ) -> Result<CompileTimeBinding, Error> {
        ctx.env
            .lookup(binding, &ctx.namespace, id, self.variable.clone())
            .map_err(Error::OutOfContext)
    }
    pub(crate) fn apply_transformer(
        m: Function,
        s: Ast,
        ctx: &ExpandContext,
    ) -> Result<Ast, Error> {
        let intro_scope = UniqueNumberManager::new_scope();
        let intro_s = s.add_scope(intro_scope.clone());
        let uses_s = Self::maybe_add_use_site_scope(intro_s, ctx);
        let transformed_s = m.apply_single(Ast::Pair(Box::new(Pair(uses_s, Ast::TheEmptyList))))?;
        if !matches!(transformed_s, Ast::Syntax(_)) {
            return Err(NonSyntaxTransformer(transformed_s).into());
        }
        let result_s = transformed_s.flip_scope(intro_scope);
        Ok(Self::maybe_add_post_site_scope(result_s, ctx))
    }

    fn maybe_add_use_site_scope(s: Ast, ctx: &ExpandContext) -> Ast {
        match &ctx.use_site_scopes {
            Some(scopes) => {
                let sc = UniqueNumberManager::new_scope();

                scopes.borrow_mut().insert(sc.clone());
                s.add_scope(sc)
            }
            None => s,
        }
    }
    fn maybe_add_post_site_scope(s: Ast, ctx: &ExpandContext) -> Ast {
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
    ) -> Result<Ast, Error> {
        match t {
            CompileTimeBinding::Regular(t) => match t {
                Ast::Function(transfromer) => {
                    let apply_transformer = Self::apply_transformer(transfromer, s, &ctx)?;
                    self.expand(apply_transformer, ctx)
                }
                Ast::Symbol(variable) if variable == self.variable => Ok(s),
                _ => Err(Error::IllegalUseOfSyntax(IllegalUseOfSyntax(t))),
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
        mut bodys: VecDeque<Ast>,
        mut done_bodys: Vec<Ast>,
        mut val_binds: Vec<(Vec<Syntax<Symbol>>, Ast)>,
        duplicate: DuplicateMap,
        original_syntax: Ast,
    ) -> Result<Ast, Error> {
        // let mut bodys = bodys.into_iter();
        match bodys.pop_front() {
            None => self.finish_expanding_body(body_ctx, done_bodys, val_binds, original_syntax),
            Some(body) => {
                let exp_body = self.expand(body, body_ctx.clone())?;
                if let Ok(pat) = Self::core_form_symbol(exp_body.clone()) {
                    match pat.to_string().as_str() {
                        "begin" => {
                            let m = match_syntax!(
                                (begin e ...)
                            )(exp_body)?;
                            let e = m.e;
                            let mut new_bodys = VecDeque::from(e.to_list_checked::<Error>()?);

                            new_bodys.append(&mut bodys);
                            self.expand_body_loop(
                                body_ctx,
                                ctx,
                                new_bodys,
                                done_bodys,
                                val_binds,
                                duplicate,
                                original_syntax,
                            )
                        }
                        "define-values" => {
                            let m = match_syntax!(
                                (
                                    define_values
                                    (id ...)
                                    rhs
                                )
                            )(exp_body.clone())?;
                            let ids = Self::remove_use_site_scopes(m.id, &body_ctx);
                            let ids = to_id_list(ids)?;
                            let new_duplicates = duplicate_check::check_no_duplicate_ids(
                                ids.clone(),
                                &exp_body,
                                duplicate,
                            )?;
                            let keys = ids
                                .clone()
                                .into_iter()
                                .map(Self::add_local_binding)
                                .collect_vec();

                            body_ctx.env.0.extend(
                                keys.into_iter()
                                    .map(|key| (key, Ast::Symbol(self.variable.clone()))),
                            );

                            val_binds.append(&mut self.no_binds(done_bodys));
                            val_binds.push((ids, m.rhs));
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
                            let m = match_syntax!((
                                define_syntaxes
                                (id ...)
                                rhs
                            ))(exp_body.clone())?;
                            let ids = Self::remove_use_site_scopes(m.id, &body_ctx);
                            let ids = ids.to_list_checked::<Error>()?;

                            let id_count = ids.len();
                            let ids = ids
                                .into_iter()
                                .map(std::convert::TryInto::try_into)
                                .collect::<Result<Vec<_>, _>>()?;
                            let new_duplicates = duplicate_check::check_no_duplicate_ids(
                                ids.clone(),
                                &exp_body,
                                duplicate,
                            )?;
                            let keys = ids.into_iter().map(Self::add_local_binding).collect_vec();
                            let vals =
                                self.eval_for_syntaxes_binding(m.rhs, id_count, ctx.clone())?;
                            body_ctx.env.0.extend(keys.into_iter().zip(vals));
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
                } else {
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
    pub fn core_datum_to_syntax(&self, expr: Ast) -> Ast {
        expr.datum_to_syntax(
            Some(self.core_syntax.1.clone()),
            Some(self.core_syntax.2.clone()),
            Some(self.core_syntax.3.clone()),
        )
    }
    fn remove_use_site_scopes(syntax: Ast, ctx: &ExpandContext) -> Ast {
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
    ) -> Result<Ast, Error> {
        if done_bodys.is_empty() {
            return Err(Error::BeginNonExpression(BeginNonExpression(s)));
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
                        self.expand(values, finish_ctx.clone()).map(|expr| {
                            list!(list!(
                                ids.into_iter().rfold(Ast::TheEmptyList, |ids, id|
                                    list!(
                                        Ast::Syntax(Box::new(id.clone().with(Ast::Symbol(id.0))));
                                        ids)
                                ).datum_to_syntax(None, None, None),
                                expr); exprs)
                        })
                    })?,
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
    ) -> Result<Ast, Error> {
        let outside_scope = UniqueNumberManager::new_scope();
        let inside_scope = UniqueNumberManager::new_scope();
        let init_bodys = bodys
            .map(|body| -> Result<Ast, Error> {
                Ok(body
                    .add_scope(scope.clone())
                    .add_scope(outside_scope.clone())
                    .add_scope(inside_scope.clone()))
            })?
            .to_list_checked::<Error>()?;
        let body_context = ExpandContext {
            use_site_scopes: Some(Rc::new(RefCell::new(BTreeSet::new()))),
            only_immediate: true,
            post_expansion_scope: Some(inside_scope),
            ..context.clone()
        };
        self.expand_body_loop(
            body_context,
            context,
            init_bodys.into(),
            vec![],
            vec![],
            make_check_no_duplicate_table(),
            original_syntax,
        )
    }

    fn expand_and_eval_for_syntaxes_binding(
        &mut self,
        rhs: Ast,
        id_count: usize,
        ctx: ExpandContext,
    ) -> Result<(Vec<Ast>, Ast), Error> {
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
    ) -> Result<Vec<Ast>, Error> {
        self.expand_and_eval_for_syntaxes_binding(rhs, id_count, ctx)
            .map(|x| x.0)
    }

    fn eval_for_bindings(
        &mut self,
        exp_rhs: Ast,
        id_count: usize,
        namespace: NameSpace,
    ) -> Result<Vec<Ast>, Error> {
        let compiled = self.compile(exp_rhs.clone(), &namespace)?;
        self.expand_time_eval(Ast1::Expression(Box::new(compiled)))
            .and_then(|values| {
                let list = match values {
                    Values::Many(vec) => vec,
                    Values::Single(ast) => vec![ast],
                };
                if id_count == list.len() {
                    Ok(list)
                } else {
                    Err(format!(
                        "wrong number of results ({} vs {id_count}) from {exp_rhs}",
                        list.len()
                    )
                    .into())
                }
            })
    }

    fn expand_transformer(&mut self, rhs: Ast, ctx: ExpandContext) -> Result<Ast, Error> {
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
    ) -> Result<Ast, Error> {
        let binding = Self::resolve(&s, false);
        let id = s.0.clone();
        let s = Ast::Syntax(Box::new(s.with(Ast::Symbol(id.clone()))));
        match binding {
            Ok(binding) => self.dispatch(self.lookup(&binding, &ctx, &id)?, s, ctx),
            _ => self.expand_implicit("#%top".into(), s, ctx),
        }
    }
}

pub fn to_id_list(ids: Ast) -> Result<Vec<Syntax<Symbol>>, Error> {
    let ids = ids.to_list_checked::<Error>()?;
    let ids = ids
        .into_iter()
        .map(std::convert::TryInto::try_into)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ids)
}

pub fn rebuild(s: Ast, rator: Ast) -> Ast {
    rator.datum_to_syntax(s.scope_set(), s.syntax_src_loc(), s.properties())
}
