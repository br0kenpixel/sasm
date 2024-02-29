use strum::{Display, EnumString};

#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString)]
pub enum Instruction {
    #[strum(to_string = "VAR")]
    CreateVariable,
    Dump,
}
