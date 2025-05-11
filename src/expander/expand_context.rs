use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

use crate::ast::scope::Scope;

use super::{binding::CompileTimeEnvoirnment, namespace::NameSpace};

#[derive(Clone, Debug)]
pub struct ExpandContext {
    pub(crate) use_site_scopes: Option<Rc<RefCell<BTreeSet<Scope>>>>,
    pub(crate) namespace: NameSpace,
    pub(crate) env: CompileTimeEnvoirnment,
    pub(crate) only_immediate: bool,
    pub(crate) post_expansion_scope: Option<Scope>,
}

impl ExpandContext {
    pub fn new(namespace: NameSpace) -> Self {
        Self {
            use_site_scopes: None,
            namespace,
            env: CompileTimeEnvoirnment::new(),
            only_immediate: false,
            post_expansion_scope: None,
        }
    }
}
