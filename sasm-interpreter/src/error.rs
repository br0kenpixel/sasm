use sasm_parse::ident::Identifier;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Illegal jump to line {0}")]
    IllegalGoto(usize),
    #[error("Undefined variable {0:?}")]
    UndefinedVar(Identifier),
    #[error("Duplicate variable definitions for '{0:?}'")]
    DuplicateVarDef(Identifier),
    #[error("Variable does not have a value")]
    NullDeref,
    #[error("Only values of type 'Number' can be incremented")]
    IllegalIncrement,
    #[error("Expected expression of type '{expected}', got '{got}'")]
    MismatchedTypes {
        got: &'static str,
        expected: &'static str,
    },
}
