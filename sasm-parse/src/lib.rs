use crate::args_sm::ArgParserStateMachine;
use args::Arguments;
use error::ParseError;
use instr::Instruction;

pub mod args;
pub mod args_sm;
pub mod error;
pub mod expression;
pub mod ident;
pub mod instr;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Command {
    instr: Instruction,
    args: Arguments,
}

impl Command {
    pub const fn instr(&self) -> Instruction {
        self.instr
    }

    pub const fn args(&self) -> &Arguments {
        &self.args
    }
}

impl TryFrom<&str> for Command {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (instr, maybe_args) = split_raw_input(value)?;

        let instruction = Instruction::try_from(instr)
            .map_err(|_| Self::Error::IllegalInstruction(instr.into()))?;

        let args = Arguments::from(ArgParserStateMachine::parse_args(maybe_args)?);

        Ok(Self {
            instr: instruction,
            args,
        })
    }
}

fn split_raw_input(value: &str) -> Result<(&str, &str), ParseError> {
    if value.len() == 3 {
        return Ok((value, ""));
    }

    value.split_once(' ').ok_or(ParseError::MissingInstrArgsSep)
}
