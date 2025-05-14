use std::{
    collections::{BTreeSet, HashMap},
    rc::Rc,
};

use binding::CoreForm;
use expand_context::ExpandContext;
use namespace::NameSpace;

use crate::ast::{scope::AdjustScope, Boolean};
use crate::{
    ast::scope::Scope,
    ast::{syntax::Syntax, Ast, Symbol},
    evaluator::{Env, EnvRef},
    primitives::new_primitive_env,
    UniqueNumberManager,
};
use crate::{
    ast::syntax::{Properties, SourceLocation},
    evaluator::Values,
};

pub mod binding;
mod compile;
mod core;
mod duplicate_check;
pub mod expand;
pub mod expand_context;
pub mod expand_expr;
mod expand_top_level;
mod namespace;
pub struct Expander {
    core_forms: HashMap<Rc<str>, CoreForm>,
    core_primitives: HashMap<Rc<str>, Ast>,
    core_scope: Scope,
    expand_time_env: EnvRef,
    run_time_env: EnvRef,
    core_syntax: Syntax<Ast>,
    pub(crate) variable: Symbol,
}

impl Default for Expander {
    fn default() -> Self {
        Self::new()
    }
}

impl Expander {
    #[must_use]
    pub fn new() -> Self {
        let core_scope = UniqueNumberManager::new_scope();
        let variable = UniqueNumberManager::gen_sym("variable");
        let mut this = Self {
            core_syntax: Syntax(
                Ast::Boolean(Boolean::False),
                BTreeSet::from([core_scope.clone()]),
                SourceLocation::default(),
                Properties::new(),
            ),
            core_scope,
            core_primitives: HashMap::new(),
            core_forms: HashMap::new(),
            run_time_env: Env::new_env(),
            expand_time_env: Env::new_env(),
            variable,
        };
        this.add_core_forms();
        new_primitive_env(|name, primitive| {
            this.add_core_primitive(name, primitive);
        });

        this
    }
    pub fn namespace(&mut self) -> NameSpace {
        let mut ns = NameSpace::default();
        self.declare_core_top_level(&mut ns);
        ns
    }
    pub fn introduce<T: AdjustScope>(&self, s: T) -> T {
        s.add_scope(self.core_scope.clone())
    }

    // TODO: mutability of ns/ctx how should/are changes to envoirnments preserved over multiple
    // expansions maybe have a namespace as part of the expander object
    // TODO: annonate fully expanded expressions so we don't have to reexpand them
    pub fn eval(&mut self, s: Ast, ns: NameSpace) -> Result<Values, String> {
        let ctx = ExpandContext::new(ns.clone());
        let expanded = self.expand(
            self.namespace_syntax_introduce(s.datum_to_syntax(None, None, None)),
            ctx,
        )?;
        let compiled = self
            .compile(expanded, &ns)
            .inspect(|e| println!("after expander: {e}"))?;
        self.run_time_eval(compiled)
    }
}
#[cfg(test)]
mod tests {

    use crate::ast::ast1::Ast1;
    use crate::ast::Ast;
    use crate::evaluator::{Evaluator, Values};

    use crate::expander::Expander;
    use crate::{list, sexpr};

    use super::expand_context::ExpandContext;

    impl Expander {
        fn expand_expression(&mut self, e: Ast) -> Result<Ast, String> {
            let ctx = ExpandContext::new(self.namespace());
            self.expand(
                self.namespace_syntax_introduce(e.datum_to_syntax(None, None, None)),
                ctx,
            )
        }

        fn compile_eval_expression(&mut self, e: Ast) -> (Ast1, Values) {
            let c = self
                .expand_expression(e)
                .and_then(|e| {
                    let namespace = self.namespace();
                    self.compile(e, &namespace)
                })
                .and_then(|e| {
                    Evaluator::eval(e.clone(), self.run_time_env.clone()).map(|v| (e, v))
                });
            match c {
                Ok(v) => v,
                Err(e) => panic!("{}", e),
            }
        }
        fn eval_expression(&mut self, e: Ast, check: Option<Values>) -> Values {
            let (_, v) = self.compile_eval_expression(e);
            check.inspect(|check| assert_eq!(&v, check));
            v
        }
    }

    fn add_let(e: Ast) -> Ast {
        sexpr!(
            ("letrec-syntaxes+values" ([ (let) (lambda (stx)
                       ("datum->syntax"
                        ("quote-syntax" here)
                        (cons
                         (list ("quote-syntax" lambda)
                               (map (lambda (b)
                                      (car ("syntax-e" b)))
                                    ("syntax-e" (car (cdr ("syntax-e" stx)))))
                               (car (cdr (cdr ("syntax-e" stx)))))
                         (map (lambda (b)
                                (car (cdr ("syntax-e" b))))
                              ("syntax-e" (car (cdr ("syntax-e" stx)))))))) ])
            ()
                #(e))
        )
    }

    #[test]
    fn expander_test_lambda() {
        let mut expander = Expander::new();
        expander.compile_eval_expression(sexpr!((lambda (x) x)));
    }

    #[test]
    fn expander_test_basic_let() {
        let mut expander = Expander::new();
        let expr = add_let(sexpr!((lambda (x) (let ((y x)) y))));
        expander.compile_eval_expression(expr);
    }
    #[test]
    fn expander_test_basic_macro() {
        let mut expander = Expander::new();
        let expr = sexpr!((lambda (x)
                ("letrec-syntaxes+values" ([ (y) (lambda (stx) ("quote-syntax" 7)) ]) ()
     y)));

        expander.compile_eval_expression(expr);
    }
    #[test]
    fn expander_test_complex_let() {
        let mut expander = Expander::new();
        let expr = add_let(sexpr!((let ((z 9))
        ("letrec-syntaxes+values" ([ (m) (lambda (stx) (car (cdr ("syntax-e" stx)))) ]) ()
          (let (( x 5 )
                ( y (lambda (z) z) ))
            (let (( z 10 ))
              (list z (m 10))))))
            ));
        expander.compile_eval_expression(expr);
    }

    #[test]
    fn expander_test_expansion_not_captured() {
        let mut expander = Expander::new();
        let expr = add_let(sexpr!((let ((x (quote "x-1")))
        ("letrec-syntaxes+values" ([ (m) (lambda (stx) ("quote-syntax" x)) ]) ()
          (let ((x (quote "x-3")))
            (m))))
            ));
        expander.eval_expression(expr, Some(Values::Single(Ast::Symbol("x-1".into()))));
    }

    #[test]
    fn expander_test_not_capturing_expansion() {
        let mut expander = Expander::new();
        let expr = add_let(sexpr!(
                    (let (( x (quote "x-1") ))
            ("letrec-syntaxes+values" ([ (m) (lambda (stx)
                              ("datum->syntax"
        ("quote-syntax" here)
                               (list ("quote-syntax" let)
                                     (list (list ("quote-syntax" x)
                                                 ("quote-syntax"(quote  "x-2"))))
                                     (car (cdr ("syntax-e" stx)))))) ]) ()
              (let (( x (quote "x-3") ))
                (m x))))
                ));
        expander.eval_expression(expr, Some(Values::Single(Ast::Symbol("x-3".into()))));
    }

    #[test]
    fn expander_test_distinct_generated_variables_via_introduction_scope() {
        let mut expander = Expander::new();
        let expr = add_let(sexpr!(
            ("letrec-syntaxes+values" ([ (gen2) (lambda (stx)
                        ("datum->syntax"
("quote-syntax" here)
                         (list ("quote-syntax" let)
                               (list (list (car (cdr (cdr ("syntax-e" stx))))
                                           (car (cdr (cdr (cdr (cdr ("syntax-e" stx)))))))
                                     (list (car (cdr (cdr (cdr ("syntax-e" stx)))))
                                           (car (cdr (cdr (cdr (cdr (cdr ("syntax-e" stx)))))))))
                               (list ("quote-syntax" list)
                                     (car (cdr (cdr ("syntax-e" stx))))
                                     (car (cdr (cdr (cdr ("syntax-e" stx))))))))) ]) ()
    ("letrec-syntaxes+values" ([ (gen1) (lambda ( stx)
                         ("datum->syntax"
("quote-syntax" here)
                          (cons (car (cdr ("syntax-e" stx)))
                                (cons ("quote-syntax" gen2)
                                      (cons ("quote-syntax" x)
                                            (cdr (cdr ("syntax-e" stx)))))))) ]) ()
      (gen1 gen1 1 2)))));
        expander.eval_expression(
            expr,
            Some(Values::Single(list!(Ast::Number(1.), Ast::Number(2.)))),
        );
    }
    #[test]
    fn expander_test_non_transformer_binding_misuse() {
        let mut expander = Expander::new();
        let env = ExpandContext::new(expander.namespace());
        let expr = sexpr!(
            ("letrec-syntaxes+values" ([ (v) 1 ]) ()
                            v)
        );
        assert!(expander
            .expand(
                expander.namespace_syntax_introduce(expr.datum_to_syntax(None, None, None)),
                env
            )
            .is_err_and(|e| e.contains("illegal use of syntax")));
    }
}
