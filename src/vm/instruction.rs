#[derive(Debug, Clone)]
pub enum Instruction {
    PUSH(usize),
    // POP,
    MSTORE,
    MLOAD,
    SSTORE(String),
    SLOAD(String),
    ADD,
    SUB,
    MUL,
    DIV,
    HALT,
    DESTRUCT,
    PRINT,
}
pub struct GasCost;
impl GasCost {
    pub const HALT: u64 = 1;
    pub const PUSH: u64 = 2;
    pub const DIV: u64 = 1;
    pub const ADD: u64 = 3;
    pub const SUB: u64 = 3;
    pub const MUL: u64 = 5;
    pub const SSTORE: u64 = 20;
    pub const SLOAD: u64 = 10;
    pub const PRINT: u64 = 2;
}
