use sasm_parse::Instruction;
use std::io::{stdin, stdout, Write};

use crate::{executor::execute, varstorage::VariableStorage};

pub fn start() {
    let mut variables = VariableStorage::new();
    let mut stdin_reader = stdin().lines();

    println!("SASM Interpreter");
    println!("v{}\n", env!("CARGO_PKG_VERSION"));

    display_prompt();

    while let Some(Ok(line)) = stdin_reader.next() {
        if line.is_empty() {
            display_prompt();
            continue;
        }

        match Instruction::try_from(line.as_str()) {
            Ok(instr) => match execute(&instr, &mut variables) {
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
