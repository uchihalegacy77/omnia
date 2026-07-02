//! Omnia: The Universal Architecture
//! 
//! A zero-bloat, Turing-complete programming language built with infinite precision mathematics
//! and dimensional memory scoping.

pub mod lexer;
pub mod parser;
pub mod interpreter;
pub mod error;
pub mod value;
pub mod validator;
pub mod runtime;

pub use error::{OmniaError, Result};
pub use interpreter::Interpreter;
pub use parser::Parser;
pub use lexer::Lexer;
pub use runtime::Runtime;

/// Initialize the Omnia runtime
pub fn init() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_initialization() {
        assert!(init().is_ok());
    }
}
