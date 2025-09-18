use crate::errors::{VMError, VMResult};

#[derive(Debug)]
pub struct GasMeter {
    limit: u64,
    used: u64,
}

impl GasMeter {
    pub fn new(limit: u64) -> Self {
        Self { limit, used: 0 }
    }
    pub fn charge(&mut self, amount: u64) -> VMResult<()> {
        if self.used > self.limit {
            Err(VMError::GasExhausted)
        } else {
            self.used += amount;
            Ok(())
        }
    }
}
