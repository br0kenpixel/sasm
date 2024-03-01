use strum::{Display, EnumString};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString)]
pub enum Instruction {
    #[strum(to_string = "VAR")]
    CreateVariable,
    #[strum(to_string = "MOV")]
    Move,
    #[strum(to_string = "INC")]
    Increment,
    #[strum(to_string = "DEC")]
    Decrement,
    #[strum(to_string = "DMP")]
    Dump,
    #[strum(to_string = "ADD")]
    Add,
    #[strum(to_string = "SUB")]
    Subtract,
    #[strum(to_string = "MUL")]
    Multiply,
    #[strum(to_string = "DIV")]
    Divide,
    #[strum(to_string = "POW")]
    Power,
    #[strum(to_string = "CMP")]
    Compare,
    #[strum(to_string = "JEQ")]
    JumpEqual,
    #[strum(to_string = "JNE")]
    JumpNotEqual,
    #[strum(to_string = "JMP")]
    Jump,
    #[strum(to_string = "DIE")]
    Die,
}
