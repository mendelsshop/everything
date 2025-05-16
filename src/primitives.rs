use std::rc::Rc;

use crate::{
    ast::{Ast, Boolean, Function, Pair, Primitive},
    error::Error,
    evaluator::Values,
};

impl Ast {
    pub fn primitive_datum_to_syntax(self) -> Result<Values, Error> {
        let arity = self.size();
        let Self::Pair(e) = self else {
            Err(format!("arity error: expected 2 argument, got {arity}",))?
        };
        let Pair(scopes, Self::Pair(syntax_object)) = *e else {
            Err(format!("arity error: expected 2 argument, got {arity}"))?
        };
        let Pair(syntax_object, Self::TheEmptyList) = *syntax_object else {
            Err(format!("arity error: expected 1 argument, got {arity}"))?
        };
        // TODO: properties and location
        Ok(Values::Single(syntax_object.datum_to_syntax(
            scopes.scope_set(),
            None,
            None,
        )))
    }
    pub fn primitive_syntax_to_datum(self) -> Result<Values, Error> {
        let Self::Pair(e) = self else {
            Err(format!(
                "arity error: expected 1 argument, got {}",
                self.size()
            ))?
        };
        let Pair(e, Self::TheEmptyList) = *e else {
            Err(format!(
                "arity error: expected 1 argument, got {}",
                e.size()
            ))?
        };
        Ok(Values::Single(e.syntax_to_datum()))
    }
    pub fn primitive_syntax_e(self) -> Result<Values, Error> {
        let Self::Pair(e) = self else {
            Err(format!(
                "arity error: expected 1 argument, got {}, syntax e",
                self.size()
            ))?
        };
        let Pair(Self::Syntax(e), Self::TheEmptyList) = *e else {
            Err(format!(
                "arity error: expected 1 argument, got {} or got non syntax object, syntax e",
                e.size()
            ))?
        };
        Ok(Values::Single(e.0))
    }
    pub fn primitive_cons(self) -> Result<Values, Error> {
        let Self::Pair(e) = self else {
            Err(format!(
                "arity error: expected 2 argument, got {}",
                self.size()
            ))?
        };
        let Pair(ref fst, Self::Pair(ref last)) = *e else {
            Err(format!(
                "arity error: expected 2 argument, got {}",
                e.size()
            ))?
        };
        let Pair(ref snd, Self::TheEmptyList) = **last else {
            Err(format!(
                "arity error: expected 2 argument, got {}",
                e.size()
            ))?
        };
        Ok(Values::Single(Self::Pair(Box::new(Pair(
            fst.clone(),
            snd.clone(),
        )))))
    }
    pub fn primitive_car(self) -> Result<Values, Error> {
        let Self::Pair(e) = self else {
            Err(format!(
                "arity error: expected 1 argument, got {}, car",
                self.size()
            ))?
        };

        let Pair(Self::Pair(e), Self::TheEmptyList) = *e else {
            Err(format!(
                "arity error: expected 1 argument, got {} or given non pair, car",
                e.size()
            ))?
        };
        let Pair(fst, _) = *e;
        Ok(Values::Single(fst))
    }
    pub fn primitive_cdr(self) -> Result<Values, Error> {
        let Self::Pair(e) = self else {
            Err(format!(
                "arity error: expected 1 argument, got {}, cdr",
                self.size()
            ))?
        };
        let Pair(Self::Pair(e), Self::TheEmptyList) = *e else {
            Err(format!(
                "arity error: expected 1 argument, got {} or given non pair, cdr",
                e.size()
            ))?
        };
        let Pair(_, snd) = *e;
        Ok(Values::Single(snd))
    }

    pub const fn primitive_list(self) -> Result<Values, Error> {
        Ok(Values::Single(self))
    }
    pub fn primitive_map(self) -> Result<Values, Error> {
        let Self::Pair(e) = self else {
            Err(format!(
                "arity error: expected 2 argument, got {}, map",
                self.size()
            ))?
        };

        let Pair(Self::Function(ref f), Self::Pair(ref last)) = *e else {
            Err(format!(
                "arity error: expected 2 argument, got {}, or given non function {}, map",
                e.size(),
                e.0
            ))?
        };
        let Pair(ref l, Self::TheEmptyList) = **last else {
            Err(format!(
                "arity error: expected 2 argument, got {}",
                e.size()
            ))?
        };
        l.map(|a| f.apply_single(Self::Pair(Box::new(Pair(a, Self::TheEmptyList)))))
            .map(Values::Single)
    }
    pub fn primitive_values(self) -> Result<Values, Error> {
        match self {
            Self::Pair(p) if p.1 == Self::TheEmptyList => Ok(Values::Single(p.0)),
            _ => self.to_list_checked().map(Values::Many),
        }
    }
    pub fn primitive_null(self) -> Result<Values, Error> {
        match self {
            Self::Pair(p) if *p == Pair(Self::TheEmptyList, Self::TheEmptyList) => {
                Ok(Values::Single(Self::Boolean(Boolean::True)))
            }
            _ => Ok(Values::Single(Self::Boolean(Boolean::False))),
        }
    }
}

pub fn new_primitive_env(mut adder: impl FnMut(Rc<str>, Ast)) {
    // add code here
    adder(
        "datum-to-syntax".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "datum-to-syntax",
            operation: Ast::primitive_datum_to_syntax,
        })),
    );
    adder(
        "syntax-to-datum".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "syntax-to-datum",
            operation: Ast::primitive_syntax_to_datum,
        })),
    );
    adder(
        "syntax-e".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "syntax-e",
            operation: Ast::primitive_syntax_e,
        })),
    );
    adder(
        "cons".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "cons",
            operation: Ast::primitive_cons,
        })),
    );
    adder(
        "car".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "car",
            operation: Ast::primitive_car,
        })),
    );
    adder(
        "cdr".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "cdr",
            operation: Ast::primitive_cdr,
        })),
    );
    adder(
        "list".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "list",
            operation: Ast::primitive_list,
        })),
    );
    adder(
        "map".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "map",
            operation: Ast::primitive_map,
        })),
    );
    adder(
        "null?".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "null?",
            operation: Ast::primitive_null,
        })),
    );
    adder(
        "values".into(),
        Ast::Function(Function::Primitive(Primitive {
            name: "values",
            operation: Ast::primitive_values,
        })),
    );
}
