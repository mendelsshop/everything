use crate::ast::{syntax::Syntax, Ast, Symbol};

#[derive(Debug)]
pub enum Error {
    OutOfContext(OutOfContext),
    MissingTransformer(MissingTransformer),
    Other(String),
    FreeVariable(FreeVariable),
    AmbiguousBinding(AmbiguousBinding),
    DuplicateBinding(DuplicateBinding),
    NonSyntaxTransformer(NonSyntaxTransformer),
    ExpectedSingleValue(ExpectedSingleValue),
    BindInEmptyScopeSet(BindInEmptyScopeSet),
    IllegalUseOfSyntax(IllegalUseOfSyntax),
    BeginNonExpression(BeginNonExpression),
}

impl From<BeginNonExpression> for Error {
    fn from(v: BeginNonExpression) -> Self {
        Self::BeginNonExpression(v)
    }
}

impl From<IllegalUseOfSyntax> for Error {
    fn from(v: IllegalUseOfSyntax) -> Self {
        Self::IllegalUseOfSyntax(v)
    }
}
#[derive(Debug)]
pub struct IllegalUseOfSyntax(pub Ast);
impl From<BindInEmptyScopeSet> for Error {
    fn from(v: BindInEmptyScopeSet) -> Self {
        Self::BindInEmptyScopeSet(v)
    }
}

impl From<ExpectedSingleValue> for Error {
    fn from(v: ExpectedSingleValue) -> Self {
        Self::ExpectedSingleValue(v)
    }
}

impl From<NonSyntaxTransformer> for Error {
    fn from(v: NonSyntaxTransformer) -> Self {
        Self::NonSyntaxTransformer(v)
    }
}

impl From<DuplicateBinding> for Error {
    fn from(v: DuplicateBinding) -> Self {
        Self::DuplicateBinding(v)
    }
}

impl From<AmbiguousBinding> for Error {
    fn from(v: AmbiguousBinding) -> Self {
        Self::AmbiguousBinding(v)
    }
}

impl<T: ToString> From<T> for Error {
    fn from(v: T) -> Self {
        Self::Other(v.to_string())
    }
}

impl From<OutOfContext> for Error {
    fn from(v: OutOfContext) -> Self {
        Self::OutOfContext(v)
    }
}

impl From<MissingTransformer> for Error {
    fn from(v: MissingTransformer) -> Self {
        Self::MissingTransformer(v)
    }
}

#[derive(Debug)]
pub struct NonSyntaxTransformer(pub Ast);
#[derive(Debug)]
pub struct BeginNonExpression(pub Ast);
#[derive(Debug)]
pub struct OutOfContext(pub String);
#[derive(Debug)]
pub struct MissingTransformer(pub Symbol);
#[derive(Debug)]
pub struct FreeVariable(pub Syntax<Symbol>);
#[derive(Debug)]
pub struct AmbiguousBinding(pub Syntax<Symbol>);
#[derive(Debug)]
pub struct DuplicateBinding(pub Syntax<Symbol>);
#[derive(Debug)]
pub struct ExpectedSingleValue();
#[derive(Debug)]
pub struct BindInEmptyScopeSet(pub Symbol);
