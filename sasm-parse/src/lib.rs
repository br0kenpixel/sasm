#![allow(
    clippy::module_name_repetitions,
    clippy::wildcard_imports,
    clippy::too_many_lines
)]

use args::Arguments;
use args_sm::ArgParserStateMachine;
use error::ParseError;
use expression::{Expression, Number, Text};
use ident::Identifier;
use instr_names::*;
use std::fmt::{self, Display};

mod args;
mod args_sm;
pub mod error;
pub mod expression;
pub mod ident;
mod instr_names;
pub mod type_trait;

/// An executable operation that can be executed by an interpreter.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instruction {
    /// Defines a variable with the given name.
    CreateVariable(Identifier),
    /// Moves a value to a variable.
    Move(Identifier, Expression),
    /// Increments a variable containing a number.
    Increment(Identifier),
    /// Decrements a variable containing a number.
    Decrement(Identifier),
    /// Dumps the value of the given variable to `stdout`.
    Dump(Expression),
    /// Performes mathematical addition on the given variable with the given operand.
    Add(Identifier, Expression),
    /// Performes mathematical subtraction on the given variable with the given [subtrahend](https://www.dictionary.com/browse/subtrahend).
    Subtract(Identifier, Expression),
    /// Performes mathematical multiplication on the given variable with the given multiplier.
    Multiply(Identifier, Expression),
    /// Performes mathematical division on the given variable with the given divisor.
    Divide(Identifier, Expression),
    /// Performes mathematical exponentiation on the given variable with the given exponent.
    Power(Identifier, Expression),
    /// Compares the value inside the given variable with an expression (possibly another variable).
    /// The result of this comparion is saved by the interpreter into some internal variable.
    Compare(Identifier, Expression),
    /// Skips a given number of instructions if the last comparison was `true`.
    JumpEqual(Number),
    /// Skips a given number of instructions if the last comparison was `false`.
    JumpNotEqual(Number),
    /// Skips a given number of instructions.
    Jump(Number),
    /// Reads a number from `stdin` and saves it into the given variable.
    ReadNumericValue(Identifier),
    /// Reads a line from `stdin` and saves it into the given variable.
    ReadStringValue(Identifier),
    /// Writes a random number into the given variable. Optionally, a _minimum_ and _maximum_ range can be specified.
    GenerateRandomNumber(Identifier, Option<Expression>, Option<Expression>),
    /// Pushes a string (or a string inside another variable) into the given variable.
    Push(Identifier, Expression),
    /// Pops a single character from a string inside the given variable.
    /// Optionally you can specify another variable, which will contain the popped character as a single character string.
    Pop(Identifier, Option<Identifier>),
    /// Writes a formatted string into a varible. The format string uses the same syntax as Rust's [`format!()`].
    Format(Identifier, Text),
    /// Writes an expression to `stdout` **without newline**.
    Print(Expression),
    /// Resets a variable's value to it's default.
    /// For numbers, it just sets them back to 0.
    /// For strings, it clears them - turning them into an empty string.
    Clear(Identifier),
    /// Calculates the length of an array-like object (eg. strings) and saves it into the given variable.
    Length(Identifier, Expression),
    /// Stops execution for a given amount of time _(milliseconds)_.
    Sleep(Expression),
    /// Delete the variable and deallocate the contained data.
    Delete(Identifier),
    /// Exits the program with the given exit code.
    Die(Number),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateVariable(..) => write!(f, "{VAR}"),
            Self::Move(..) => write!(f, "{MOV}"),
            Self::Increment(..) => write!(f, "{INC}"),
            Self::Decrement(..) => write!(f, "{DEC}"),
            Self::Dump(..) => write!(f, "{DMP}"),
            Self::Add(..) => write!(f, "{ADD}"),
            Self::Subtract(..) => write!(f, "{SUB}"),
            Self::Multiply(..) => write!(f, "{MUL}"),
            Self::Divide(..) => write!(f, "{DIV}"),
            Self::Power(..) => write!(f, "{POW}"),
            Self::Compare(..) => write!(f, "{CMP}"),
            Self::JumpEqual(..) => write!(f, "{JEQ}"),
            Self::JumpNotEqual(..) => write!(f, "{JNE}"),
            Self::Jump(..) => write!(f, "{JMP}"),
            Self::ReadNumericValue(..) => write!(f, "{RNV}"),
            Self::ReadStringValue(..) => write!(f, "{RSV}"),
            Self::GenerateRandomNumber(..) => write!(f, "{RNG}"),
            Self::Push(..) => write!(f, "{PSH}"),
            Self::Pop(..) => write!(f, "{POP}"),
            Self::Format(..) => write!(f, "{FMT}"),
            Self::Print(..) => write!(f, "{SAY}"),
            Self::Length(..) => write!(f, "{LEN}"),
            Self::Clear(..) => write!(f, "{CLR}"),
            Self::Sleep(..) => write!(f, "{HLT}"),
            Self::Delete(..) => write!(f, "{DEL}"),
            Self::Die(..) => write!(f, "{DIE}"),
        }
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
                args.check_count_exact(2)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;
                let amount = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Add(var, amount))
            }
            SUB => {
                args.check_count_exact(2)?;
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
            JMP => {
                args.check_count_exact(1)?;
                let amount = args.fetch_nth_as_number(0).into_parse_err()?;

                Ok(Self::Jump(amount))
            }
            RNV => {
                args.check_count_exact(1)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::ReadNumericValue(var))
            }
            RSV => {
                args.check_count_exact(1)?;
                let var = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::ReadStringValue(var))
            }
            RNG => {
                args.check_count(1, 3)?;

                let var = args.fetch_nth_as_ident(0).into_parse_err()?;

                if args.len() == 1 {
                    return Ok(Self::GenerateRandomNumber(var, None, None));
                }

                let range_min = args.fetch_nth_as_any(1).into_parse_err()?;
                let range_max = args.fetch_nth_as_any(2).into_parse_err()?;

                Ok(Self::GenerateRandomNumber(
                    var,
                    Some(range_min),
                    Some(range_max),
                ))
            }
            PSH => {
                args.check_count_exact(2)?;
                let dst = args.fetch_nth_as_ident(0).into_parse_err()?;
                let src = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Push(dst, src))
            }
            POP => {
                args.check_count(1, 2)?;
                let what = args.fetch_nth_as_ident(0).into_parse_err()?;
                let pop_where = args.fetch_nth_as_ident(1).into_optional()?;

                Ok(Self::Pop(what, pop_where))
            }
            FMT => {
                args.check_count_exact(2)?;
                let dst = args.fetch_nth_as_ident(0).into_parse_err()?;
                let fmt_text = args.fetch_nth::<Text>(1)?;

                Ok(Self::Format(dst, fmt_text))
            }
            SAY => {
                args.check_count_exact(1)?;
                let what = args.fetch_nth_as_any(0).into_parse_err()?;

                Ok(Self::Print(what))
            }
            LEN => {
                args.check_count_exact(2)?;
                let dst = args.fetch_nth_as_ident(0).into_parse_err()?;
                let what = args.fetch_nth_as_any(1).into_parse_err()?;

                Ok(Self::Length(dst, what))
            }
            CLR => {
                args.check_count_exact(1)?;
                let what = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::Clear(what))
            }
            HLT => {
                args.check_count_exact(1)?;
                let time = args.fetch_nth_as_any(0).into_parse_err()?;

                Ok(Self::Sleep(time))
            }
            DEL => {
                args.check_count_exact(1)?;
                let what = args.fetch_nth_as_ident(0).into_parse_err()?;

                Ok(Self::Delete(what))
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
