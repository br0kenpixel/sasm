use args::Arguments;
use args_sm::ArgParserStateMachine;
use error::ParseError;
use expression::Expression;
use ident::Identifier;
use instr_names::{DMP, INC, MOV, VAR};

mod args;
mod args_sm;
pub mod error;
pub mod expression;
pub mod ident;
mod instr_names;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    CreateVariable(Identifier),
    Move(Identifier, Expression),
    Increment(Identifier),
    Dump(Expression),
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Self::CreateVariable(..) => VAR,
            Self::Move(..) => MOV,
            Self::Increment(..) => INC,
            Self::Dump(..) => DMP,
        }
        .into()
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let Some((instr, maybe_args)) = value.split_once(' ') else {
            return Err(Self::Error::MissingInstrArgsSep);
        };

        let args: Arguments = ArgParserStateMachine::parse_args(maybe_args)?.into();

        match instr {
            VAR => {
                let ident = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::CreateVariable(ident))
            }
            MOV => {
                let dst = args.fetch_nth_as_ident(0).into_parse_err()?;
                let src = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Move(dst, src))
            }
            INC => {
                let ident = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::Increment(ident))
            }
            DMP => {
                let expr = args.fetch_nth_as_any(0).into_parse_err()?;

                Ok(Self::Dump(expr))
            }
            other => Err(Self::Error::IllegalInstruction(other.into())),
        }
    }
}
