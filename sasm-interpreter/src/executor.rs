use crate::{error::RuntimeError, varstorage::VariableStorage};
use sasm_parse::{
    expression::{Expression, Number},
    ident::Identifier,
    Instruction,
};

pub enum ExecutorState {
    Ok,
    Goto(isize),
}

pub fn execute(
    instr: &Instruction,
    vars: &mut VariableStorage,
) -> Result<ExecutorState, RuntimeError> {
    match instr {
        Instruction::CreateVariable(ident) => {
            vars.create(ident)?;
        }
        Instruction::Move(dst, src) => {
            let value = pass_or_fetch(vars, src)?.clone();
            vars.set(dst, value)?;
        }
        Instruction::Increment(ident) => single_step(ident, vars, 1)?,
        Instruction::Decrement(ident) => single_step(ident, vars, -1)?,
        Instruction::Dump(expr) => var_dump(pass_or_fetch_nullable(vars, expr)?),
    }

    Ok(ExecutorState::Ok)
}

pub fn pass_or_fetch<'a>(
    vars: &'a mut VariableStorage,
    expr: &'a Expression,
) -> Result<&'a Expression, RuntimeError> {
    match expr {
        Expression::Identifier(ident) => vars.get_nonnull(ident),
        other => Ok(other),
    }
}

pub fn pass_or_fetch_nullable<'a>(
    vars: &'a mut VariableStorage,
    expr: &'a Expression,
) -> Result<Option<&'a Expression>, RuntimeError> {
    match expr {
        Expression::Identifier(ident) => vars.get(ident),
        other => Ok(Some(other)),
    }
}

fn single_step(
    ident: &Identifier,
    vars: &mut VariableStorage,
    step: Number,
) -> Result<(), RuntimeError> {
    let value_ref = vars.get_nonnull(ident)?;

    let Expression::Number(current) = value_ref else {
        return Err(RuntimeError::IllegalIncrement);
    };

    vars.set(ident, Expression::Number(current + step))?;
    Ok(())
}

fn var_dump(expr: Option<&Expression>) {
    match expr {
        None => println!("null"),
        Some(Expression::Number(n)) => println!("{n}"),
        Some(Expression::String(s)) => println!("{s}"),
        Some(Expression::Identifier(..)) => unreachable!(),
    }
}
