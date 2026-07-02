//! Error handling for Omnia

use std::fmt;

/// Result type for Omnia operations
pub type Result<T> = std::result::Result<T, OmniaError>;

/// Error types in Omnia
#[derive(Debug, Clone)]
pub enum OmniaError {
    LexerError(String),
    ParserError(String),
    RuntimeError(String),
    TypeError(String),
    ValueError(String),
    NameError(String),
    DimensionError(String),
    ValidationError(String),
    OverflowError(String),
    IndexError(String),
    StackOverflow(String),
    MemoryError(String),
}

impl fmt::Display for OmniaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OmniaError::LexerError(msg) => write!(f, "[LEXER] {}", msg),
            OmniaError::ParserError(msg) => write!(f, "[PARSER] {}", msg),
            OmniaError::RuntimeError(msg) => write!(f, "[RUNTIME] {}", msg),
            OmniaError::TypeError(msg) => write!(f, "[TYPE] {}", msg),
            OmniaError::ValueError(msg) => write!(f, "[VALUE] {}", msg),
            OmniaError::NameError(msg) => write!(f, "[NAME] {}", msg),
            OmniaError::DimensionError(msg) => write!(f, "[DIMENSION] {}", msg),
            OmniaError::ValidationError(msg) => write!(f, "[VALIDATION] {}", msg),
            OmniaError::OverflowError(msg) => write!(f, "[OVERFLOW] {}", msg),
            OmniaError::IndexError(msg) => write!(f, "[INDEX] {}", msg),
            OmniaError::StackOverflow(msg) => write!(f, "[STACK] {}", msg),
            OmniaError::MemoryError(msg) => write!(f, "[MEMORY] {}", msg),
        }
    }
}

impl std::error::Error for OmniaError {}

impl From<String> for OmniaError {
    fn from(msg: String) -> Self {
        OmniaError::RuntimeError(msg)
    }
}

impl From<&str> for OmniaError {
    fn from(msg: &str) -> Self {
        OmniaError::RuntimeError(msg.to_string())
    }
}

impl From<std::num::ParseIntError> for OmniaError {
    fn from(err: std::num::ParseIntError) -> Self {
        OmniaError::ValueError(format!("Integer parse error: {}", err))
    }
}

impl From<std::num::ParseFloatError> for OmniaError {
    fn from(err: std::num::ParseFloatError) -> Self {
        OmniaError::ValueError(format!("Float parse error: {}", err))
    }
}
