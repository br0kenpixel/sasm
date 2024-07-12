use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid identifier: {0}")]
    IllegalIdentifier(String),
    #[error("Invalid expression: {0}")]
    IllegalExpression(String),
    #[error("Invalid instruction: {0}")]
    IllegalInstruction(String),
    #[error("Missing separator for instruction-args")]
    MissingInstrArgsSep,
    #[error("Missing required argument")]
    MissingArg,
    #[error("Expected value of type {expected}, got {got}")]
    MismatchedTypes { got: String, expected: String },
    #[error("Missing end quotes for string expression")]
    MissingStringEndQuote,
    #[error("Expected at least {expected} arguments, got {got}")]
    NotEnoughArgs { got: usize, expected: usize },
    #[error("Expected at most {expected} arguments, got {got}")]
    TooManyArgs { got: usize, expected: usize },
    #[error("Unexpected token: `{0}`")]
    UnexpectedToken(char),
}
