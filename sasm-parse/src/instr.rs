use strum::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumString)]
pub enum Instruction {
    #[strum(serialize = "VAR")]
    CreateVariable,
    Dump,
}
