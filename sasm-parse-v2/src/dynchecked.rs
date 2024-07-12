use sasm_lang_core::{obj_type::SasmObject, typetrait::SasmType};
use std::{any::type_name, marker::PhantomData, ops::Deref};

/// A wrapper that can only be created if a dynamically typed object's type
/// matches `T`, effectively allowing runtime type checking.
#[derive(Debug)]
pub struct DynamicallyCheckedType<T: SasmType> {
    inner: SasmObject,
    _pd: PhantomData<T>,
}

impl<T: SasmType + 'static> DynamicallyCheckedType<T> {
    pub fn value(&self) -> &T {
        // SAFETY: Value will always be of type T.
        unsafe { self.inner.expect::<T>().unwrap_unchecked() }
    }
}

impl<T: SasmType + 'static> TryFrom<SasmObject> for DynamicallyCheckedType<T> {
    type Error = crate::error::ParseError;

    fn try_from(value: SasmObject) -> Result<Self, Self::Error> {
        match value.expect::<T>() {
            Some(_) => Ok(Self {
                inner: value,
                _pd: PhantomData,
            }),
            None => Err(Self::Error::MismatchedTypes {
                got: value.kind().name().into(),
                expected: type_name::<T>().into(),
            }),
        }
    }
}

impl<T: SasmType> Deref for DynamicallyCheckedType<T> {
    type Target = SasmObject;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
