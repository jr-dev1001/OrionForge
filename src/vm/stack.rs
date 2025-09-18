use crate::errors::VMError;
use crate::errors::VMResult;
#[derive(Debug, Default)]
pub struct Stack(Vec<i64>);

impl Stack {
    pub fn push(&mut self, v: i64) {
        self.0.push(v);
    }
    pub fn pop(&mut self) -> VMResult<i64> {
        self.0.pop().ok_or(VMError::StackUnderflow)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
