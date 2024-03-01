use crate::{error::RuntimeError, varstorage::VariableStorage};
use sasm_parse::{
    args::Arguments,
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
    match cmd.instr() {
        Instruction::CreateVariable => {
            cmd.args().check_count_exact(1)?;
            let ident = cmd.args().fetch_nth_as_ident(0).into_parse_err()?;

            vars.create(&ident)?;
        }
        Instruction::Move => {
            cmd.args().check_count_exact(2)?;
            let dst = cmd.args().fetch_nth_as_ident(0).into_parse_err()?;
            let src = cmd.args().fetch_nth_as_any(1).into_parse_err()?;
            let real_src = pass_or_fetch(vars, &src)?;

            vars.set(&dst, real_src.clone())?;
        }
        Instruction::Increment => {
            cmd.args().check_count_exact(1)?;
            let ident = cmd.args().fetch_nth_as_ident(0).into_parse_err()?;

            single_step(&ident, vars, |current| current + 1)?;
        }
        Instruction::Decrement => {
            cmd.args().check_count_exact(1)?;
            let ident = cmd.args().fetch_nth_as_ident(0).into_parse_err()?;

            single_step(&ident, vars, |current| current - 1)?;
        }
        Instruction::Dump => {
            cmd.args().check_count_exact(1)?;
            let expr = cmd.args().fetch_nth_as_any(0).into_parse_err()?;
            let real_value = pass_or_fetch_nullable(vars, &expr)?;

            var_dump(real_value);
        }
        Instruction::Add => {
            simple_math_op(cmd.args(), vars, |current, operand| current + operand)?;
        }
        Instruction::Subtract => {
            simple_math_op(cmd.args(), vars, |current, operand| current - operand)?;
        }
        Instruction::Multiply => {
            simple_math_op(cmd.args(), vars, |current, operand| current * operand)?;
        }
        Instruction::Divide => {
            simple_math_op(cmd.args(), vars, |current, operand| current / operand)?;
        }
        Instruction::Power => {
            simple_math_op(cmd.args(), vars, |current, exp| current.pow(exp as _))?;
        }
        _ => panic!(),
    }

    Ok(ExecutorState::Ok)
}

fn simple_math_op<F: FnOnce(Number, Number) -> Number>(
    args: &Arguments,
    vars: &mut VariableStorage,
    op: F,
) -> Result<(), RuntimeError> {
    args.check_count_exact(2)?;
    let ident = args.fetch_nth_as_ident(0).into_parse_err()?;
    let src = args.fetch_nth_as_any(1).into_parse_err()?;
    let real_src = pass_or_fetch(vars, &src)?;
    let value = expect_number(real_src)?;

    single_step(&ident, vars, |current| op(current, value))?;
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
