//! Interpreter for executing Omnia AST

use std::collections::HashMap;
use crate::error::{OmniaError, Result};
use crate::parser::AstNode;
use crate::value::Value;
use crate::validator::Validator;

/// Interpreter state and execution engine
pub struct Interpreter {
    variables: HashMap<String, Value>,
    functions: HashMap<String, AstNode>,
    validator: Validator,
    max_recursion_depth: usize,
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
            functions: HashMap::new(),
            validator: Validator::new(),
            max_recursion_depth: 1_000,
        }
    }

    /// Execute an AST with safety checks
    pub fn execute(&mut self, node: &AstNode) -> Result<Value> {
        self.execute_with_depth(node, 0)
    }

    fn execute_with_depth(&mut self, node: &AstNode, depth: usize) -> Result<Value> {
        self.validator.check_recursion_depth(depth)?;

        match node {
            AstNode::Program(statements) => {
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.execute_with_depth(stmt, depth.saturating_add(1))?;
                }
                Ok(result)
            }
            AstNode::Sequence { name, body } => {
                self.functions.insert(name.clone(), (**body).clone());
                Ok(Value::Null)
            }
            AstNode::Set { name, value } => {
                let val = self.execute_with_depth(value, depth.saturating_add(1))?;
                self.validator.validate_value(&val)?;
                self.variables.insert(name.clone(), val.clone());
                Ok(val)
            }
            AstNode::Evolve { name, value } => {
                let val = self.execute_with_depth(value, depth.saturating_add(1))?;
                self.validator.validate_value(&val)?;
                self.variables.insert(name.clone(), val.clone());
                Ok(val)
            }
            AstNode::Display(expr) => {
                let val = self.execute_with_depth(expr, depth.saturating_add(1))?;
                println!("{}", val);
                Ok(Value::Null)
            }
            AstNode::Literal(val) => self.parse_literal(val),
            AstNode::Identifier(name) => self.resolve_variable(name),
            AstNode::BinaryOp { left, op, right } => {
                self.execute_binary_op(left, op, right, depth.saturating_add(1))
            }
            AstNode::UnaryOp { op, operand } => {
                self.execute_unary_op(op, operand, depth.saturating_add(1))
            }
            AstNode::Array(elements) => {
                let mut arr = Vec::new();
                for elem in elements {
                    arr.push(self.execute_with_depth(elem, depth.saturating_add(1))?);
                }
                let array_val = Value::Array(arr);
                self.validator.validate_value(&array_val)?;
                Ok(array_val)
            }
            AstNode::Block(statements) => {
                let mut result = Value::Null;
                for stmt in statements {
                    result = self.execute_with_depth(stmt, depth.saturating_add(1))?;
                }
                Ok(result)
            }
            AstNode::If { condition, then_branch, else_branch } => {
                let cond_val = self.execute_with_depth(condition, depth.saturating_add(1))?;
                if cond_val.is_truthy() {
                    self.execute_with_depth(then_branch, depth.saturating_add(1))
                } else if let Some(else_b) = else_branch {
                    self.execute_with_depth(else_b, depth.saturating_add(1))
                } else {
                    Ok(Value::Null)
                }
            }
            AstNode::Empty => Ok(Value::Null),
        }
    }

    fn parse_literal(&self, val: &str) -> Result<Value> {
        match val {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            "null" => Ok(Value::Null),
            _ => {
                if val.contains('.') {
                    Ok(Value::Float(val.to_string()))
                } else if val.chars().all(|c| c.is_ascii_digit() || c == '-') {
                    Ok(Value::Integer(val.to_string()))
                } else {
                    Ok(Value::String(val.to_string()))
                }
            }
        }
    }

    fn resolve_variable(&self, name: &str) -> Result<Value> {
        self.variables
            .get(name)
            .cloned()
            .ok_or_else(|| OmniaError::NameError(format!("Undefined variable: {}", name)))
    }

    fn execute_binary_op(
        &mut self,
        left: &AstNode,
        op: &str,
        right: &AstNode,
        depth: usize,
    ) -> Result<Value> {
        let left_val = self.execute_with_depth(left, depth.saturating_add(1))?;
        let right_val = self.execute_with_depth(right, depth.saturating_add(1))?;

        match op {
            "+" => self.add_values(&left_val, &right_val),
            "-" => self.subtract_values(&left_val, &right_val),
            "*" => self.multiply_values(&left_val, &right_val),
            "/" => self.divide_values(&left_val, &right_val),
            "%" => self.modulo_values(&left_val, &right_val),
            "==" => Ok(Value::Bool(self.values_equal(&left_val, &right_val))),
            "!=" => Ok(Value::Bool(!self.values_equal(&left_val, &right_val))),
            "<" => self.compare_less(&left_val, &right_val),
            "<=" => self.compare_less_equal(&left_val, &right_val),
            ">" => self.compare_greater(&left_val, &right_val),
            ">=" => self.compare_greater_equal(&left_val, &right_val),
            "&&" => Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy())),
            "||" => Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy())),
            _ => Err(OmniaError::RuntimeError(format!("Unknown operator: {}", op))),
        }
    }

    fn execute_unary_op(&mut self, op: &str, operand: &AstNode, depth: usize) -> Result<Value> {
        let val = self.execute_with_depth(operand, depth.saturating_add(1))?;
        match op {
            "!" => Ok(Value::Bool(!val.is_truthy())),
            "-" => match val {
                Value::Integer(n) => {
                    if n.starts_with('-') {
                        Ok(Value::Integer(n[1..].to_string()))
                    } else {
                        Ok(Value::Integer(format!("-{}", n)))
                    }
                }
                Value::Float(n) => {
                    if n.starts_with('-') {
                        Ok(Value::Float(n[1..].to_string()))
                    } else {
                        Ok(Value::Float(format!("-{}", n)))
                    }
                }
                _ => Err(OmniaError::TypeError(format!(
                    "Cannot negate {}",
                    val.type_name()
                ))),
            },
            _ => Err(OmniaError::RuntimeError(format!("Unknown unary operator: {}", op))),
        }
    }

    fn add_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                match (a.parse::<i128>(), b.parse::<i128>()) {
                    (Ok(x), Ok(y)) => {
                        let result = x.checked_add(y)
                            .ok_or_else(|| OmniaError::OverflowError("Integer addition overflow".to_string()))?;
                        Ok(Value::Integer(result.to_string()))
                    }
                    _ => Ok(Value::Integer(format!("({} + {})", a, b))),
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                let af = a.parse::<f64>()?;
                let bf = b.parse::<f64>()?;
                Ok(Value::Float(format!("{}", af + bf)))
            }
            (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
            _ => Err(OmniaError::TypeError(format!(
                "Cannot add {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn subtract_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                let a_num = a.parse::<i64>()?;
                let b_num = b.parse::<i64>()?;
                Ok(Value::Integer((a_num.saturating_sub(b_num)).to_string()))
            }
            (Value::Float(a), Value::Float(b)) => {
                let af = a.parse::<f64>()?;
                let bf = b.parse::<f64>()?;
                Ok(Value::Float(format!("{}", af - bf)))
            }
            _ => Err(OmniaError::TypeError(format!(
                "Cannot subtract {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn multiply_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                match (a.parse::<i128>(), b.parse::<i128>()) {
                    (Ok(x), Ok(y)) => {
                        let result = x.checked_mul(y)
                            .ok_or_else(|| OmniaError::OverflowError("Integer multiplication overflow".to_string()))?;
                        Ok(Value::Integer(result.to_string()))
                    }
                    _ => Ok(Value::Integer(format!("({} * {})", a, b))),
                }
            }
            (Value::Float(a), Value::Float(b)) => {
                let af = a.parse::<f64>()?;
                let bf = b.parse::<f64>()?;
                Ok(Value::Float(format!("{}", af * bf)))
            }
            _ => Err(OmniaError::TypeError(format!(
                "Cannot multiply {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn divide_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                let b_num = b.parse::<i64>()?;
                if b_num == 0 {
                    return Err(OmniaError::ValueError("Division by zero".to_string()));
                }
                let a_num = a.parse::<i64>()?;
                Ok(Value::Integer((a_num.saturating_div(b_num)).to_string()))
            }
            (Value::Float(a), Value::Float(b)) => {
                let af = a.parse::<f64>()?;
                let bf = b.parse::<f64>()?;
                if bf == 0.0 {
                    return Err(OmniaError::ValueError("Division by zero".to_string()));
                }
                Ok(Value::Float(format!("{}", af / bf)))
            }
            _ => Err(OmniaError::TypeError(format!(
                "Cannot divide {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn modulo_values(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                let a_num = a.parse::<i64>()?;
                let b_num = b.parse::<i64>()?;
                if b_num == 0 {
                    return Err(OmniaError::ValueError("Modulo by zero".to_string()));
                }
                Ok(Value::Integer((a_num % b_num).to_string()))
            }
            _ => Err(OmniaError::TypeError(format!(
                "Cannot modulo {} and {}",
                left.type_name(),
                right.type_name()
            ))),
        }
    }

    fn compare_less(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::Bool(a.parse::<i64>()? < b.parse::<i64>()?))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::Bool(a.parse::<f64>()? < b.parse::<f64>()?))
            }
            _ => Err(OmniaError::TypeError("Cannot compare".to_string())),
        }
    }

    fn compare_less_equal(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::Bool(a.parse::<i64>()? <= b.parse::<i64>()?))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::Bool(a.parse::<f64>()? <= b.parse::<f64>()?))
            }
            _ => Err(OmniaError::TypeError("Cannot compare".to_string())),
        }
    }

    fn compare_greater(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::Bool(a.parse::<i64>()? > b.parse::<i64>()?))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::Bool(a.parse::<f64>()? > b.parse::<f64>()?))
            }
            _ => Err(OmniaError::TypeError("Cannot compare".to_string())),
        }
    }

    fn compare_greater_equal(&self, left: &Value, right: &Value) -> Result<Value> {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::Bool(a.parse::<i64>()? >= b.parse::<i64>()?))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::Bool(a.parse::<f64>()? >= b.parse::<f64>()?))
            }
            _ => Err(OmniaError::TypeError("Cannot compare".to_string())),
        }
    }

    fn values_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => (a.parse::<f64>().unwrap_or(0.0)
                - b.parse::<f64>().unwrap_or(0.0))
                .abs()
                < f64::EPSILON,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_execution() {
        let mut interpreter = Interpreter::new();
        let node = AstNode::Literal("42".to_string());
        let result = interpreter.execute(&node).unwrap();
        assert!(matches!(result, Value::Integer(_)));
    }
}
