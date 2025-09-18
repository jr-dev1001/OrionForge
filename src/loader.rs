use crate::errors::{VMError, VMResult};
use crate::vm::instruction::Instruction;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
struct ProgramFile {
    program: Vec<JsonInstr>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "op", rename_all = "PascalCase")]
enum JsonInstr {
    Halt,
    Push { arg: i64 },
    Add,
    Sub,
    Mul,
    Store { key: String },
    Load  { key: String },
    Print,
}

pub fn load_program_from_json<P: AsRef<Path>>(path: P) -> VMResult<Vec<Instruction>> {
    let raw = fs::read_to_string(path).map_err(|e| VMError::Io(e))?;
    let pf: ProgramFile = serde_json::from_str(&raw).map_err(|e| VMError::Parse(e.to_string()))?;
    let mut out = Vec::with_capacity(pf.program.len());
    for j in pf.program {
        use Instruction as I;
        let inst = match j {
            JsonInstr::Halt         => I::HALT,
            JsonInstr::Push{arg}    => I::PUSH(arg.try_into().unwrap()),
            JsonInstr::Add          => I::ADD,
            JsonInstr::Sub          => I::SUB,
            JsonInstr::Mul          => I::MUL,
            JsonInstr::Store{key}   => I::SSTORE(key),
            JsonInstr::Load{key}    => I::SLOAD(key),
            JsonInstr::Print        => I::PRINT,
        };
        out.push(inst);
    }
    Ok(out)
}
