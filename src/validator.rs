//! Validation module for Omnia
//! 
//! Provides type checking, bounds validation, and safe execution guarantees

use crate::error::{OmniaError, Result};
use crate::value::Value;

/// Limits for different value types
pub struct Limits {
    pub max_integer_digits: usize,
    pub max_string_length: usize,
    pub max_array_size: usize,
    pub max_object_size: usize,
    pub max_tokens: usize,
    pub max_recursion_depth: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            max_integer_digits: 100_000,
            max_string_length: 10_000_000,
            max_array_size: 10_000_000,
            max_object_size: 1_000_000,
            max_tokens: 1_000_000,
            max_recursion_depth: 1_000,
        }
    }
}

/// Validator for ensuring safe execution
pub struct Validator {
    limits: Limits,
}

impl Validator {
    /// Create a new validator with default limits
    pub fn new() -> Self {
        Validator {
            limits: Limits::default(),
        }
    }

    /// Create a validator with custom limits
    pub fn with_limits(limits: Limits) -> Self {
        Validator { limits }
    }

    /// Validate a value's bounds
    pub fn validate_value(&self, value: &Value) -> Result<()> {
        match value {
            Value::Integer(n) => {
                if n.len() > self.limits.max_integer_digits {
                    return Err(OmniaError::ValidationError(
                        format!("Integer exceeds maximum {} digits", self.limits.max_integer_digits)
                    ));
                }
            }
            Value::Float(f) => {
                if f.len() > self.limits.max_integer_digits {
                    return Err(OmniaError::ValidationError(
                        format!("Float exceeds maximum length of {} characters", self.limits.max_integer_digits)
                    ));
                }
            }
            Value::String(s) => {
                if s.len() > self.limits.max_string_length {
                    return Err(OmniaError::MemoryError(
                        format!("String exceeds maximum length of {} bytes", self.limits.max_string_length)
                    ));
                }
            }
            Value::Array(arr) => {
                if arr.len() > self.limits.max_array_size {
                    return Err(OmniaError::MemoryError(
                        format!("Array exceeds maximum size of {} elements", self.limits.max_array_size)
                    ));
                }
                for elem in arr {
                    self.validate_value(elem)?;
                }
            }
            Value::Object(map) => {
                if map.len() > self.limits.max_object_size {
                    return Err(OmniaError::MemoryError(
                        format!("Object exceeds maximum size of {} entries", self.limits.max_object_size)
                    ));
                }
                for (_, v) in map {
                    self.validate_value(v)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Check type compatibility
    pub fn is_compatible(actual: &str, expected: &str) -> bool {
        if actual == expected {
            return true;
        }
        (actual == "integer" || actual == "float") && (expected == "integer" || expected == "float")
    }

    /// Validate recursion depth
    pub fn check_recursion_depth(&self, depth: usize) -> Result<()> {
        if depth > self.limits.max_recursion_depth {
            Err(OmniaError::StackOverflow(
                format!("Recursion depth exceeds maximum of {}", self.limits.max_recursion_depth)
            ))
        } else {
            Ok(())
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_string_bounds() {
        let validator = Validator::new();
        let s = Value::String("hello".to_string());
        assert!(validator.validate_value(&s).is_ok());
    }

    #[test]
    fn test_validator_array_bounds() {
        let validator = Validator::new();
        let arr = Value::Array(vec![Value::Integer("1".to_string())]);
        assert!(validator.validate_value(&arr).is_ok());
    }
}
