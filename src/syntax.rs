use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug,
    hash::Hash,
};

use crate::{Ast, Binding, Scope, Symbol};

type ScopeSet = BTreeSet<Scope>;
#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct Syntax<T: Clone + PartialEq + Debug>(pub T, pub ScopeSet);

// impl<T: Clone + PartialEq + Debug + Hash> Hash for Syntax<T> {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//         self.1.hash(state);
//     }
// }
const EMPTY_SCOPE: BTreeSet<Scope> = ScopeSet::new();
const EMPTY_SYNTAX: Syntax<Ast> = Syntax(Ast::Boolean(false), EMPTY_SCOPE);

impl Ast {
    pub fn identifier(&self) -> bool {
        matches!( self, Ast::Syntax(s) if  matches!(**s,Syntax(Ast::Symbol(_), _)))
    }

    pub fn syntax_to_datum(self) -> Self {
        match self {
            Self::Syntax(s) => s.0,
            Self::List(l) => Self::List(l.into_iter().map(Self::syntax_to_datum).collect()),
            _ => self,
        }
    }

    pub fn datum_to_syntax(self, scopes: Option<ScopeSet>) -> Self {
        let wrap = |e| Self::Syntax(Box::new(Syntax(e, scopes.clone().unwrap_or_default())));
        match self {
            Self::Syntax(_) => self,
            Self::List(l) => wrap(Self::List(
                l.into_iter()
                    .map(|e| e.datum_to_syntax(scopes.clone()))
                    .collect(),
            )),
            _ => wrap(self),
        }
    }
}
impl<T: Clone + Debug + PartialEq> Syntax<T> {
    #[must_use]
    pub fn new(expr: T) -> Self {
        Self(expr, BTreeSet::new())
    }
    fn bound_identifier(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

