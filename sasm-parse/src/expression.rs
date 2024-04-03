use crate::{error::ParseError, ident::Identifier};
use std::{any::Any, mem};

pub type Number = i64;
pub type Text = String;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expression {
    Number(Number),
    String(Text),
    Identifier(Identifier),
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        let mut buf = self.type_name().to_string();

        match self {
            Self::Identifier(ident) => buf.push_str(&format!("<'{}'>", ident.name())),
            Self::Number(num) => buf.push_str(&format!("<{num}>")),
            Self::String(text) => buf.push_str(&format!("<'{text}'>")),
        }

        buf
    }
}

impl Expression {
    pub const NUMBER_TYPE_NAME: &'static str = "Number";
    pub const STRING_TYPE_NAME: &'static str = "String";
    pub const IDENT_TYPE_NAME: &'static str = "Identifier";

    #[must_use]
    pub fn into_ident(self) -> Option<Identifier> {
        if let Self::Identifier(ident) = self {
            return Some(ident);
        }

        None
    }

    #[must_use]
    pub fn cmp_type(&self, rhs: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(rhs)
    }

    #[must_use]
    pub const fn type_name(&self) -> &'static str {
        match self {
            Self::Identifier(..) => Self::IDENT_TYPE_NAME,
            Self::Number(..) => Self::NUMBER_TYPE_NAME,
            Self::String(..) => Self::STRING_TYPE_NAME,
        }
    }

    #[must_use]
    pub fn singe_char_string(ch: char) -> Self {
        Self::String(ch.into())
    }

    #[must_use]
    pub fn inner_as_any(self) -> Box<dyn Any> {
        match self {
            Self::Identifier(ident) => Box::new(ident),
            Self::Number(num) => Box::new(num),
            Self::String(string) => Box::new(string),
        }
    }
}

impl TryFrom<&str> for Expression {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(value) = value.parse::<Number>() {
            return Ok(Self::Number(value));
        }

        if (value.starts_with('\'') && value.ends_with('\''))
            || (value.starts_with('\"') && value.ends_with('\"'))
        {
            return Ok(Self::String(value[1..value.len() - 1].into()));
        }

        if let Ok(ident) = Identifier::try_from(value) {
            return Ok(Self::Identifier(ident));
        }

        Err(Self::Error::IllegalExpression(value.into()))
    }
}
