use crate::{
    expression::{Expression, Number, Text},
    ident::Identifier,
};

pub trait SasmType {
    #[must_use]
    fn type_name() -> &'static str;
}

impl SasmType for Number {
    fn type_name() -> &'static str {
        Expression::NUMBER_TYPE_NAME
    }
}

impl SasmType for Text {
    fn type_name() -> &'static str {
        Expression::STRING_TYPE_NAME
    }
}

impl SasmType for Identifier {
    fn type_name() -> &'static str {
        Expression::IDENT_TYPE_NAME
    }
}
