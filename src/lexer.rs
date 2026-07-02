//! Lexer for Omnia language

use crate::error::{OmniaError, Result};

/// Token types in Omnia
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    // Keywords
    Sequence,
    Set,
    Evolve,
    Assess,
    Fallback,
    Cycle,
    Invoke,
    Display,
    To,
    With,
    From,
    Return,
    If,
    Else,
    
    // Literals
    Integer(String),
    Float(String),
    String(String),
    Identifier(String),
    True,
    False,
    Null,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    Not,
    Ampersand,
    Pipe,
    
    // Punctuation
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    Question,
    
    // Special
    Comment,
    Whitespace,
    Newline,
    Eof,
}

/// Token with type and position information
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

/// Lexer for tokenizing Omnia source code
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    /// Create a new lexer for the given input
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Tokenize the entire input with error recovery
    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        let max_tokens = 1_000_000;

        while self.position < self.input.len() && tokens.len() < max_tokens {
            match self.next_token() {
                Ok(token) => {
                    match token.token_type {
                        TokenType::Whitespace | TokenType::Comment | TokenType::Newline => {}
                        _ => tokens.push(token),
                    }
                }
                Err(e) => {
                    self.advance();
                    return Err(e);
                }
            }
        }

        if tokens.len() >= max_tokens {
            return Err(OmniaError::RuntimeError(
                "Token limit exceeded: source too complex".to_string()
            ));
        }

        tokens.push(Token {
            token_type: TokenType::Eof,
            line: self.line,
            column: self.column,
        });

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        let ch = self.current_char();
        let line = self.line;
        let column = self.column;

        match ch {
            Some(' ') | Some('\t') | Some('\r') => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Whitespace,
                    line,
                    column,
                })
            }
            Some('\n') => {
                self.advance();
                Ok(Token {
                    token_type: TokenType::Newline,
                    line,
                    column,
                })
            }
            Some('/') if self.peek_char() == Some('/') => {
                self.consume_line_comment();
                Ok(Token {
                    token_type: TokenType::Comment,
                    line,
                    column,
                })
            }
            Some('{') => {
                self.advance();
                Ok(Token { token_type: TokenType::LeftBrace, line, column })
            }
            Some('}') => {
                self.advance();
                Ok(Token { token_type: TokenType::RightBrace, line, column })
            }
            Some('[') => {
                self.advance();
                Ok(Token { token_type: TokenType::LeftBracket, line, column })
            }
            Some(']') => {
                self.advance();
                Ok(Token { token_type: TokenType::RightBracket, line, column })
            }
            Some('(') => {
                self.advance();
                Ok(Token { token_type: TokenType::LeftParen, line, column })
            }
            Some(')') => {
                self.advance();
                Ok(Token { token_type: TokenType::RightParen, line, column })
            }
            Some(',') => {
                self.advance();
                Ok(Token { token_type: TokenType::Comma, line, column })
            }
            Some('.') => {
                self.advance();
                Ok(Token { token_type: TokenType::Dot, line, column })
            }
            Some(':') => {
                self.advance();
                Ok(Token { token_type: TokenType::Colon, line, column })
            }
            Some(';') => {
                self.advance();
                Ok(Token { token_type: TokenType::Semicolon, line, column })
            }
            Some('?') => {
                self.advance();
                Ok(Token { token_type: TokenType::Question, line, column })
            }
            Some('+') => {
                self.advance();
                Ok(Token { token_type: TokenType::Plus, line, column })
            }
            Some('-') if self.peek_char() == Some('>') => {
                self.advance();
                self.advance();
                Ok(Token { token_type: TokenType::Arrow, line, column })
            }
            Some('-') => {
                self.advance();
                Ok(Token { token_type: TokenType::Minus, line, column })
            }
            Some('*') => {
                self.advance();
                Ok(Token { token_type: TokenType::Star, line, column })
            }
            Some('/') => {
                self.advance();
                Ok(Token { token_type: TokenType::Slash, line, column })
            }
            Some('%') => {
                self.advance();
                Ok(Token { token_type: TokenType::Percent, line, column })
            }
            Some('&') if self.peek_char() == Some('&') => {
                self.advance();
                self.advance();
                Ok(Token { token_type: TokenType::And, line, column })
            }
            Some('&') => {
                self.advance();
                Ok(Token { token_type: TokenType::Ampersand, line, column })
            }
            Some('|') if self.peek_char() == Some('|') => {
                self.advance();
                self.advance();
                Ok(Token { token_type: TokenType::Or, line, column })
            }
            Some('|') => {
                self.advance();
                Ok(Token { token_type: TokenType::Pipe, line, column })
            }
            Some('=') if self.peek_char() == Some('=') => {
                self.advance();
                self.advance();
                Ok(Token { token_type: TokenType::EqualEqual, line, column })
            }
            Some('=') => {
                self.advance();
                Ok(Token { token_type: TokenType::Equal, line, column })
            }
            Some('!') if self.peek_char() == Some('=') => {
                self.advance();
                self.advance();
                Ok(Token { token_type: TokenType::NotEqual, line, column })
            }
            Some('!') => {
                self.advance();
                Ok(Token { token_type: TokenType::Not, line, column })
            }
            Some('<') if self.peek_char() == Some('=') => {
                self.advance();
                self.advance();
                Ok(Token { token_type: TokenType::LessEqual, line, column })
            }
            Some('<') => {
                self.advance();
                Ok(Token { token_type: TokenType::Less, line, column })
            }
            Some('>') if self.peek_char() == Some('=') => {
                self.advance();
                self.advance();
                Ok(Token { token_type: TokenType::GreaterEqual, line, column })
            }
            Some('>') => {
                self.advance();
                Ok(Token { token_type: TokenType::Greater, line, column })
            }
            Some('"') => self.read_string(),
            Some('0'..='9') => self.read_number(),
            Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(),
            Some(c) => {
                Err(OmniaError::LexerError(format!(
                    "Unexpected character '{}' at line {}, column {}",
                    c, line, column
                )))
            }
            None => Ok(Token {
                token_type: TokenType::Eof,
                line,
                column,
            }),
        }
    }

    fn read_string(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        self.advance();

        let mut value = String::new();
        let max_len = 10_000_000;

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                self.advance();
                return Ok(Token {
                    token_type: TokenType::String(value),
                    line,
                    column,
                });
            } else if ch == '\\' {
                self.advance();
                match self.current_char() {
                    Some('n') => value.push('\n'),
                    Some('t') => value.push('\t'),
                    Some('r') => value.push('\r'),
                    Some('\\') => value.push('\\'),
                    Some('"') => value.push('"'),
                    Some(c) => value.push(c),
                    None => return Err(OmniaError::LexerError("Unterminated string".to_string())),
                }
                self.advance();
            } else {
                value.push(ch);
                self.advance();
            }
            
            if value.len() > max_len {
                return Err(OmniaError::MemoryError(
                    format!("String exceeds maximum length of {} characters", max_len)
                ));
            }
        }
        Err(OmniaError::LexerError("Unterminated string".to_string()))
    }

    fn read_number(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();
        let mut is_float = false;
        let max_len = 100_000;

        while let Some(ch) = self.current_char() {
            match ch {
                '0'..='9' => {
                    value.push(ch);
                    self.advance();
                }
                '.' if !is_float && self.peek_char().map_or(false, |c| c.is_ascii_digit()) => {
                    is_float = true;
                    value.push(ch);
                    self.advance();
                }
                '_' => {
                    self.advance();
                }
                _ => break,
            }

            if value.len() > max_len {
                return Err(OmniaError::LexerError(
                    format!("Number exceeds maximum length of {} digits", max_len)
                ));
            }
        }

        if value.is_empty() {
            return Err(OmniaError::LexerError("Invalid number".to_string()));
        }

        let clean_value = value.replace('_', "");
        let token_type = if is_float {
            TokenType::Float(clean_value)
        } else {
            TokenType::Integer(clean_value)
        };

        Ok(Token { token_type, line, column })
    }

    fn read_identifier(&mut self) -> Result<Token> {
        let line = self.line;
        let column = self.column;
        let mut value = String::new();
        let max_len = 10_000;

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                value.push(ch);
                self.advance();
            } else {
                break;
            }

            if value.len() > max_len {
                return Err(OmniaError::LexerError(
                    format!("Identifier exceeds maximum length of {} characters", max_len)
                ));
            }
        }

        let token_type = match value.as_str() {
            "sequence" => TokenType::Sequence,
            "set" => TokenType::Set,
            "evolve" => TokenType::Evolve,
            "assess" => TokenType::Assess,
            "fallback" => TokenType::Fallback,
            "cycle" => TokenType::Cycle,
            "invoke" => TokenType::Invoke,
            "display" => TokenType::Display,
            "to" => TokenType::To,
            "with" => TokenType::With,
            "from" => TokenType::From,
            "return" => TokenType::Return,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::Identifier(value),
        };

        Ok(Token { token_type, line, column })
    }

    fn consume_line_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            self.advance();
            if ch == '\n' {
                break;
            }
        }
    }

    fn current_char(&self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.position + 1 < self.input.len() {
            Some(self.input[self.position + 1])
        } else {
            None
        }
    }

    fn advance(&mut self) {
        if self.position < self.input.len() {
            if self.input[self.position] == '\n' {
                self.line = self.line.saturating_add(1);
                self.column = 1;
            } else {
                self.column = self.column.saturating_add(1);
            }
            self.position = self.position.saturating_add(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_keywords() {
        let mut lexer = Lexer::new("sequence set evolve");
        let tokens = lexer.tokenize().unwrap();
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_lexer_numbers() {
        let mut lexer = Lexer::new("42 3.14 1_000_000");
        let tokens = lexer.tokenize().unwrap();
        assert!(tokens.len() > 0);
    }
}
