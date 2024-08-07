#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use core::f32;
use sasm_parse::{expression::Expression, Instruction};
use std::{env, fs};
use varstorage::VariableStorage;

mod error;
mod executor;
mod formatter;
mod repl_runner;
mod script_runner;
mod varstorage;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        exec_script(&args[1]);
    } else {
        repl_runner::start();
    }
}

fn exec_script(path: &str) {
    let Ok(script) = fs::read_to_string(path) else {
        eprintln!("Failed to read script file");
        return;
    };

    let instructions: Vec<_> = script
        .lines()
        .filter(|line| !line.is_empty())
        .map(Instruction::try_from)
        .collect();

    let mut errors = instructions
        .iter()
        .filter(|result| result.is_err())
        .peekable();
    if errors.peek().is_some() {
        for (i, error) in errors.enumerate() {
            eprintln!("Error on line {}: {}", i + 1, error.as_ref().unwrap_err());
        }

        return;
    }

    let commands: Vec<_> = instructions.into_iter().map(Result::unwrap).collect();
    script_runner::start(&commands);
}

pub fn fill_internal_vars(vars: &mut VariableStorage) {
    vars.set_internal("PLATFORM", Expression::make_string("UNSPECIFIED"));
    vars.set_internal(
        "SASMVER",
        Expression::make_string(env!("CARGO_PKG_VERSION")),
    );
    vars.set_internal("PI", Expression::Float(f32::consts::PI));
    vars.set_internal("E", Expression::Float(f32::consts::E));
}
