// 🌌 Omni-Drive Engine V8.0
// Architecture: Hybrid Flow (Natural Keywords + Rigid Brackets)
// Paradigms: Highly Readable, Visually Structured, Immutable State

use std::collections::HashMap;
use std::process;

// --- 1. THE HYBRID LEXER ---
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Sequence, Display, Set, To,      
    OpenBracket, CloseBracket,       
    Plus, Minus, Star, Slash,                      
    Ident(String), Text(String), Num(i64), 
}

fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '[' => { tokens.push(Token::OpenBracket); chars.next(); }
            ']' => { tokens.push(Token::CloseBracket); chars.next(); }
            '+' => { tokens.push(Token::Plus); chars.next(); }
            '-' => { tokens.push(Token::Minus); chars.next(); }
            '*' => { tokens.push(Token::Star); chars.next(); }
            '/' => { tokens.push(Token::Slash); chars.next(); }
            '"' => {
                chars.next();
                let mut text = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '"' { chars.next(); break; }
                    text.push(chars.next().unwrap());
                }
                tokens.push(Token::Text(text));
            }
            ' ' | '\n' | '\r' | '\t' => { chars.next(); }
            _ if c.is_ascii_digit() => {
                let mut num_str = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() { num_str.push(chars.next().unwrap()); }
                    else { break; }
                }
                tokens.push(Token::Num(num_str.parse().unwrap()));
            }
            _ if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' { ident.push(chars.next().unwrap()); }
                    else { break; }
                }
                match ident.as_str() {
                    "sequence" => tokens.push(Token::Sequence),
                    "display" => tokens.push(Token::Display),
                    "set" => tokens.push(Token::Set),
                    "to" => tokens.push(Token::To),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            _ => { chars.next(); }
        }
    }
    tokens
}

// --- 2. THE ABSTRACT SYNTAX TREE (AST) ---
#[derive(Debug, Clone)]
enum Expr {
    Text(String),
    Num(i64),
    Variable(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
enum Statement {
    Display(Expr),
    Set(String, Expr),
}

struct SequenceCore {
    name: String,
    body: Vec<Statement>,
}

fn parse(tokens: &[Token]) -> Vec<SequenceCore> {
    let mut cores = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i] == Token::Sequence {
            if i + 2 < tokens.len() {
                if let Token::Ident(ref name) = tokens[i+1] {
                    if tokens[i+2] == Token::OpenBracket {
                        let mut body = Vec::new();
                        i += 3;
                        
                        while i < tokens.len() && tokens[i] != Token::CloseBracket {
                            if tokens[i] == Token::Display {
                                if i + 1 >= tokens.len() { abort("Syntax Error: Missing argument for 'display'"); }
                                match &tokens[i+1] {
                                    Token::Text(t) => { body.push(Statement::Display(Expr::Text(t.clone()))); i += 2; },
                                    Token::Ident(id) => { body.push(Statement::Display(Expr::Variable(id.clone()))); i += 2; },
                                    Token::Num(n) => { body.push(Statement::Display(Expr::Num(*n))); i += 2; },
                                    _ => abort("Syntax Error: Invalid display argument"),
                                }
                            } else if tokens[i] == Token::Set {
                                if i + 3 >= tokens.len() { abort("Syntax Error: Incomplete 'set' statement"); }
                                if let Token::Ident(ref var_name) = tokens[i+1] {
                                    if tokens[i+2] == Token::To {
                                        let get_expr = |tok: &Token| -> Expr {
                                            match tok {
                                                Token::Num(n) => Expr::Num(*n),
                                                Token::Ident(id) => Expr::Variable(id.clone()),
                                                Token::Text(t) => Expr::Text(t.clone()),
                                                _ => abort("Syntax Error: Invalid assignment value"),
                                            }
                                        };

                                        let left_expr = get_expr(&tokens[i+3]);

                                        if i + 5 < tokens.len() {
                                            match tokens[i+4] {
                                                Token::Plus => {
                                                    let right = get_expr(&tokens[i+5]);
                                                    body.push(Statement::Set(var_name.clone(), Expr::Add(Box::new(left_expr), Box::new(right))));
                                                    i += 6; continue;
                                                },
                                                Token::Minus => {
                                                    let right = get_expr(&tokens[i+5]);
                                                    body.push(Statement::Set(var_name.clone(), Expr::Sub(Box::new(left_expr), Box::new(right))));
                                                    i += 6; continue;
                                                },
                                                Token::Star => {
                                                    let right = get_expr(&tokens[i+5]);
                                                    body.push(Statement::Set(var_name.clone(), Expr::Mul(Box::new(left_expr), Box::new(right))));
                                                    i += 6; continue;
                                                },
                                                Token::Slash => {
                                                    let right = get_expr(&tokens[i+5]);
                                                    body.push(Statement::Set(var_name.clone(), Expr::Div(Box::new(left_expr), Box::new(right))));
                                                    i += 6; continue;
                                                },
                                                _ => {}
                                            }
                                        }
                                        
                                        body.push(Statement::Set(var_name.clone(), left_expr));
                                        i += 4;
                                        continue;
                                    }
                                }
                            } else {
                                abort(&format!("Syntax Error: Unexpected token {:?}", tokens[i]));
                            }
                        }
                        cores.push(SequenceCore { name: name.clone(), body });
                    }
                }
            }
        }
        i += 1;
    }
    cores
}

// --- 3. THE VIRTUAL MACHINE ---
#[derive(Clone, Debug)]
enum Value { Num(i64), Text(String) }

fn abort(msg: &str) -> ! {
    eprintln!("❌ [FATAL ERROR] {}", msg);
    process::exit(1);
}

fn evaluate(expr: &Expr, env: &HashMap<String, Value>) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Text(t) => Value::Text(t.clone()),
        Expr::Variable(id) => env.get(id).cloned().unwrap_or_else(|| abort(&format!("'{}' is undefined.", id))),
        Expr::Add(left, right) => {
            if let (Value::Num(l), Value::Num(r)) = (evaluate(left, env), evaluate(right, env)) {
                Value::Num(l + r)
            } else { abort("Math Error: Can only add numbers."); }
        }
        Expr::Sub(left, right) => {
            if let (Value::Num(l), Value::Num(r)) = (evaluate(left, env), evaluate(right, env)) {
                Value::Num(l - r)
            } else { abort("Math Error: Can only subtract numbers."); }
        }
        Expr::Mul(left, right) => {
            if let (Value::Num(l), Value::Num(r)) = (evaluate(left, env), evaluate(right, env)) {
                Value::Num(l * r)
            } else { abort("Math Error: Can only multiply numbers."); }
        }
        Expr::Div(left, right) => {
            if let (Value::Num(l), Value::Num(r)) = (evaluate(left, env), evaluate(right, env)) {
                if r == 0 { abort("Math Error: Division by zero."); }
                Value::Num(l / r)
            } else { abort("Math Error: Can only divide numbers."); }
        }
    }
}

fn run(cores: Vec<SequenceCore>) {
    let mut memory: HashMap<String, Value> = HashMap::new();
    for core in cores {
        if core.name == "genesis" {
            for stmt in core.body {
                match stmt {
                    Statement::Set(name, expr) => { 
                        if memory.contains_key(&name) {
                            abort(&format!("IMMUTABILITY BREACH: Variable '{}' is already bound and cannot be changed.", name));
                        }
                        let val = evaluate(&expr, &memory);
                        memory.insert(name, val); 
                    }
                    Statement::Display(expr) => {
                        match evaluate(&expr, &memory) {
                            Value::Num(n) => println!("  📡 Output: {}", n),
                            Value::Text(t) => println!("  📡 Output: {}", t),
                        }
                    }
                }
            }
        }
    }
}

// --- IGNITION ---
fn main() {
    let omnia_code = r#"
        sequence genesis [
            display "Omni-Drive Hybrid Engine Initialized."
            
            set base to 500
            set multiplier to 2
            
            set total to base * multiplier
            
            display "Total system power locked at:"
            display total
        ]
    "#;

    println!("🚀 Booting Hybrid Engine...\n");
    let tokens = lex(omnia_code);
    let ast = parse(&tokens);
    run(ast);
    println!("\n✅ Local Execution Complete. Zero bloat detected.");
}


