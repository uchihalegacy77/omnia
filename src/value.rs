//! Value system for Omnia with infinite precision mathematics

use std::fmt;
use std::collections::HashMap;
use crate::error::{OmniaError, Result};

/// Represents a value in Omnia with support for infinite precision
#[derive(Debug, Clone)]
pub enum Value {
    /// Infinite precision integer (represented as String)
    Integer(String),
    /// Infinite precision float (represented as String)
    Float(String),
    /// Boolean value
    Bool(bool),
    /// String value (pure ASCII)
    String(String),
    /// Array of values
    Array(Vec<Value>),
    /// Object/Map with key-value pairs
    Object(HashMap<String, Value>),
    /// Null value
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::String(s) => write!(f, "{}", s),
            Value::Array(arr) => {
                write!(f, "[").ok();
                for (i, v) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ").ok();
                    }
                    write!(f, "{}", v).ok();
                }
                write!(f, "]")
            }
            Value::Object(map) => {
                write!(f, "{{").ok();
                for (i, (k, v)) in map.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ").ok();
                    }
                    write!(f, \"{}\": {}", k, v).ok();
                }
                write!(f, "}}")
            }
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    /// Check if value is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Integer(n) => n != "0" && !n.is_empty(),
            Value::Float(n) => n != "0.0" && n != "0" && !n.is_empty(),
            Value::String(s) => !s.is_empty(),
            Value::Array(a) => !a.is_empty(),
            Value::Object(o) => !o.is_empty(),
        }
    }

    /// Get the type name of this value
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::Bool(_) => "bool",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Null => "null",
        }
    }

    /// Safe length check
    pub fn len(&self) -> Result<usize> {
        match self {
            Value::String(s) => Ok(s.len()),
            Value::Array(a) => Ok(a.len()),
            Value::Object(o) => Ok(o.len()),
            _ => Err(OmniaError::TypeError(format!("Cannot get length of {}", self.type_name()))),
        }
    }
}
