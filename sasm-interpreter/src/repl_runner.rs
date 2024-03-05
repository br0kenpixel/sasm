use sasm_parse::Instruction;
use std::io::{stdin, stdout, Write};

use crate::{executor::execute, varstorage::VariableStorage};

pub fn start() {
    let mut variables = VariableStorage::new();
    let mut line = String::new();
    let mut cmp_result = false;

    println!("SASM Interpreter");
    println!("v{}\n", env!("CARGO_PKG_VERSION"));

    display_prompt();

    while stdin().read_line(&mut line).is_ok() {
        line = line.trim_end().to_string();

        if line.is_empty() {
            display_prompt();
            continue;
        }

        match Instruction::try_from(line.as_str()) {
            Ok(Instruction::JumpNotEqual(..)) => {
                eprintln!("Jumps are not supported in REPL mode");
            }
            Ok(instr) => match execute(&instr, &mut variables, &mut cmp_result) {
                Ok(_) => (),
                Err(why) => {
                    eprintln!("Runtime error: {why}");
                }
            },
            Err(why) => {
                eprintln!("Failed to parse input: {why}");
            }
        }

        display_prompt();
        line.clear();
    }
}

fn display_prompt() {
    print!(">>> ");
    stdout().flush().unwrap();
}
