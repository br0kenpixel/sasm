pub type Number = i64;
pub type Float = f32;
pub type Text = String;
pub type Char = char;
pub type Null = ();

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SasmObjectType {
    name: &'static str,
}

impl SasmObjectType {
    pub const NUMBER: Self = Self { name: "Number" };
    pub const TEXT: Self = Self { name: "String" };
    pub const CHAR: Self = Self { name: "Char" };
    pub const FLOAT: Self = Self { name: "Float" };
    pub const NULL: Self = Self { name: "NULL" };

    pub const fn name(&self) -> &'static str {
        self.name
    }
}
