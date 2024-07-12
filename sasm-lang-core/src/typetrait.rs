use crate::object::{Char, Float, Number, Text};

pub trait SasmType {}

impl SasmType for Number {}
impl SasmType for Text {}
impl SasmType for Float {}
impl SasmType for Char {}
