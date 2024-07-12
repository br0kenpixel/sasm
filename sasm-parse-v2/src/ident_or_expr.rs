use crate::ident::Identifier;
use sasm_lang_core::obj_type::SasmObject;

#[derive(Debug)]
pub enum IdentOrExpr {
    Identifier(Identifier),
    ObjectLiteral(SasmObject),
}
