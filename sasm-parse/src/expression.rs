use crate::{error::ParseError, ident::Identifier};
use sasm_lang_core::obj_type::SasmObject;
use std::{
    any::Any,
    fmt::{self, Display},
    mem,
};

pub type Number = i64;
pub type Text = String;
pub type Float = f32;

/// An expression.
#[derive(Debug)]
pub enum Expression {
    /// An object literal
    ObjectLiteral(SasmObject),
    /// An identifier
    Identifier(Identifier),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "Identifier<'{}'>", ident.name()),
            Self::ObjectLiteral(obj) => {
                write!(f, "{}", obj.to_string().unwrap_or_else(|| obj.repr()))
            }
        }
    }
}

impl Expression {
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
            Self::ObjectLiteral(obj) => obj.kind().name(),
        }
    }

    #[must_use]
    pub fn singe_char_string(ch: char) -> Self {
        Self::ObjectLiteral(ch.to_string().as_str().into())
    }

    #[must_use]
    pub fn inner_as_any(self) -> Box<dyn Any> {
        match self {
            Self::Identifier(ident) => Box::new(ident),
            Self::ObjectLiteral(obj) => Box::new(obj),
        }
    }
}

impl TryFrom<&str> for Expression {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(value) = value.parse::<Number>() {
            return Ok(Self::Number(value));
        }

        if let Ok(value) = value.parse::<Float>() {
            return Ok(Self::Float(value));
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
