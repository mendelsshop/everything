use std::collections::HashMap;

use crate::ast::{Ast, Symbol};

use super::binding::CompileTimeBinding;

// TODO: this might need to be recursive
// TODO: this might have to be behind a an rc/refcell so that it can be part of the lambda ast for
// eval
pub struct NameSpace {
    pub variables: HashMap<Symbol, Ast>,
    pub transformers: HashMap<Symbol, CompileTimeBinding>,
}
