use dynchecked::DynamicallyCheckedType;
use ident::Identifier;
use ident_or_expr::IdentOrExpr;
use sasm_lang_core::object::{Number, Text};

pub mod dynchecked;
pub mod error;
pub mod ident;
pub mod ident_or_expr;
pub mod literal;
pub mod parsing;

/// An executable operation that can be executed by an interpreter.
#[derive(Debug)]
pub enum Instruction {
    /// Defines a variable with the given name.
    CreateVariable(Identifier),
    /// Moves a value to a variable.
    Move(Identifier, IdentOrExpr),
    /// Increments a variable containing a number.
    Increment(Identifier),
    /// Decrements a variable containing a number.
    Decrement(Identifier),
    /// Dumps the value of the given variable to `stdout`.
    Dump(IdentOrExpr),
    /// Performes mathematical addition on the given variable with the given operand.
    Add(Identifier, IdentOrExpr),
    /// Performes mathematical subtraction on the given variable with the given [subtrahend](https://www.dictionary.com/browse/subtrahend).
    Subtract(Identifier, IdentOrExpr),
    /// Performes mathematical multiplication on the given variable with the given multiplier.
    Multiply(Identifier, IdentOrExpr),
    /// Performes mathematical division on the given variable with the given divisor.
    Divide(Identifier, IdentOrExpr),
    /// Performes mathematical exponentiation on the given variable with the given exponent.
    Power(Identifier, IdentOrExpr),
    /// Compares the value inside the given variable with an expression (possibly another variable).
    /// The result of this comparion is saved by the interpreter into some internal variable.
    Compare(Identifier, IdentOrExpr),
    /// Skips a given number of instructions if the last comparison was `true`.
    JumpEqual(DynamicallyCheckedType<Number>),
    /// Skips a given number of instructions if the last comparison was `false`.
    JumpNotEqual(DynamicallyCheckedType<Number>),
    /// Skips a given number of instructions.
    Jump(DynamicallyCheckedType<Number>),
    /// Reads a number from `stdin` and saves it into the given variable.
    ReadNumericValue(Identifier),
    /// Reads a line from `stdin` and saves it into the given variable.
    ReadStringValue(Identifier),
    /// Writes a random number into the given variable. Optionally, a _minimum_ and _maximum_ range can be specified.
    GenerateRandomNumber(Identifier, Option<IdentOrExpr>, Option<IdentOrExpr>),
    /// Pushes a string (or a string inside another variable) into the given variable.
    Push(Identifier, IdentOrExpr),
    /// Pops a single character from a string inside the given variable.
    /// Optionally you can specify another variable, which will contain the popped character as a single character string.
    Pop(Identifier, Option<Identifier>),
    /// Writes a formatted string into a varible. The format string uses the same syntax as Rust's [`format!()`].
    Format(Identifier, DynamicallyCheckedType<Text>),
    /// Writes an expression to `stdout` **without newline**.
    Print(IdentOrExpr),
    /// Resets a variable's value to it's default.
    /// For numbers, it just sets them back to 0.
    /// For strings, it clears them - turning them into an empty string.
    Clear(Identifier),
    /// Calculates the length of an array-like object (eg. strings) and saves it into the given variable.
    Length(Identifier, IdentOrExpr),
    /// Stops execution for a given amount of time _(milliseconds)_.
    Sleep(IdentOrExpr),
    /// Delete the variable and deallocate the contained data.
    Delete(Identifier),
    /// Exits the program with the given exit code.
    Die(Number),
}
