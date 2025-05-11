use crate::ast::Ast;

use super::{expand_context::ExpandContext, Expander};

impl Expander {
    pub fn core_form_define_values(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        Err(format!("not allowed in an expression postion: {s} "))
    }
    pub fn core_form_define_syntaxes(&mut self, s: Ast, ctx: ExpandContext) -> Result<Ast, String> {
        Err(format!("not allowed in an expression postion: {s} "))
    }
}
