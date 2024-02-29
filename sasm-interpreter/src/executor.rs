use crate::{error::RuntimeError, varstorage::VariableStorage};
use sasm_parse::{
    expression::{Expression, Number},
    ident::Identifier,
    instr::Instruction,
    Command,
};

pub enum ExecutorState {
    Ok,
    Goto(isize),
}

pub fn execute(
    cmd: &Command,
    vars: &mut VariableStorage,
    cmp_result: &mut bool,
) -> Result<ExecutorState, RuntimeError> {
    todo!();
    //match cmd.instr() {}

    Ok(ExecutorState::Ok)
}

pub fn pass_or_fetch<'a>(
    vars: &'a VariableStorage,
    expr: &'a Expression,
) -> Result<&'a Expression, RuntimeError> {
    match expr {
        Expression::Identifier(ident) => vars.get_nonnull(ident),
        other => Ok(other),
    }
}

pub fn pass_or_fetch_nullable<'a>(
    vars: &'a VariableStorage,
    expr: &'a Expression,
) -> Result<Option<&'a Expression>, RuntimeError> {
    match expr {
        Expression::Identifier(ident) => vars.get(ident),
        other => Ok(Some(other)),
    }
}

fn single_step<F: FnOnce(Number) -> Number>(
    ident: &Identifier,
    vars: &mut VariableStorage,
    step: F,
) -> Result<(), RuntimeError> {
    let value_ref = vars.get_nonnull(ident)?;

    let Expression::Number(current) = value_ref else {
        return Err(RuntimeError::IllegalMathOp);
    };

    vars.set(ident, Expression::Number(step(*current)))?;
    Ok(())
}

const fn expect_number(expr: &Expression) -> Result<Number, RuntimeError> {
    let Expression::Number(n) = expr else {
        return Err(RuntimeError::MismatchedTypes {
            got: expr.type_name(),
            expected: "Number",
        });
    };

    Ok(*n)
}

fn var_dump(expr: Option<&Expression>) {
    match expr {
        None => println!("null"),
        Some(Expression::Number(n)) => println!("{n}"),
        Some(Expression::String(s)) => println!("{s}"),
        Some(Expression::Identifier(..)) => unreachable!(),
    }
}
