use args::Arguments;
use args_sm::ArgParserStateMachine;
use error::ParseError;
use expression::{Expression, Number};
use ident::Identifier;
use instr_names::{ADD, CMP, DEC, DIE, DIV, DMP, INC, JEQ, JNE, MOV, MUL, POW, SUB, VAR};

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
    Decrement(Identifier),
    Dump(Expression),
    Add(Identifier, Expression),
    Subtract(Identifier, Expression),
    Multiply(Identifier, Expression),
    Divide(Identifier, Expression),
    Power(Identifier, Expression),
    Compare(Identifier, Expression),
    JumpEqual(Number),
    JumpNotEqual(Number),
    Die(Number),
}

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Self::CreateVariable(..) => VAR,
            Self::Move(..) => MOV,
            Self::Increment(..) => INC,
            Self::Decrement(..) => DEC,
            Self::Dump(..) => DMP,
            Self::Add(..) => ADD,
            Self::Subtract(..) => SUB,
            Self::Multiply(..) => MUL,
            Self::Divide(..) => DIV,
            Self::Power(..) => POW,
            Self::Compare(..) => CMP,
            Self::JumpEqual(..) => JEQ,
            Self::JumpNotEqual(..) => JNE,
            Self::Die(..) => DIE,
        }
        .into()
    }
}

impl TryFrom<&str> for Instruction {
    type Error = ParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (instr, maybe_args) = if value.len() == 3 {
            (value, "")
        } else {
            value
                .split_once(' ')
                .ok_or(Self::Error::MissingInstrArgsSep)?
        };

        let args: Arguments = ArgParserStateMachine::parse_args(maybe_args)?.into();

        match instr {
            VAR => {
                args.check_count_exact(1)?;
                let ident = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::CreateVariable(ident))
            }
            MOV => {
                args.check_count_exact(2)?;
                let dst = args.fetch_nth_as_ident(0).into_parse_err()?;
                let src = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Move(dst, src))
            }
            INC => {
                args.check_count_exact(1)?;
                let ident = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::Increment(ident))
            }
            DEC => {
                args.check_count_exact(1)?;
                let ident = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::Decrement(ident))
            }
            DMP => {
                args.check_count_exact(1)?;
                let expr = args.fetch_nth_as_any(0).into_parse_err()?;

                Ok(Self::Dump(expr))
            }
            ADD => {
                args.check_count_exact(1)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;
                let amount = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Add(var, amount))
            }
            SUB => {
                args.check_count_exact(1)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;
                let amount = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Subtract(var, amount))
            }
            MUL => {
                args.check_count_exact(2)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;
                let amount = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Multiply(var, amount))
            }
            DIV => {
                args.check_count_exact(2)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;
                let amount = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Divide(var, amount))
            }
            POW => {
                args.check_count_exact(2)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;
                let amount = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Power(var, amount))
            }
            CMP => {
                args.check_count_exact(2)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;
                let expr = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Compare(var, expr))
            }
            JNE => {
                args.check_count_exact(1)?;
                let amount = args.fetch_nth_as_number(0).into_parse_err()?;

                Ok(Self::JumpNotEqual(amount))
            }
            JEQ => {
                args.check_count_exact(1)?;
                let amount = args.fetch_nth_as_number(0).into_parse_err()?;

                Ok(Self::JumpEqual(amount))
            }
            DIE => {
                args.check_count(0, 1)?;
                let expr = args.fetch_nth_as_number(0).into_optional()?;

                Ok(Self::Die(expr.unwrap_or_default()))
            }
            other => Err(Self::Error::IllegalInstruction(other.into())),
        }
    }
}
