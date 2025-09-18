pub mod gas;
pub mod instruction;
pub mod stack;
pub mod storage;
use crate::errors::{VMError, VMResult};
use gas::GasMeter;
use instruction::GasCost;
use instruction::Instruction;
use stack::Stack;
use storage::Inststorage;

pub struct VM {
    pc: usize,
    stack: Stack,
    program: Vec<Instruction>,
    storage: Inststorage,
    gas: GasMeter,
}

impl VM {
    pub fn new(program: Vec<Instruction>) -> Self {
        Self {
            pc: 0,
            stack: Stack::default(),
            program,
            storage: Inststorage::default(),
            gas: GasMeter::new(10_000),
        }
    }
    pub fn run(self: &mut Self) -> VMResult<()> {
        while self.pc < self.program.len() {
            let inst = self
                .program
                .get(self.pc)
                .ok_or(VMError::UnexpectedEof)?
                .clone();
            self.pc += 1;
            self.step(inst)?;
        }
        Ok(())
    }

    pub fn with_gas_limit(program: Vec<Instruction>, gas_limit: u64) -> Self {
        Self {
            pc: 0,
            stack: Stack::default(),
            program,
            storage: Inststorage::default(),
            gas: GasMeter::new(gas_limit),
        }
    }
    pub fn step(&mut self, instruction: Instruction) -> VMResult<()> {
        match instruction {
            Instruction::ADD => {
                self.gas.charge(GasCost::ADD)?;
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a + b);
            }
            Instruction::DIV => {
                self.gas.charge(GasCost::DIV)?;
                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a / b);
            }
            Instruction::DESTRUCT => {}
            Instruction::HALT => {
                self.gas.charge(GasCost::HALT)?;

                self.pc = self.program.len();
            }
            Instruction::MLOAD => {}
            Instruction::MSTORE => {}
            Instruction::MUL => {
                self.gas.charge(GasCost::MUL)?;

                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a * b);
            }
            Instruction::PUSH(v) => {
                self.gas.charge(GasCost::PUSH)?;

                self.stack.push(v.try_into().unwrap());
            }
            Instruction::SLOAD(s) => {
                self.gas.charge(GasCost::SLOAD)?;

                let val = self.storage.get(&s);
                if let Some(val) = val {
                    self.stack.push(val);
                } else {
                    return Err(VMError::StorageKeyNotFound((s)));
                }
            }
            Instruction::SSTORE(s) => {
                self.gas.charge(GasCost::SSTORE)?;

                let val = self.stack.pop()?;
                self.storage.set(s, val);
            }
            Instruction::SUB => {
                self.gas.charge(GasCost::SUB)?;

                let b = self.stack.pop()?;
                let a = self.stack.pop()?;
                self.stack.push(a - b);
            }
            Instruction::PRINT => {
                self.gas.charge(GasCost::PRINT)?;

                let v = self.stack.pop()?;
                println!("printed val is {}", v);
            }
        }
        Ok(())
    }
}
