use matcher_proc_macro::match_syntax;
use std::{collections::BTreeSet, rc::Rc};

use crate::{
    ast::{
        syntax::{Properties, SourceLocation, Syntax},
        Ast, Symbol,
    },
    error::Error,
};

use super::{
    binding::{Binding, CompileTimeBinding, CoreForm},
    namespace::NameSpace,
    Expander,
};

impl Expander {
    fn add_core_binding(&self, sym: Symbol) -> Result<(), Error> {
        Self::add_binding(
            Syntax(
                sym.clone(),
                BTreeSet::from([self.core_scope.clone()]),
                SourceLocation::default(),
                Properties::new(),
            ),
            Binding::TopLevel(sym.0),
        )
    }

    pub fn add_core_form(&mut self, sym: Rc<str>, proc: CoreForm) {
        self.add_core_binding(sym.clone().into());
        self.core_forms.insert(sym, proc);
    }
    pub fn add_core_primitive(&mut self, sym: Rc<str>, proc: Ast) {
        self.add_core_binding(sym.clone().into());
        self.core_primitives.insert(sym, proc);
    }

    pub fn declare_core_top_level(&self, ns: &mut NameSpace) {
        ns.transformers.extend(
            self.core_forms
                .clone()
                .into_iter()
                .map(|(key, value)| (key.into(), CompileTimeBinding::CoreForm(value))),
        );
        ns.variables.extend(
            self.core_primitives
                .clone()
                .into_iter()
                .map(|(key, value)| (key.into(), value)),
        );
    }

    pub fn core_form_symbol(s: Ast) -> Result<Rc<str>, Error> {
        match_syntax!((id._id))(s)
            .map_err(std::convert::Into::into)
            .and_then(|f| {
                // could this also be a plain symbol?
                let Ast::Syntax(s) = f.id else {
                    return Err("no such pattern variable id".into());
                };
                let Ast::Symbol(ref sym) = s.0 else {
                    return Err("no such pattern variable id".into());
                };
                let b = Self::resolve(&s.with_ref(sym.clone()), false)?;
                match b {
                    Binding::Local(_) => Err(format!("{sym} is not a core form").into()),
                    Binding::TopLevel(s) => Ok(s),
                }
            })
    }
}
