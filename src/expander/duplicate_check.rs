use std::collections::HashMap;

use crate::{
    ast::{syntax::Syntax, Ast, Symbol},
    error::{DuplicateBinding, Error},
};

pub type DuplicateMap = HashMap<Symbol, Vec<Syntax<Symbol>>>;

#[allow(non_upper_case_globals)]
pub const make_check_no_duplicate_table: fn() -> DuplicateMap = DuplicateMap::new;
pub fn check_no_duplicate_ids(
    ids: Vec<Syntax<Symbol>>,
    _s: &Ast,
    ht: DuplicateMap,
) -> Result<DuplicateMap, Error> {
    ids.into_iter().try_fold(ht, |mut ht, id| {
        if let Some(ids) = ht.get_mut(&id.0) {
            if ids.iter().any(|id| id.bound_identifier(id)) {
                Err(Error::DuplicateBinding(DuplicateBinding(id.clone())))
            } else {
                ids.push(id);
                Ok(ht)
            }
        } else {
            ht.insert(id.0.clone(), vec![id]);
            Ok(ht)
        }
    })
}
