#[derive(Debug)]
pub enum VMError {
    StackUnderflow,
    UnexpectedEof,
    Invalid_Opcode,
    StorageKeyNotFound(String),
    GasExhausted,
     Io(std::io::Error),
    Parse(String),
}

pub type VMResult<T> = Result<T, VMError>;
