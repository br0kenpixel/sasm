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
            let value = match src {
                Expression::Identifier(ident) => {
                    let value = vars.get(ident)?;

                    if value.is_none() {
                        return Err(RuntimeError::NullDeref);
                    }

                    value.cloned().unwrap()
                }
                other => other.clone(),
            };

            vars.set(dst, value)?;
        }
        Instruction::Increment(ident) => single_step(ident, vars, 1)?,
        Instruction::Decrement(ident) => single_step(ident, vars, -1)?,
        Instruction::Dump(expr) => {
            if let Expression::Identifier(ident) = expr {
                var_dump(vars.get(ident)?);
            } else {
                var_dump(Some(expr));
            }
        }
    }

    Ok(ExecutorState::Ok)
}

fn single_step(
    ident: &Identifier,
    vars: &mut VariableStorage,
    step: Number,
) -> Result<(), RuntimeError> {
    let value_ref = vars.get(ident)?;

    if value_ref.is_none() {
        return Err(RuntimeError::NullDeref);
    }

    let Expression::Number(current) = value_ref.unwrap() else {
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
