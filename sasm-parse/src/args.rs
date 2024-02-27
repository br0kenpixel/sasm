use crate::{error::ParseError, expression::Expression, ident::Identifier};

pub enum ArgFetchResult<T> {
    Found(T),
    Missing,
    InvalidType { got: String, expected: String },
}

pub struct Arguments(Vec<Expression>);

impl Arguments {
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
}

impl From<Vec<Expression>> for Arguments {
    fn from(value: Vec<Expression>) -> Self {
        Self(value)
    }
}
