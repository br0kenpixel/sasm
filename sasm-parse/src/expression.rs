use crate::{error::ParseError, ident::Identifier};
use std::{
    any::Any,
    fmt::{self, Display},
    mem,
    rc::Rc,
};

/// A 64-bit signed integer [`i64`].
pub type Number = i64;
/// A dynamically-allocated string of characters that can be cheaply cloned.
pub type Text = Rc<String>;
/// A 32-bit floating point number [`f32`].
pub type Float = f32;

/// An expression.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    /// A 64-bit signed integer [`i64`].
    Number(Number),
    /// A dynamically-allocated string of characters.
    String(Text),
    /// A 32-bit floating point number [`f32`].
    Float(Float),
    /// An identifier
    Identifier(Identifier),
}

impl Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_name())?;

        match self {
            Self::Identifier(ident) => write!(f, "<'{}'>", ident.name()),
            Self::Number(num) => write!(f, "<{num}>"),
            Self::String(text) => write!(f, "<'{text}'>"),
            Self::Float(val) => write!(f, "<{val}>"),
        }
    }
}

impl Expression {
    pub const NUMBER_TYPE_NAME: &'static str = "Number";
    pub const FLOAT_TYPE_NAME: &'static str = "Number";
    pub const STRING_TYPE_NAME: &'static str = "String";
    pub const IDENT_TYPE_NAME: &'static str = "Identifier";

    pub fn make_string<S: AsRef<str>>(obj: S) -> Self {
        Self::String(Rc::new(obj.as_ref().to_string()))
    }

    pub fn empty_string() -> Self {
        Self::String(Rc::new(String::new()))
    }

    pub const fn zero() -> Self {
        Self::Number(0)
    }

    pub const fn zero_float() -> Self {
        Self::Float(0.0)
    }

    #[must_use]
    pub fn rewrap_string(rcs: Text) -> Self {
        Self::String(rcs)
    }

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
            Self::Float(..) => Self::FLOAT_TYPE_NAME,
        }
    }

    #[must_use]
    pub fn singe_char_string(ch: char) -> Self {
        Self::String(Rc::new(ch.into()))
    }

    #[must_use]
    pub fn inner_as_any(self) -> Box<dyn Any> {
        match self {
            Self::Identifier(ident) => Box::new(ident),
            Self::Number(num) => Box::new(num),
            Self::String(string) => Box::new(string),
            Self::Float(val) => Box::new(val),
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
            let string_content = &value[1..value.len() - 1];
            return Ok(Self::make_string(string_content));
        }

        if let Ok(ident) = Identifier::try_from(value) {
            return Ok(Self::Identifier(ident));
        }

        Err(Self::Error::IllegalExpression(value.into()))
    }
}
