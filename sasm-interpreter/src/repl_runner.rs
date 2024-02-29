use sasm_parse::Command;
use std::io::{stdin, stdout, Write};

use crate::{executor::execute, varstorage::VariableStorage};

pub fn start() {
    let mut variables = VariableStorage::new();
    let mut stdin_reader = stdin().lines();
    let mut cmp_result = false;

    println!("SASM Interpreter");
    println!("v{}\n", env!("CARGO_PKG_VERSION"));

    display_prompt();

    while let Some(Ok(line)) = stdin_reader.next() {
        if line.is_empty() {
            display_prompt();
            continue;
        }

        match Command::try_from(line.as_str()) {
            //Ok(Instruction::JumpNotEqual(..)) => {
            //    eprintln!("Jumps are not supported in REPL mode");
            //}
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
    }
}

fn display_prompt() {
    print!(">>> ");
    stdout().flush().unwrap();
}
