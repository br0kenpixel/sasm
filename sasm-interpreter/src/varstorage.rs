use crate::error::RuntimeError;
use sasm_parse::{expression::Expression, ident::Identifier};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct VariableStorage(HashMap<Identifier, Option<Expression>>);

impl VariableStorage {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn create(&mut self, ident: &Identifier) -> Result<(), RuntimeError> {
        if self.get(ident).is_ok() {
            return Err(RuntimeError::DuplicateVarDef(ident.clone()));
        }

        self.0.insert(ident.clone(), None);
        Ok(())
    }

    pub fn get(&self, ident: &Identifier) -> Result<Option<&Expression>, RuntimeError> {
        let Some(maybe) = self.0.get(ident) else {
            return Err(RuntimeError::UndefinedVar(ident.clone()));
        };

        let Some(inner) = maybe else {
            return Ok(None);
        };

        Ok(Some(inner))
    }

    pub fn get_nonnull(&self, ident: &Identifier) -> Result<&Expression, RuntimeError> {
        let Some(value) = self.get(ident)? else {
            return Err(RuntimeError::NullDeref);
        };

        Ok(value)
    }

    pub fn set(&mut self, ident: &Identifier, value: Expression) -> Result<(), RuntimeError> {
        if ident.is_internal() {
            return Err(RuntimeError::IllegalWriteInternal(ident.clone()));
        }

        if self.get(ident).is_ok() {
            self.replace(ident, value)
        } else {
            self.insert(ident, value);
            Ok(())
        }
    }

    pub fn delete(&mut self, ident: &Identifier) -> Result<(), RuntimeError> {
        let _ = self.get(ident)?;
        self.0.remove(ident);

        Ok(())
    }

    pub fn set_internal(&mut self, name: &'static str, value: Expression) {
        let ident = Identifier::try_from(format!("_{name}").as_str()).unwrap();

        assert!(
            value.type_name() != Expression::IDENT_TYPE_NAME,
            "internal variable cannot be of type `Identifier`"
        );

        self.insert(&ident, value);
    }

    fn replace(&mut self, ident: &Identifier, value: Expression) -> Result<(), RuntimeError> {
        let current = self.get(ident).unwrap();

        if current.is_some_and(|inner| !inner.cmp_type(&value)) {
            return Err(RuntimeError::MismatchedTypes {
                got: value.type_name(),
                expected: current.unwrap().type_name(),
            });
        }

        self.0.insert(ident.clone(), Some(value));
        Ok(())
    }

    fn insert(&mut self, ident: &Identifier, value: Expression) {
        self.0.insert(ident.clone(), Some(value));
    }
}
