//! Parser for Omnia language

use crate::error::{OmniaError, Result};
use crate::lexer::{Token, TokenType};

/// Abstract Syntax Tree node types
#[derive(Debug, Clone)]
pub enum AstNode {
    Program(Vec<AstNode>),
    Sequence { name: String, body: Box<AstNode> },
    Set { name: String, value: Box<AstNode> },
    Evolve { name: String, value: Box<AstNode> },
    Display(Box<AstNode>),
    Literal(String),
    Identifier(String),
    BinaryOp { left: Box<AstNode>, op: String, right: Box<AstNode> },
    UnaryOp { op: String, operand: Box<AstNode> },
    Array(Vec<AstNode>),
    Block(Vec<AstNode>),
    If { condition: Box<AstNode>, then_branch: Box<AstNode>, else_branch: Option<Box<AstNode>> },
    Empty,
}

/// Parser for converting tokens to AST
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    /// Create a new parser
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    /// Parse tokens into an AST
    pub fn parse(&mut self) -> Result<AstNode> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if !self.check(TokenType::Eof) {
                statements.push(self.parse_statement()?);
            } else {
                break;
            }
        }

        Ok(AstNode::Program(statements))
    }

    fn parse_statement(&mut self) -> Result<AstNode> {
        match self.current_token_type() {
            Some(TokenType::Sequence) => self.parse_sequence(),
            Some(TokenType::Set) => self.parse_set(),
            Some(TokenType::Evolve) => self.parse_evolve(),
            Some(TokenType::Display) => self.parse_display(),
            Some(TokenType::If) => self.parse_if(),
            Some(TokenType::LeftBrace) => self.parse_block(),
            Some(TokenType::Semicolon) => {
                self.advance();
                Ok(AstNode::Empty)
            }
            _ => self.parse_expression(),
        }
    }

    fn parse_sequence(&mut self) -> Result<AstNode> {
        self.expect(TokenType::Sequence)?;
        let name = match self.current_token_type() {
            Some(TokenType::Identifier(n)) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(OmniaError::ParserError("Expected sequence name".to_string())),
        };
        self.expect(TokenType::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(TokenType::RightBrace)?;
        Ok(AstNode::Sequence { name, body: Box::new(body) })
    }

    fn parse_set(&mut self) -> Result<AstNode> {
        self.expect(TokenType::Set)?;
        let name = match self.current_token_type() {
            Some(TokenType::Identifier(n)) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(OmniaError::ParserError("Expected variable name".to_string())),
        };
        self.expect(TokenType::To)?;
        let value = self.parse_expression()?;
        Ok(AstNode::Set { name, value: Box::new(value) })
    }

    fn parse_evolve(&mut self) -> Result<AstNode> {
        self.expect(TokenType::Evolve)?;
        let name = match self.current_token_type() {
            Some(TokenType::Identifier(n)) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(OmniaError::ParserError("Expected variable name".to_string())),
        };
        self.expect(TokenType::To)?;
        let value = self.parse_expression()?;
        Ok(AstNode::Evolve { name, value: Box::new(value) })
    }

    fn parse_display(&mut self) -> Result<AstNode> {
        self.expect(TokenType::Display)?;
        let expr = self.parse_expression()?;
        Ok(AstNode::Display(Box::new(expr)))
    }

    fn parse_if(&mut self) -> Result<AstNode> {
        self.expect(TokenType::If)?;
        let condition = self.parse_expression()?;
        let then_branch = self.parse_statement()?;
        let else_branch = if let Some(TokenType::Else) = self.current_token_type() {
            self.advance();
            Some(Box::new(self.parse_statement()?))
        } else {
            None
        };
        Ok(AstNode::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn parse_block(&mut self) -> Result<AstNode> {
        self.expect(TokenType::LeftBrace)?;
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.parse_statement()?;
            if !matches!(stmt, AstNode::Empty) {
                statements.push(stmt);
            }
        }
        self.expect(TokenType::RightBrace)?;
        Ok(AstNode::Block(statements))
    }

    fn parse_expression(&mut self) -> Result<AstNode> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<AstNode> {
        let mut left = self.parse_logical_and()?;
        while let Some(TokenType::Or) = self.current_token_type() {
            self.advance();
            let right = self.parse_logical_and()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op: "||".to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<AstNode> {
        let mut left = self.parse_comparison()?;
        while let Some(TokenType::And) = self.current_token_type() {
            self.advance();
            let right = self.parse_comparison()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op: "&&".to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<AstNode> {
        let mut left = self.parse_additive()?;
        while let Some(op_type) = self.current_token_type() {
            let op = match op_type {
                TokenType::EqualEqual => "==",
                TokenType::NotEqual => "!=",
                TokenType::Less => "<",
                TokenType::LessEqual => "<=",
                TokenType::Greater => ">",
                TokenType::GreaterEqual => ">=",
                _ => break,
            };
            self.advance();
            let right = self.parse_additive()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_additive(&mut self) -> Result<AstNode> {
        let mut left = self.parse_multiplicative()?;
        while let Some(op_type) = self.current_token_type() {
            let op = match op_type {
                TokenType::Plus => "+",
                TokenType::Minus => "-",
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplicative(&mut self) -> Result<AstNode> {
        let mut left = self.parse_unary()?;
        while let Some(op_type) = self.current_token_type() {
            let op = match op_type {
                TokenType::Star => "*",
                TokenType::Slash => "/",
                TokenType::Percent => "%",
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                op: op.to_string(),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<AstNode> {
        match self.current_token_type() {
            Some(TokenType::Not) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(AstNode::UnaryOp {
                    op: "!".to_string(),
                    operand: Box::new(operand),
                })
            }
            Some(TokenType::Minus) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(AstNode::UnaryOp {
                    op: "-".to_string(),
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<AstNode> {
        match self.current_token_type() {
            Some(TokenType::Integer(n)) => {
                let val = n.clone();
                self.advance();
                Ok(AstNode::Literal(val))
            }
            Some(TokenType::Float(n)) => {
                let val = n.clone();
                self.advance();
                Ok(AstNode::Literal(val))
            }
            Some(TokenType::String(s)) => {
                let val = s.clone();
                self.advance();
                Ok(AstNode::Literal(val))
            }
            Some(TokenType::Identifier(id)) => {
                let name = id.clone();
                self.advance();
                Ok(AstNode::Identifier(name))
            }
            Some(TokenType::True) => {
                self.advance();
                Ok(AstNode::Literal("true".to_string()))
            }
            Some(TokenType::False) => {
                self.advance();
                Ok(AstNode::Literal("false".to_string()))
            }
            Some(TokenType::Null) => {
                self.advance();
                Ok(AstNode::Literal("null".to_string()))
            }
            Some(TokenType::LeftBracket) => self.parse_array(),
            Some(TokenType::LeftParen) => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                Ok(expr)
            }
            _ => Err(OmniaError::ParserError(
                format!("Unexpected token in expression: {:?}", self.current_token_type()),
            )),
        }
    }

    fn parse_array(&mut self) -> Result<AstNode> {
        self.expect(TokenType::LeftBracket)?;
        let mut elements = Vec::new();
        while !self.check(TokenType::RightBracket) && !self.is_at_end() {
            elements.push(self.parse_expression()?);
            if !self.check(TokenType::RightBracket) {
                self.expect(TokenType::Comma)?;
            }
        }
        self.expect(TokenType::RightBracket)?;
        Ok(AstNode::Array(elements))
    }

    fn current_token_type(&self) -> Option<TokenType> {
        if self.position < self.tokens.len() {
            Some(self.tokens[self.position].token_type.clone())
        } else {
            None
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        if let Some(current) = self.current_token_type() {
            std::mem::discriminant(&current) == std::mem::discriminant(&token_type)
        } else {
            false
        }
    }

    fn expect(&mut self, token_type: TokenType) -> Result<()> {
        if self.check(token_type.clone()) {
            self.advance();
            Ok(())
        } else {
            Err(OmniaError::ParserError(format!(
                "Expected {:?}, found {:?}",
                token_type, self.current_token_type()
            )))
        }
    }

    fn advance(&mut self) {
        if !self.is_at_end() {
            self.position = self.position.saturating_add(1);
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
            || matches!(self.current_token_type(), Some(TokenType::Eof) | None)
    }
}
