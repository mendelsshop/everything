use std::usize;

use crate::{
    ast::{
        scope::{AdjustScope, Scope},
        syntax::Syntax,
        Ast, Pair, Symbol, Varidiac,
    },
    expander::{
        duplicate_check::{check_no_duplicate_ids, make_check_no_duplicate_table},
        expand,
    },
    list,
};
use crate::{sexpr, UniqueNumberManager};
use itertools::Itertools;
use matcher::match_syntax;

use super::binding::CompileTimeBinding;
use super::{expand::rebuild, expand_context::ExpandContext, Expander};
matcher::match_syntax_as!(LetSyntaxMatcher as

    (
        // TODO: letrec_syntaxes+values
        letrec_syntaxes_and_values
        (((trans_id ...) trans_rhs) ...)
        (((val_id ...) val_rhs) ...)
        body ..+
    )
);
matcher::match_syntax_as!(
LetMatcher as
                (
                    let_values
                    (((val_id ...) val_rhs) ...)
                    body
                    ..+
                ));
macro_rules! make_let_values_form {
    (syntax $id:ident,  $rec:literal) => {
        make_let_values_form!(
            $id,
            |s: Ast| LetSyntaxMatcher::matches(s),
            |m: &LetSyntaxMatcher, sc: Scope| itertools::Itertools::try_collect(
                m.trans_id
                    .clone()
                    .to_list_checked()?
                    .into_iter()
                    .map(|ids| {
                        expand::to_id_list(ids).map(|ids| {
                            ids.into_iter()
                                .map(|id| id.add_scope(sc.clone()))
                                .collect_vec()
                        })
                    }),
            ),
            |m: &LetSyntaxMatcher,
             trans_idss,
             this: &mut Expander,
             ctx: ExpandContext,
             sc: Scope| m
                .trans_rhs
                .clone()
                .to_list_checked()?
                .into_iter()
                .zip(trans_idss)
                .map(|(vals, ids): (_, Vec<_>)| {
                    this.eval_for_syntaxes_binding(
                        vals.add_scope(sc.clone()),
                        ids.len(),
                        ctx.clone(),
                    )
                })
                .try_collect(),
            |_, this: &mut Expander| this.core_datum_to_syntax("letrec-values".into()),
            $rec
        );
    };

    ( $id:ident,  $rec:literal) => {
        make_let_values_form!(
            $id,
            |s: Ast| LetMatcher::matches(s.clone()),
            |_, _| -> Result<_, String> { Ok(vec![]) },
            |_, _, _: &mut Expander, _, _| -> Result<_, String> { Ok(vec![]) },
            |m: &LetMatcher, _: &mut Expander| m.let_values.clone(),
            $rec
        );
    };
    ($id:ident, $m:expr, $trans_idss:expr, $trans_valss:expr, $letrecvalues:expr, $rec:literal) => {
        fn $id(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
            let variable = Ast::Symbol(self.variable.clone());
            //TODO: compile time matcher for this would have 2 different types
            let m = ($m)(s.clone())?;

            let sc: Scope = UniqueNumberManager::new_scope();
            let trans_idss: Vec<_> = ($trans_idss)(&m, sc.clone())?;

            let val_idss: Vec<_> = itertools::Itertools::try_collect(
                m.val_id.clone().to_list_checked()?.into_iter().map(|ids| {
                    expand::to_id_list(ids).map(|ids| {
                        ids.into_iter()
                            .map(|id| id.add_scope(sc.clone()))
                            .collect_vec()
                    })
                }),
            )?;
            check_no_duplicate_ids(
                trans_idss
                    .clone()
                    .into_iter()
                    .chain(val_idss.clone())
                    .concat(),
                &s,
                make_check_no_duplicate_table(),
            )?;
            let mut rec_ctx = ctx.clone();
            let val_keyss = val_idss
                .clone()
                .into_iter()
                .flat_map(|ids| Self::add_local_bindings(ids));
            rec_ctx
                .env
                .0
                .extend(val_keyss.map(|key| (key, variable.clone())));
            let trans_keyss = trans_idss
                .clone()
                .into_iter()
                .flat_map(|ids| Self::add_local_bindings(ids))
                .collect_vec();
            let trans_valss: Vec<Vec<_>> =
                ($trans_valss)(&m, trans_idss, self, ctx.clone(), sc.clone())?;

            rec_ctx
                .env
                .0
                .extend(trans_keyss.into_iter().zip(trans_valss.concat()));
            let letrec_values_id = ($letrecvalues)(&m, self);
            let val_idss = list_to_cons(val_idss.into_iter(), |ids| {
                list_to_cons(ids.into_iter(), |x| {
                    Ast::Syntax(Box::new(x.clone().with(Ast::Symbol(x.0))))
                })
            });
            Ok(expand::rebuild(
                s.clone(),
                list!(
                    letrec_values_id,
                    Ast::map2(val_idss, m.val_rhs, |ids, rhs| {
                        Ok(list!(
                            ids,
                            if $rec {
                                self.expand(rhs.add_scope(sc.clone()), rec_ctx.clone())?
                            } else {
                                self.expand(rhs, ctx.clone())?
                            }
                        ))
                    },)?,
                    self.expand_body(m.body, sc, s, rec_ctx)?
                ),
            ))
        }
    };
}
fn list_to_cons<T>(list: impl DoubleEndedIterator<Item = T>, mut f: impl FnMut(T) -> Ast) -> Ast {
    list.into_iter()
        .rfold(Ast::TheEmptyList, |rest, current| list!(f(current); rest))
}

impl Expander {
    fn add_local_bindings(ids: Vec<Syntax<Symbol>>) -> Vec<Symbol> {
        ids.into_iter().map(Self::add_local_binding).collect()
    }
    pub fn add_core_forms(&mut self) {
        self.add_core_form("lambda".into(), Self::core_form_lambda);
        // self.add_core_form("case-lambda".into(), Self::core_form_case_lambda);
        self.add_core_form("let-values".into(), Self::core_form_let_values);
        self.add_core_form("letrec-values".into(), Self::core_form_letrec_values);
        self.add_core_form(
            "letrec-syntaxes+values".into(),
            Self::core_form_letrec_syntaxes_and_values,
        );
        self.add_core_form("#%datum".into(), Self::core_form_datum);
        self.add_core_form("#%app".into(), Self::core_form_app);
        self.add_core_form("param".into(), Self::core_form_param);
        self.add_core_form("%app".into(), Self::core_form_app);
        self.add_core_form("quote".into(), Self::core_form_quote);
        self.add_core_form("quote-syntax".into(), Self::core_form_quote_syntax);
        self.add_core_form("if".into(), Self::core_form_if);
        // self.add_core_form(
        //     "with-continuation-mark".into(),
        //     Self::core_form_with_continuation_mark,
        // );
        self.add_core_form("begin".into(), Self::core_form_begin);
        self.add_core_form("begin0".into(), Self::core_form_begin0);
        self.add_core_form("set!".into(), Self::core_form_set);
        // from expand_top_level
        self.add_core_form("define-values".into(), Self::core_form_define_values);
        self.add_core_form("define-syntaxes".into(), Self::core_form_define_syntaxes);
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
    fn get_syntax(s: Ast) -> Option<Syntax<Ast>> {
        if let Ast::Syntax(s) = s {
            Some(*s)
        } else {
            None
        }
    }
    fn to_number(argc: Ast) -> Result<Syntax<usize>, String> {
        Self::get_syntax(argc)
            .ok_or("formals must be number".to_string())
            .and_then(|argc_syntax| {
                if let Ast::Number(argc) = argc_syntax.0 {
                    Ok(argc_syntax.with(argc.trunc() as usize))
                } else {
                    Err("formals must be number".to_string())
                }
            })
    }
    fn lambda_formals(formals: Ast) -> Result<(Syntax<usize>, Option<Varidiac>), String> {
        let check_variadic = |argc: Ast| {
            Self::get_syntax(argc)
                .ok_or("formals must be number".to_string())
                .and_then(|argc| {
                    if let Ast::Symbol(variadic) = argc.0 {
                        if &*variadic.0 == "+" || &*variadic.0 == "*" {
                            Ok(if &*variadic.0 == "+" {
                                Varidiac::AtLeast1
                            } else {
                                Varidiac::AtLeast0
                            })
                        } else {
                            Err("variadics must be * or +".to_string())
                        }
                    } else {
                        Err("variadics must be * or +".to_string())
                    }
                })
        };

        match_syntax!( (argc variadic))(formals.clone())
            .map(|m| {
                Self::to_number(m.argc)
                    .and_then(|n| check_variadic(m.variadic).map(|x| (n, Some(x))))
            })
            .or_else(|_| {
                match_syntax!((argc))(formals.clone())
                    .map(|m| Self::to_number(m.argc).map(|n| (n, None)))
            })?
    }
    fn core_form_param(&mut self, s: Ast, env: ExpandContext) -> Result<Ast, String> {
        let _ = env;
        match_syntax!((param index))(s.clone()).and_then(|m| {
            Self::to_number(m.index).map(|n| {
                let n0 = n.0;
                rebuild(
                    s,
                    Ast::Syntax(Box::new(
                        n.with::<Ast>(Ast::Symbol(format!("{n0:o}").into())),
                    )),
                )
            })
        })
    }
    //#[trace]
    fn core_form_lambda(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = match_syntax!(
            (lambda formals body)
        )(s.clone())?;
        let sc = UniqueNumberManager::new_scope();
        let id = m.formals;
        let (formals, variadiac) = Self::lambda_formals(id)?;

        //let ids = m("id".into()).ok_or("internal error".to_string())?;
        //let ids = ids.map_pair(|term, base| match term {
        //    Ast::Syntax(id) => {
        //        let id = id.add_scope(sc);
        //        Ok(Ast::Syntax(Box::new(id)))
        //    }
        //    Ast::TheEmptyList if base => Ok(Ast::TheEmptyList),
        //    _ => Err(format!(
        //        "{term} is not a symbol so it cannot be a parameter"
        //    )),
        //})?;
        //let body_env = ids.clone().foldl_pair(
        //    |term, base, env: Result<CompileTimeEnvoirnment, String>| match term {
        //        Ast::Syntax(ref id_syntax) => {
        //            if let Ast::Symbol(id) = &id_syntax.0 {
        //                let binding =
        //                    self.add_local_binding(Syntax(id.clone(), id_syntax.1.clone()));
        //                env.map(|env| {
        //                    env.extend(binding.clone(), Ast::Symbol(self.variable.clone()))
        //                })
        //            } else {
        //                Err(format!(
        //                    "{term} is not a symbol so it cannot be a parameter"
        //                ))
        //            }
        //        }
        //        Ast::TheEmptyList if base => env,
        //        _ => Err(format!(
        //            "{term} is not a symbol so it cannot be a parameter"
        //        )),
        //    },
        //    Ok(env),
        //)?;
        let arg_count = variadiac.map_or(formals.0, |_| formals.0 + 1);
        let args = (0..arg_count).map(|i| {
            formals
                .clone()
                .with::<Symbol>(format!("{i:o}").into())
                .add_scope(sc.clone())
        });
        // let body_env = args

        let mut body_ctx = ctx;
        body_ctx.env.0.extend(
            args.clone().map(|i| {
                (
                    Self::add_local_binding(i),
                    Ast::Symbol(self.variable.clone()),
                )
            }), // .fold(HashMap::new(), |env, i| {
                //     env.extend(i.0, Ast::Symbol(i.1))
                // }),
        );
        let exp_body = self.expand(m.body.add_scope(sc.clone()), body_ctx)?;
        let lambda = m.lambda;
        let new_lambda = if formals.0 == 0 && variadiac.is_none() {
            sexpr!( (#(lambda) exp_body))
        } else {
            args.rfold(exp_body, |body, i| {
                let i0 = &i.0;
                list!(
                    lambda.clone(),
                    Ast::Syntax(Box::new(
                        i.with_ref(Ast::Symbol(
                            variadiac
                                .and_then(|varidiac| {
                                    if i0.0.to_string() == format!("{arg_count:o}") {
                                        Some(Symbol(format!("{i0}{varidiac}").into()))
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or(i0.clone()),
                        ))
                    )),
                    body
                )
            })
        };
        // (lambda (0 *) ..) becomes (lambda 0* ...)
        // (lambda (0 ) ..) becomes (lambda  ...)
        // o(N) = octal of n
        // (lambda (N ) ..) becomes (lambda 0 (.. (lambda o(N)  ...)))
        // (lambda (N +) ..) becomes (lambda 0 (.. (lambda o(N) lambda o(N + 1 )*  ...)))
        Ok(rebuild(s, new_lambda))
    }

    // fn make_lambda_expander(
    //     &mut self,
    //     s: Ast,
    //     formals: Ast,
    //     bodys: Ast,
    //     ctx: ExpandContext,
    // ) -> Result<(Ast, Ast), String> {
    //     let sc = UniqueNumberManager::new_scope();
    //     let ids = self.parse_and_flatten_formals(formals.clone(), sc.clone())?;
    //     check_no_duplicate_ids(ids.clone(), &s, make_check_no_duplicate_table())?;
    //     let variable = Ast::Symbol(self.variable.clone());
    //     let keys = ids.into_iter().map(Self::add_local_binding);
    //     let mut body_ctx = ctx;
    //     body_ctx
    //         .env
    //         .0
    //         .extend(keys.map(|key| (key, variable.clone())));
    //     let exp_body = self.expand_body(bodys, sc.clone(), s, body_ctx)?;
    //     Ok((formals.add_scope(sc), exp_body))
    // }
    // fn core_form_lambda(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
    //     let m = matcher::match_syntax!(
    //         (
    //             lambda
    //             formal
    //             body
    //             ..+
    //         )
    //     )(s.clone())?;
    //     let (formals, body) = self.make_lambda_expander(s.clone(), m.formal, m.body, ctx)?;
    //     Ok(rebuild(s, list!(m.lambda, formals, body)))
    // }
    // fn core_form_case_lambda(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
    //     let m = matcher::match_syntax!((
    //         case_lambda
    //         (formals body ..+)
    //         ...
    //     ))(s.clone())?;
    //     let cm = matcher::match_syntax!(
    //         (case_lambda clause ...)
    //     )(s.clone())?;
    //     let iter = m
    //         .formals
    //         .to_list_checked()?
    //         .into_iter()
    //         .zip(m.body.to_list_checked()?)
    //         .zip(cm.clause.to_list_checked()?)
    //         .try_rfold(Ast::TheEmptyList, |rest, ((formals, bodys), clause)| {
    //             let (formals, body) =
    //                 self.make_lambda_expander(s.clone(), formals, bodys, ctx.clone())?;
    //             Ok::<Ast, String>(list!(rebuild(clause, list!(formals, body));rest))
    //         })?;
    //     Ok(rebuild(s, list!(m.case_lambda; iter)))
    // }
    // fn parse_and_flatten_formals(
    //     &self,
    //     formals: Ast,
    //     sc: Scope,
    // ) -> Result<Vec<Syntax<Symbol>>, String> {
    //     fn parse_and_flatten_formals_loop(
    //         formals: Ast,
    //         all_formals: Ast,
    //         sc: Scope,
    //         formals_list: &mut Vec<Syntax<Symbol>>,
    //     ) -> Result<(), String> {
    //         match formals {
    //             Ast::Syntax(s) => match s.0 {
    //                 Ast::Symbol(ref sym) => {
    //                     formals_list.push(s.clone().with(sym.clone()));
    //                     Ok(())
    //                 }
    //                 Ast::TheEmptyList => Ok(()),
    //                 p @ Ast::Pair(_) => {
    //                     parse_and_flatten_formals_loop(p, all_formals, sc, formals_list)
    //                 }
    //                 p => Err(format!("not an identifier: {p}")),
    //             },
    //             Ast::TheEmptyList => Ok(()),
    //             Ast::Pair(p) => {
    //                 let Pair(car, cdr) = *p;
    //                 let s: Syntax<Symbol> = car
    //                     .clone()
    //                     .try_into()
    //                     .map_err(|_| format!("not and identifier {car}"))?;
    //                 let s = s.add_scope(sc.clone());
    //                 formals_list.push(s);
    //                 parse_and_flatten_formals_loop(cdr, all_formals, sc, formals_list)
    //             }
    //             _ => Err(format!("bad arguement sequence: {all_formals}")),
    //         }
    //     }
    //     let mut formal_list = vec![];
    //     parse_and_flatten_formals_loop(formals.clone(), formals, sc, &mut formal_list)
    //         .map(|()| formal_list)
    // }
    make_let_values_form!(core_form_let_values, false);
    make_let_values_form!(core_form_letrec_values, true);
    make_let_values_form!(syntax core_form_letrec_syntaxes_and_values, true);

    fn core_form_datum(&mut self, s: Ast, _ctx: ExpandContext) -> Result<Ast, String> {
        // TODO: let m = matcher::match_syntax!((#%datum  datum))(s.clone())?;
        let m = matcher::match_syntax!((_datum  datum))(s.clone())?;
        let datum = m.datum;
        if matches!(datum, Ast::Syntax(ref s) if s.0.is_keyword()) {
            return Err(format!("keyword misused as an expression: {datum}"));
        }
        Ok(rebuild(
            s,
            list!(self.core_datum_to_syntax("quote".into()), datum),
        ))
    }
    fn core_form_app(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = matcher::match_syntax!(
            //TODO: should app be a syntax object
            // TODO: (%app rator rand ...)
            (app rator rand ...)
        )(s.clone())?;
        let rator = self.expand(m.rator, ctx.clone())?;
        let rand = self.expand(m.rand, ctx)?;
        Ok(rebuild(
            s,
            Ast::Pair(Box::new(Pair(
                m.app,
                Ast::Pair(Box::new(Pair(rator, rand))),
            ))),
        ))
    }
    fn core_form_quote(&mut self, s: Ast, _ctx: ExpandContext) -> Result<Ast, String> {
        matcher::match_syntax!( (quote datum))(s.clone()).map(|_| s)
    }
    fn core_form_quote_syntax(&mut self, s: Ast, _ctx: ExpandContext) -> Result<Ast, String> {
        matcher::match_syntax!( (quote_syntax datum))(s.clone()).map(|_| s)
    }
    fn core_form_if(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = matcher::match_syntax!(

            (
                r#if
                condition
                consequent
                alternative
            )
        )(s.clone())?;
        Ok(rebuild(
            s,
            list!(
                m.r#if,
                self.expand(m.condition, ctx.clone())?,
                self.expand(m.consequent, ctx.clone())?,
                self.expand(m.alternative, ctx)?
            ),
        ))
    }
    fn core_form_with_continuation_mark(
        &mut self,
        s: Ast,
        ctx: ExpandContext,
    ) -> Result<Ast, String> {
        let m = matcher::match_syntax!(
            (
                with_continuation_mark
                key
                val
                body
            )
        )(s.clone())?;
        Ok(rebuild(
            s,
            list!(
                m.with_continuation_mark,
                self.expand(m.key, ctx.clone())?,
                self.expand(m.val, ctx.clone())?,
                self.expand(m.body, ctx)?
            ),
        ))
    }
    fn make_begin(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        let m = match_syntax!( (begin e ..+))(s.clone())?;

        Ok(rebuild(
            s,
            list!(m.begin; m.e.map(|e|self.expand(e, ctx.clone()))?),
        ))
    }
    fn core_form_begin(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        self.make_begin(s, ctx)
    }
    fn core_form_begin0(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        self.make_begin(s, ctx)
    }
    fn core_form_set(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        // TODO: let m = matcher::match_syntax!( (set! id rhs))(s.clone(),)?;
        let m = matcher::match_syntax!( (set id rhs))(s.clone())?;
        let id = m.id;
        let binding = Self::resolve(&id.clone().try_into()?, false)
            .inspect_err(|e| {
                dbg!(format!("{e}"));
            })
            .map_err(|_| format!("no binding for assignment: {s}"))?;
        let t = ctx
            .env
            .lookup(&binding, &ctx.namespace, &s, self.variable.clone())?;
        if !matches!(t, CompileTimeBinding::Regular(Ast::Symbol(s)) if s == self.variable) {
            return Err(format!("cannot assign to syntax: {s}"));
        }
        let set = m.set;
        let rhs = m.rhs;
        Ok(expand::rebuild(s, list!(set, id, rhs)))
    }

    fn core_form_link(&mut self, s: Ast, env: ExpandContext) -> Result<Ast, String> {
        // TODO: verify that dest/src label(s) are actually labels (requires updating ast with
        // everything features)
        // TODO: expand recursivly?
        let m = match_syntax!(
            (
                link
                dest_label
                src_labels
                ...
            )
        )(s.clone())?;
        let filter_label = |label| {
            if matches!(label, Ast::Label(_)) {
                Ok(label)
            } else {
                Err("not a label".to_string())
            }
        };
        let link = m.link;
        let dest = filter_label(m.dest_label)?;
        let src = m.src_labels.map(filter_label)?;
        Ok(rebuild(s, sexpr!((link dest src))))
    }
}
