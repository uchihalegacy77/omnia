//! Runtime system for Omnia
//! 
//! Manages execution context and resource tracking

use crate::error::{OmniaError, Result};
use crate::validator::{Validator, Limits};

/// Runtime execution context
pub struct Runtime {
    validator: Validator,
    instructions_executed: usize,
    max_instructions: usize,
}

impl Runtime {
    /// Create a new runtime
    pub fn new() -> Self {
        Runtime {
            validator: Validator::new(),
            instructions_executed: 0,
            max_instructions: 1_000_000_000,
        }
    }

    /// Create runtime with custom limits
    pub fn with_limits(limits: Limits) -> Self {
        Runtime {
            validator: Validator::with_limits(limits),
            instructions_executed: 0,
            max_instructions: 1_000_000_000,
        }
    }

    /// Track an instruction execution
    pub fn execute_instruction(&mut self) -> Result<()> {
        self.instructions_executed = self.instructions_executed.saturating_add(1);
        
        if self.instructions_executed > self.max_instructions {
            return Err(OmniaError::RuntimeError(
                "Maximum instruction count exceeded".to_string()
            ));
        }
        
        Ok(())
    }

    /// Get the validator
    pub fn validator(&self) -> &Validator {
        &self.validator
    }

    /// Reset execution stats
    pub fn reset(&mut self) {
        self.instructions_executed = 0;
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = Runtime::new();
        assert_eq!(runtime.instructions_executed, 0);
    }
}
