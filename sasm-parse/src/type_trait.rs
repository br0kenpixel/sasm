use crate::{
    expression::{Expression, Float, Number, Text},
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

impl SasmType for Float {
    fn type_name() -> &'static str {
        Expression::FLOAT_TYPE_NAME
    }
}
