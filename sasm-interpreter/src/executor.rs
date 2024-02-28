use crate::{error::RuntimeError, varstorage::VariableStorage};
use sasm_parse::{
    expression::{Expression, Number},
    ident::Identifier,
    Instruction,
};
use std::process::exit;

pub enum ExecutorState {
    Ok,
    Goto(isize),
}

pub fn execute(
    instr: &Instruction,
    vars: &mut VariableStorage,
    cmp_result: &mut bool,
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
        Instruction::Add(ident, expr) => {
            let Expression::Number(amount) = pass_or_fetch(vars, expr)? else {
                return Err(RuntimeError::MismatchedTypes {
                    got: expr.type_name(),
                    expected: "Number",
                });
            };
            let amount = *amount;

            single_step(ident, vars, amount)?;
        }
        Instruction::Subtract(ident, expr) => {
            let Expression::Number(amount) = pass_or_fetch(vars, expr)? else {
                return Err(RuntimeError::MismatchedTypes {
                    got: expr.type_name(),
                    expected: "Number",
                });
            };
            let amount = *amount;

            single_step(ident, vars, -amount)?;
        }
        Instruction::Compare(ident, expr) => {
            let first = vars.get_nonnull(ident)?.clone();
            let second = pass_or_fetch(vars, expr)?;

            *cmp_result = &first == second;
        }
        Instruction::JumpEqual(offset) => {
            if *cmp_result {
                return Ok(ExecutorState::Goto(*offset as isize));
            }
        }
        Instruction::JumpNotEqual(offset) => {
            if !*cmp_result {
                return Ok(ExecutorState::Goto(*offset as isize));
            }
        }
        Instruction::Die(code) => exit(*code as i32),
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
