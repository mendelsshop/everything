use std::collections::HashMap;

use crate::ast::{syntax::Syntax, Ast, Symbol};

pub type DuplicateMap = HashMap<Symbol, Vec<Syntax<Symbol>>>;

pub const make_check_no_duplicate_table: fn() -> DuplicateMap = DuplicateMap::new;
pub fn check_no_duplicate_ids(
    ids: Vec<Syntax<Symbol>>,
    s: Ast,
    ht: DuplicateMap,
) -> Result<DuplicateMap, String> {
    ids.into_iter().try_fold(ht, |mut ht, id| {
        let this = ht.get_mut(&id.0);
        match this {
            Some(ids) => {
                if ids.iter().any(|id| id.bound_identifier(id)) {
                    Err(format!("duplicate binding: {id:?}"))
                } else {
                    ids.push(id);
                    Ok(ht)
                }
            }
            None => {
                ht.insert(id.0.clone(), vec![id]);
                Ok(ht)
            }
        }
    })
}
