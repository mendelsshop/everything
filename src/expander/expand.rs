use std::collections::BTreeSet;

use crate::{
    ast::{scope::AdjustScope, syntax::Syntax, Ast, Function, Pair, Symbol},
    list,
};

use super::{
    binding::{CompileTimeBinding, CompileTimeEnvoirnment},
    r#match::match_syntax,
    Expander,
};

impl Expander {
    pub fn namespace_syntax_introduce<T: AdjustScope>(&self, s: T) -> T {
        s.add_scope(self.core_scope.clone())
    }
    //#[trace]
    pub fn expand(&mut self, s: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        match s.clone() {
            Ast::Syntax(syntax) => match syntax.0 {
                Ast::Symbol(ref symbol) => {
                    self.expand_identifier(syntax.with_ref(symbol.clone()), env)
                }
                Ast::Pair(p) if p.0.identifier() => self.expand_id_application_form(*p, s, env),
                Ast::Pair(_) => self.expand_app(s, env),
                Ast::TheEmptyList => self.expand_app(s, env),
                _ => Ok(rebuild(
                    s.clone(),
                    list!(
                        Ast::Symbol("quote".into()).datum_to_syntax(
                            Some(BTreeSet::from([self.core_scope.clone()])),
                            None,
                            None
                        ),
                        s
                    ),
                )),
            },
            Ast::Pair(_) => self.expand_app(s, env),
            Ast::TheEmptyList => self.expand_app(s, env),
            _ => Ok(rebuild(
                s.clone(),
                list!(
                    Ast::Symbol("quote".into()).datum_to_syntax(
                        Some(BTreeSet::from([self.core_scope.clone()])),
                        None,
                        None
                    ),
                    s
                ),
            )),
        }
    }
    // constraints = s.len() > 0
    // constraints = s[0] == Ast::Syntax(Symbol)
    //#[trace]
    pub(crate) fn expand_id_application_form(
        &mut self,
        p: Pair,
        s: Ast,
        env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        let Ast::Syntax(ref id_syntax) = p.0 else {
            unreachable!()
        };
        let Ast::Symbol(ref id) = id_syntax.0 else {
            unreachable!();
        };
        let binding = self.resolve(&id_syntax.with_ref(id.clone()))?;
        let t = env.lookup(binding, self.core_forms.clone(), self.variable.clone());
        match t {
            Err(_) => self.expand_app(s, env),
            Ok(CompileTimeBinding::Regular(Ast::Symbol(sym))) if sym == self.variable => {
                self.expand_app(s, env)
            }
            Ok(t) => self.dispatch(t, s, env),
        }
    }

    pub(crate) fn apply_transformer(&mut self, m: Function, s: Ast) -> Result<Ast, String> {
        let intro_scope = self.scope_creator.new_scope();
        let intro_s = s.add_scope(intro_scope.clone());
        let transformed_s = m.apply(Ast::Pair(Box::new(Pair(intro_s, Ast::TheEmptyList))))?;
        if !matches!(transformed_s, Ast::Syntax(_)) {
            return Err(format!("transformer produced non syntax: {transformed_s}"));
        }
        Ok(transformed_s.flip_scope(intro_scope))
    }

    fn dispatch(
        &mut self,
        t: CompileTimeBinding,
        s: Ast,
        env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        match t {
            CompileTimeBinding::Regular(t) => match t {
                Ast::Function(transfromer) => {
                    let apply_transformer = self.apply_transformer(transfromer, s)?;
                    self.expand(apply_transformer, env)
                }
                Ast::Symbol(variable) if variable == self.variable => Ok(s),
                _ => Err(format!("illegal use of syntax: {t}")),
            },
            CompileTimeBinding::CoreForm(form) => form(self, s, env),
        }
    }
    pub fn eval_for_syntax_binding(
        &mut self,
        rhs: Ast,
        env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        // let var_name = format!("problem `evaluating` macro {rhs}");
        let expand = self.expand_transformer(rhs, env)?;
        let compile = self.compile(expand)?;
        self.expand_time_eval(compile)
    }

    fn expand_transformer(&mut self, rhs: Ast, env: CompileTimeEnvoirnment) -> Result<Ast, String> {
        self.expand(rhs, CompileTimeEnvoirnment::new())
    }

    pub(crate) fn expand_app(
        &mut self,
        s: Ast,
        env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        let m = match_syntax(
            s.clone(),
            list!("rator".into(), "rand".into(), "...".into()),
        )?;
        let rator = m("rator".into()).ok_or("internal error".to_string())?;
        let rand = m("rand".into())
            .ok_or("internal error".to_string())?
            .map(|rand| self.expand(rand, env.clone()))?;
        Ok(rebuild(
            s,
            Ast::Pair(Box::new(Pair(
                Into::<Ast>::into("%app").datum_to_syntax(
                    Some(BTreeSet::from([self.core_scope.clone()])),
                    None,
                    None,
                ),
                Ast::Pair(Box::new(Pair(self.expand(rator, env)?, rand))),
            ))),
        ))
    }
    pub(crate) fn expand_identifier(
        &mut self,
        s: Syntax<Symbol>,
        env: CompileTimeEnvoirnment,
    ) -> Result<Ast, String> {
        let binding = self.resolve(&s)?;
        let t = env
            .lookup(binding, self.core_forms.clone(), self.variable.clone())
            .map_err(|_| format!("illegal use of syntax {s:?}"))?;
        self.dispatch(
            t,
            Ast::Syntax(Box::new(s.with_ref(Ast::Symbol(s.0.clone())))),
            env,
        )
    }
}

pub fn rebuild(s: Ast, rator: Ast) -> Ast {
    rator.datum_to_syntax(s.scope_set(), s.syntax_src_loc(), s.properties())
}
