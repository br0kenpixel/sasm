use crate::{
    error::RuntimeError,
    executor::{execute, ExecutorState},
    fill_internal_vars,
    varstorage::VariableStorage,
};
use sasm_parse::Instruction;

pub fn start(cmds: &[Instruction]) {
    let mut variables = VariableStorage::new();
    let mut exec_pos = 0;
    let mut cmp_result = false;

    fill_internal_vars(&mut variables);

    while exec_pos < cmds.len() {
        let instr = &cmds[exec_pos];

        match execute(instr, &mut variables, &mut cmp_result) {
            Ok(ExecutorState::Ok) => exec_pos += 1,
            Ok(ExecutorState::Goto(offset)) => {
                if offset > 0 {
                    exec_pos += offset as usize;
                } else {
                    exec_pos -= offset.unsigned_abs();
                }

                if cmds.get(exec_pos).is_none() {
                    report_runtime_err(exec_pos, instr, &RuntimeError::IllegalGoto(exec_pos + 1));
                    break;
                }
            }
            Err(why) => {
                report_runtime_err(exec_pos, instr, &why);
                break;
            }
        }
    }
}

fn report_runtime_err(exec_pos: usize, instr: &Instruction, err: &RuntimeError) {
    eprintln!("Runtime error on line {}:", exec_pos + 1);
    eprintln!("\t-> {instr}");
    eprintln!("\t|- {err}");
}
