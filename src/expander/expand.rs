use std::collections::BTreeSet;

use crate::{
    ast::{scope::AdjustScope, syntax::Syntax, Ast, Function, Pair, Symbol},
    list,
};

use super::{
    binding::{Binding, CompileTimeBinding, CompileTimeEnvoirnment},
    expand_context::ExpandContext,
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
        let binding = binding.and_then(|binding| self.lookup(&binding, &ctx, id.clone()));
        match binding {
            Ok(binding) if !matches!(&binding, CompileTimeBinding::Regular(Ast::Symbol(sym)) if *sym == self.variable) => {
                self.dispatch(binding, s, ctx)
            }
            _ => self.expand_implicit("%app".into(), s, ctx),
        }
    }

    fn expand_implicit(&self, sym: Symbol, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        todo!()
    }

    fn lookup(
        &self,
        binding: &Binding,
        ctx: &ExpandContext,
        id: Symbol,
    ) -> Result<CompileTimeBinding, String> {
        ctx.env.lookup(binding, &ctx.namespace, id)
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
        ctx: ExpandContext,
    ) -> Result<Ast, String> {
        match t {
            CompileTimeBinding::Regular(t) => match t {
                Ast::Function(transfromer) => {
                    let apply_transformer = self.apply_transformer(transfromer, s)?;
                    self.expand(apply_transformer, ctx)
                }
                Ast::Symbol(variable) if variable == self.variable => Ok(s),
                _ => Err(format!("illegal use of syntax: {t}")),
            },
            CompileTimeBinding::CoreForm(form) => form(self, s, ctx),
        }
    }
    pub fn eval_for_syntax_binding(&mut self, rhs: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        // let var_name = format!("problem `evaluating` macro {rhs}");
        let expand = self.expand_transformer(rhs, ctx)?;
        let compile = self.compile(expand)?;
        self.expand_time_eval(compile)
    }

    fn expand_transformer(&mut self, rhs: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        self.expand(rhs, CompileTimeEnvoirnment::new())
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
            Ok(binding) => self.dispatch(self.lookup(&binding, &ctx, id)?, s, ctx),
            _ => self.expand_implicit("%top".into(), s, ctx),
        }
    }
}

pub fn rebuild(s: Ast, rator: Ast) -> Ast {
    rator.datum_to_syntax(s.scope_set(), s.syntax_src_loc(), s.properties())
}
