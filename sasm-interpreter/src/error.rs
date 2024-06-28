use sasm_parse::ident::Identifier;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RuntimeError {
    #[error("Illegal jump to line {0}")]
    IllegalGoto(usize),
    #[error("Undefined variable '{0}'")]
    UndefinedVar(Identifier),
    #[error("Variable '{}' has already been defined", (.0).name())]
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
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Invalid number value: `{0}`")]
    IllegalNumber(String),
}
