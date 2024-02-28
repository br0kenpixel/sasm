use crate::{
    error::ParseError,
    expression::{Expression, Number},
    ident::Identifier,
};

pub enum ArgFetchResult<T> {
    Found(T),
    Missing,
    InvalidType { got: String, expected: String },
}

pub struct Arguments(Vec<Expression>);

impl Arguments {
    pub fn check_count(&self, min: usize, max: usize) -> Result<(), ParseError> {
        let argc = self.0.len();

        if argc < min {
            return Err(ParseError::NotEnoughArgs {
                got: argc,
                expected: min,
            });
        }

        if argc > max {
            return Err(ParseError::TooManyArgs {
                got: argc,
                expected: max,
            });
        }

        Ok(())
    }

    pub fn check_count_exact(&self, n: usize) -> Result<(), ParseError> {
        self.check_count(n, n)
    }

    pub fn fetch_nth_as_ident(&self, n: usize) -> ArgFetchResult<Identifier> {
        let Some(expr) = self.0.get(n) else {
            return ArgFetchResult::Missing;
        };

        let Some(ident) = expr.clone().into_ident() else {
            return ArgFetchResult::InvalidType {
                got: expr.to_string(),
                expected: "Identifier".into(),
            };
        };

        ArgFetchResult::Found(ident)
    }

    pub fn fetch_nth_as_any(&self, n: usize) -> ArgFetchResult<Expression> {
        if let Some(expr) = self.0.get(n) {
            return ArgFetchResult::Found(expr.clone());
        }

        ArgFetchResult::Missing
    }

    pub fn fetch_nth_as_number(&self, n: usize) -> ArgFetchResult<Number> {
        match self.fetch_nth_as_any(n) {
            ArgFetchResult::Missing => ArgFetchResult::Missing,
            ArgFetchResult::InvalidType { got, expected } => {
                ArgFetchResult::InvalidType { got, expected }
            }
            ArgFetchResult::Found(Expression::Number(n)) => ArgFetchResult::Found(n),
            ArgFetchResult::Found(invalid) => ArgFetchResult::InvalidType {
                got: invalid.to_string(),
                expected: "Number".into(),
            },
        }
    }
}

impl<T> ArgFetchResult<T> {
    pub fn into_parse_err(self) -> Result<T, ParseError> {
        match self {
            Self::Found(value) => Ok(value),
            Self::Missing => Err(ParseError::MissingArg),
            Self::InvalidType { got, expected } => {
                Err(ParseError::MismatchedTypes { got, expected })
            }
        }
    }

    pub fn into_optional(self) -> Result<Option<T>, ParseError> {
        match self {
            Self::Missing => Ok(None),
            Self::InvalidType { got, expected } => {
                Err(ParseError::MismatchedTypes { got, expected })
            }
            Self::Found(obj) => Ok(Some(obj)),
        }
    }
}

impl From<Vec<Expression>> for Arguments {
    fn from(value: Vec<Expression>) -> Self {
        Self(value)
    }
}
