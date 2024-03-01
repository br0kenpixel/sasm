use sasm_parse::{args::ArgFetchResult, error::ParseError, ident::Identifier};
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
    #[error("This operation requires a variable of type 'Number'")]
    IllegalMathOp,
    #[error("Expected expression of type '{expected}', got '{got}'")]
    MismatchedTypes {
        got: &'static str,
        expected: &'static str,
    },
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Failed to parse command: {0}")]
    ParseError(#[from] ParseError),
}
