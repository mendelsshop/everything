use crate::ast::{syntax::Syntax, Symbol};

pub enum Error {
    Expand(ExpandError),
    Evaluator(EvaluatorError),
}
pub enum EvaluatorError {
    Other(String),
    ExpectedSingleValue(ExpectedSingleValue),
}
pub enum ExpandError {
    OutOfContext(OutOfContext),
    MissingTransformer(MissingTransformer),
    Other(String),
    FreeVariable(FreeVariable),
    AmbiguousBinding(AmbiguousBinding),
    DuplicateBinding(DuplicateBinding),
}
pub struct OutOfContext(pub String);
pub struct MissingTransformer(pub Symbol);
pub struct FreeVariable(pub Syntax<Symbol>);
pub struct AmbiguousBinding(pub Syntax<Symbol>);
pub struct DuplicateBinding(pub Syntax<Symbol>);
pub struct ExpectedSingleValue();
