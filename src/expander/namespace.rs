use std::collections::HashMap;

use crate::ast::{Ast, Symbol};

use super::binding::CompileTimeBinding;

// Currently just keeps top level bindings + transformers
#[derive(Default, Clone)]
pub struct NameSpace {
    pub variables: HashMap<Symbol, Ast>,
    pub transformers: HashMap<Symbol, CompileTimeBinding>,
}
