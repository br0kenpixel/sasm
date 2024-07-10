use crate::{error::RuntimeError, formatter::format, varstorage::VariableStorage};
use sasm_parse::{
    expression::{Expression, Number, Text},
    ident::Identifier,
    type_trait::SasmType,
    Instruction,
};
use std::{io::stdin, process::exit};

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
        Instruction::Increment(ident) => single_step(ident, vars, |current| current + 1)?,
        Instruction::Decrement(ident) => single_step(ident, vars, |current| current - 1)?,
        Instruction::Dump(expr) => var_dump(pass_or_fetch_nullable(vars, expr)?),
        Instruction::Add(ident, expr) => math_op(vars, ident, expr, Number::wrapping_add)?,
        Instruction::Multiply(ident, expr) => math_op(vars, ident, expr, Number::wrapping_mul)?,
        Instruction::Divide(ident, expr) => {
            let amount = expect::<Number>(pass_or_fetch(vars, expr)?)?;

            if amount == 0 {
                return Err(RuntimeError::DivisionByZero);
            }

            single_step(ident, vars, |current| current / amount)?;
        }
        Instruction::Power(ident, expr) => {
            let amount = expect::<Number>(pass_or_fetch(vars, expr)?)?;

            single_step(ident, vars, |current| current.pow(amount as _))?;
        }
        Instruction::Subtract(ident, expr) => math_op(vars, ident, expr, Number::wrapping_sub)?,
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
        Instruction::Jump(offset) => {
            return Ok(ExecutorState::Goto(*offset as isize));
        }
        Instruction::ReadNumericValue(ident) => {
            let line = stdin_readline()?;

            let Ok(num) = line.parse::<Number>() else {
                return Err(RuntimeError::IllegalNumber(line));
            };

            vars.set(ident, Expression::Number(num))?;
        }
        Instruction::ReadStringValue(ident) => {
            vars.set(ident, Expression::String(stdin_readline()?))?;
        }
        Instruction::GenerateRandomNumber(ident, range_min, range_max) => {
            let mut min = Number::MIN;
            let mut max = Number::MAX;
            let range_bounds = range_min.as_ref().zip(range_max.as_ref());

            if let Some((range_min, range_max)) = range_bounds {
                min = expect(pass_or_fetch(vars, range_min)?)?;
                max = expect(pass_or_fetch(vars, range_max)?)?;
            }

            let randval = fastrand::i64(min..=max);
            vars.set(ident, Expression::Number(randval))?;
        }
        Instruction::Push(ident, src) => {
            let src = pass_or_fetch(vars, src)?;

            let mut string = expect::<Text>(vars.get_nonnull(ident)?)?;

            match src {
                Expression::String(other) => string.push_str(other),
                Expression::Number(other) => string.push_str(&other.to_string()),
                Expression::Identifier(..) => unreachable!(),
            }
            vars.set(ident, Expression::String(string))?;
        }
        Instruction::Pop(what, dst) => {
            let mut string = expect::<Text>(vars.get_nonnull(what)?)?;
            let popped = string.pop();

            if let Some((dst_ident, ch)) = dst.as_ref().zip(popped) {
                vars.set(dst_ident, Expression::singe_char_string(ch))?;
            }

            vars.set(what, Expression::String(string))?;
        }
        Instruction::Print(what) => match what {
            Expression::Identifier(..) => panic!("Attempted to print an identifier"),
            Expression::Number(n) => println!("{n}"),
            Expression::String(text) => print!("{text}"),
        },
        Instruction::Format(dst, fmt) => {
            let formatted = format(fmt, vars)?;
            vars.set(dst, Expression::String(formatted))?;
        }
        Instruction::Delete(ident) => {
            vars.delete(ident)?;
        }
        Instruction::Die(code) => exit(*code as i32),
    }

    Ok(ExecutorState::Ok)
}

fn stdin_readline() -> Result<String, RuntimeError> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;

    line = line.trim_end().to_string();
    Ok(line)
}

fn math_op<F: FnOnce(Number, Number) -> Number>(
    vars: &mut VariableStorage,
    ident: &Identifier,
    expr: &Expression,
    op: F,
) -> Result<(), RuntimeError> {
    let amount = expect(pass_or_fetch(vars, expr)?)?;

    single_step(ident, vars, |current| op(current, amount))?;
    Ok(())
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

fn expect<T: SasmType + 'static>(expr: &Expression) -> Result<T, RuntimeError> {
    let got_type_name = expr.type_name();
    let any = expr.clone().inner_as_any();

    let res = any
        .downcast::<T>()
        .map_err(|_| RuntimeError::MismatchedTypes {
            got: got_type_name,
            expected: T::type_name(),
        })?;

    Ok(*res)
}

fn var_dump(expr: Option<&Expression>) {
    match expr {
        None => println!("null"),
        Some(Expression::Number(n)) => println!("{n}"),
        Some(Expression::String(s)) => println!("{s}"),
        Some(Expression::Identifier(..)) => unreachable!(),
    }
}
