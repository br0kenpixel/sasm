use crate::error::ParseError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier(String);

impl Identifier {
    #[cfg(test)]
    pub(crate) fn new<S: AsRef<str>>(name: S) -> Self {
        Self(name.as_ref().into())
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.0
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

        Ok(Self(value.into()))
    }
}
