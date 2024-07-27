use crate::error::ParseError;
use std::{fmt::Display, rc::Rc};

/// An identifier (variable name).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(Rc<String>);

impl Identifier {
    #[cfg(test)]
    pub(crate) fn new<S: AsRef<str>>(name: S) -> Self {
        Self(Rc::new(name.as_ref().into()))
    }

    /// Returns the identifier name as a string slice.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn is_internal(&self) -> bool {
        self.0.starts_with('_')
    }
}

impl TryFrom<&str> for Identifier {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if !value
            .chars()
            .all(|ch| ch.is_ascii_alphabetic() || ch == '_')
        {
            return Err(Self::Error::IllegalIdentifier(value.into()));
        }

        Ok(Self(Rc::new(value.into())))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
