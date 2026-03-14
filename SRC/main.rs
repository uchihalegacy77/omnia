// [OMNIA] Omni-Drive Engine V1.0 (Public Launch Edition)
// Architecture: Universal Engine (Infinite Scoping, 128-bit Math, Pure ASCII)

use std::collections::HashMap;
use std::process;

// ==========================================
// 1. THE LEXER (Tokenization)
// Converts raw human text into machine-readable Tokens.
// ==========================================
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Sequence, Display, Set, Evolve, To, Assess, Fallback, Cycle, Invoke, With,
    OpenBracket, CloseBracket,
    Plus, Minus, Star, Slash, EqEq, NotEq, Lt, Gt,
    Ident(String), Text(String), Num(i128), // Note: i128 supports cosmic-scale math
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
            '/' => {
                chars.next();
                // Ignore comments starting with //
                if let Some(&'/') = chars.peek() {
                    while let Some(&next) = chars.peek() {
                        if next == '\n' { break; } chars.next();
                    }
                } else { tokens.push(Token::Slash); }
            }
            '=' => {
                chars.next();
                if let Some(&'=') = chars.peek() { tokens.push(Token::EqEq); chars.next(); }
                else { abort("[OMNIA-ERR-01] Syntax Error: Use 'to' for assignment, '==' for comparison."); }
            }
            '!' => {
                chars.next();
                if let Some(&'=') = chars.peek() { tokens.push(Token::NotEq); chars.next(); }
            }
            '<' => { tokens.push(Token::Lt); chars.next(); }
            '>' => { tokens.push(Token::Gt); chars.next(); }
            '"' => {
                chars.next();
                let mut text = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '"' { chars.next(); break; }
                    text.push(chars.next().unwrap());
                }
                tokens.push(Token::Text(text));
            }
            ' ' | '\n' | '\r' | '\t' => { chars.next(); } // Ignore whitespace
            _ if c.is_ascii_digit() => {
                let mut num_str = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() { num_str.push(chars.next().unwrap()); } else { break; }
                }
                tokens.push(Token::Num(num_str.parse().unwrap()));
            }
            _ if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphanumeric() || next == '_' { ident.push(chars.next().unwrap()); } else { break; }
                }
                // Map identifiers to reserved keywords
                match ident.as_str() {
                    "sequence" => tokens.push(Token::Sequence),
                    "display" => tokens.push(Token::Display),
                    "set" => tokens.push(Token::Set),
                    "evolve" => tokens.push(Token::Evolve),
                    "to" => tokens.push(Token::To),
                    "assess" => tokens.push(Token::Assess),
                    "fallback" => tokens.push(Token::Fallback),
                    "cycle" => tokens.push(Token::Cycle),
                    "invoke" => tokens.push(Token::Invoke),
                    "with" => tokens.push(Token::With),
                    _ => tokens.push(Token::Ident(ident)),
                }
            }
            _ => { chars.next(); }
        }
    }
    tokens
}

// ==========================================
// 2. THE ABSTRACT SYNTAX TREE (AST)
// Structures tokens into logical, nested operations.
// ==========================================
#[derive(Debug, Clone)]
enum Expr {
    Text(String), Num(i128), Variable(String),
    Add(Box<Expr>, Box<Expr>), Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>), Div(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>), Neq(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>), Gt(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
enum Statement {
    Display(Expr),
    Set(String, Expr),
    Evolve(String, Expr),
    Assess(Expr, Vec<Statement>, Option<Vec<Statement>>),
    Cycle(Expr, Vec<Statement>),
    Invoke(String, Expr), // Bridges logic to host OS capabilities
}

struct Parser { tokens: Vec<Token>, pos: usize }

impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Parser { tokens, pos: 0 } }
    fn current(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn consume(&mut self) -> Option<Token> { let t = self.current().cloned(); self.pos += 1; t }
    fn expect(&mut self, expected: Token, err: &str) { if self.consume() != Some(expected) { abort(err); } }

    // Parses mathematical and comparative expressions
    fn parse_expr(&mut self) -> Expr {
        let left = self.parse_primary();
        if let Some(tok) = self.current() {
            if matches!(tok, Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::EqEq | Token::NotEq | Token::Lt | Token::Gt) {
                let op = self.consume().unwrap();
                let right = self.parse_primary();
                return match op {
                    Token::Plus => Expr::Add(Box::new(left), Box::new(right)),
                    Token::Minus => Expr::Sub(Box::new(left), Box::new(right)),
                    Token::Star => Expr::Mul(Box::new(left), Box::new(right)),
                    Token::Slash => Expr::Div(Box::new(left), Box::new(right)),
                    Token::EqEq => Expr::Eq(Box::new(left), Box::new(right)),
                    Token::NotEq => Expr::Neq(Box::new(left), Box::new(right)),
                    Token::Lt => Expr::Lt(Box::new(left), Box::new(right)),
                    Token::Gt => Expr::Gt(Box::new(left), Box::new(right)),
                    _ => unreachable!(),
                };
            }
        }
        left
    }

    fn parse_primary(&mut self) -> Expr {
        match self.consume() {
            Some(Token::Num(n)) => Expr::Num(n),
            Some(Token::Text(t)) => Expr::Text(t),
            Some(Token::Ident(id)) => Expr::Variable(id),
            _ => abort("[OMNIA-ERR-02] Syntax Error: Expected valid expression or data."),
        }
    }

    // Parses a block of code enclosed in [ ] brackets
    fn parse_block(&mut self) -> Vec<Statement> {
        self.expect(Token::OpenBracket, "[OMNIA-ERR-03] Syntax Error: Expected '[' to open code block.");
        let mut stmts = Vec::new();
        while let Some(tok) = self.current() {
            if *tok == Token::CloseBracket { self.consume(); return stmts; }
            stmts.push(self.parse_statement());
        }
        abort("[OMNIA-ERR-04] Syntax Error: Missing ']' to close code block.");
    }

    // Identifies exactly what command the user is giving
    fn parse_statement(&mut self) -> Statement {
        match self.consume().unwrap() {
            Token::Display => Statement::Display(self.parse_expr()),
            Token::Set => {
                let Token::Ident(name) = self.consume().unwrap() else { abort("[OMNIA-ERR-05] Expected variable name after 'set'."); };
                self.expect(Token::To, "[OMNIA-ERR-06] Expected 'to' in set statement.");
                Statement::Set(name, self.parse_expr())
            }
            Token::Evolve => {
                let Token::Ident(name) = self.consume().unwrap() else { abort("[OMNIA-ERR-07] Expected variable name after 'evolve'."); };
                self.expect(Token::To, "[OMNIA-ERR-08] Expected 'to' in evolve statement.");
                Statement::Evolve(name, self.parse_expr())
            }
            Token::Assess => {
                let condition = self.parse_expr();
                let true_block = self.parse_block();
                let mut false_block = None;
                if self.current() == Some(&Token::Fallback) {
                    self.consume();
                    false_block = Some(self.parse_block());
                }
                Statement::Assess(condition, true_block, false_block)
            }
            Token::Cycle => {
                let condition = self.parse_expr();
                let block = self.parse_block();
                Statement::Cycle(condition, block)
            }
            Token::Invoke => {
                let Token::Text(target) = self.consume().unwrap() else { abort("[OMNIA-ERR-22] Expected target string after 'invoke'."); };
                self.expect(Token::With, "[OMNIA-ERR-23] Expected 'with' in invoke statement.");
                Statement::Invoke(target, self.parse_expr())
            }
            t => abort(&format!("[OMNIA-ERR-09] Unexpected token: {:?}", t)),
        }
    }

    fn parse_program(&mut self) -> Vec<Statement> {
        self.expect(Token::Sequence, "[OMNIA-ERR-10] Program must start with 'sequence'");
        self.expect(Token::Ident("genesis".to_string()), "[OMNIA-ERR-11] Entry sequence must be named 'genesis'");
        self.parse_block()
    }
}

// Global panic handler for clean ASCII error outputs
fn abort(msg: &str) -> ! { eprintln!("[FATAL] {}", msg); process::exit(1); }

// ==========================================
// 3. DIMENSIONAL SCOPING (Environment)
// Ensures memory isolation. Blocks create new dimensions.
// Closing a block perfectly erases that dimension to prevent memory leaks.
// ==========================================
#[derive(Clone, Debug)]
enum Value { Num(i128), Text(String), Bool(bool) }

struct Environment { scopes: Vec<HashMap<String, Value>> }

impl Environment {
    fn new() -> Self { Environment { scopes: vec![HashMap::new()] } }
    
    // Step into a new memory scope (e.g., entering a `cycle` or `assess` block)
    fn enter_dimension(&mut self) { self.scopes.push(HashMap::new()); }
    
    // Step out and destroy the memory scope
    fn exit_dimension(&mut self) { self.scopes.pop(); }
    
    // Look for a variable, starting from the deepest nested scope outwards
    fn get(&self, name: &str) -> Option<Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(val) = scope.get(name) { return Some(val.clone()); }
        }
        None
    }
    
    // Lock a variable immutably in the current dimension
    fn insert(&mut self, name: String, val: Value) { self.scopes.last_mut().unwrap().insert(name, val); }
    
    // Allows `evolve` to update an existing variable in its native dimension
    fn update_evolution(&mut self, name: &str, val: Value) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if scope.contains_key(name) { scope.insert(name.to_string(), val); return true; }
        }
        false
    }
    
    fn contains_in_current(&self, name: &str) -> bool { self.scopes.last().unwrap().contains_key(name) }
}

// ==========================================
// 4. THE VIRTUAL MACHINE (Evaluator)
// Executes the logic generated by the Parser.
// ==========================================
fn evaluate(expr: &Expr, env: &Environment) -> Value {
    match expr {
        Expr::Num(n) => Value::Num(*n),
        Expr::Text(t) => Value::Text(t.clone()),
        Expr::Variable(id) => env.get(id.as_str()).unwrap_or_else(|| abort(&format!("[OMNIA-ERR-12] Memory Error: '{}' is undefined in this dimension.", id))),
        
        // Polymorphic Data Fusion (Safely combining Text and Numbers)
        Expr::Add(l, r) => match (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) {
            (Value::Num(a), Value::Num(b)) => Value::Num(a + b),
            (Value::Text(a), Value::Text(b)) => Value::Text(format!("{}{}", a, b)),
            (Value::Text(a), Value::Num(b)) => Value::Text(format!("{}{}", a, b)),
            (Value::Num(a), Value::Text(b)) => Value::Text(format!("{}{}", a, b)),
            _ => abort("[OMNIA-ERR-13] Type Error: Cannot fuse these types."),
        },
        Expr::Sub(l, r) => if let (Value::Num(a), Value::Num(b)) = (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) { Value::Num(a - b) } else { abort("[OMNIA-ERR-14] Math Error") },
        Expr::Mul(l, r) => if let (Value::Num(a), Value::Num(b)) = (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) { Value::Num(a * b) } else { abort("[OMNIA-ERR-15] Math Error") },
        
        Expr::Div(l, r) => if let (Value::Num(a), Value::Num(b)) = (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) { 
            if b == 0 { abort("[OMNIA-ERR-16] Math Error: Div by 0"); } 
            Value::Num(a / b) 
        } else { abort("[OMNIA-ERR-17] Math Error") },
        
        Expr::Eq(l, r) => if let (Value::Num(a), Value::Num(b)) = (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) { Value::Bool(a == b) } else { abort("[OMNIA-ERR-18] Type Error") },
        Expr::Neq(l, r) => if let (Value::Num(a), Value::Num(b)) = (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) { Value::Bool(a != b) } else { abort("[OMNIA-ERR-18] Type Error") },
        Expr::Lt(l, r) => if let (Value::Num(a), Value::Num(b)) = (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) { Value::Bool(a < b) } else { abort("[OMNIA-ERR-18] Type Error") },
        Expr::Gt(l, r) => if let (Value::Num(a), Value::Num(b)) = (evaluate(l.as_ref(), env), evaluate(r.as_ref(), env)) { Value::Bool(a > b) } else { abort("[OMNIA-ERR-18] Type Error") },
    }
}

fn run_block(stmts: &[Statement], env: &mut Environment) {
    env.enter_dimension(); // Protect scope
    for stmt in stmts {
        match stmt {
            Statement::Set(name, expr) => {
                if env.contains_in_current(name) { abort(&format!("[OMNIA-ERR-19] IMMUTABILITY BREACH: '{}' is locked in this dimension.", name)); }
                let val = evaluate(expr, env); env.insert(name.clone(), val);
            }
            Statement::Evolve(name, expr) => {
                let val = evaluate(expr, env);
                if !env.update_evolution(name, val) { abort(&format!("[OMNIA-ERR-20] EVOLUTION ERROR: '{}' not found in any active dimension.", name)); }
            }
            Statement::Display(expr) => match evaluate(expr, env) {
                Value::Num(n) => println!("  [OUT] {}", n),
                Value::Text(t) => println!("  [OUT] {}", t),
                Value::Bool(b) => println!("  [OUT] {}", b),
            },
            Statement::Assess(cond, true_blk, false_blk) => {
                if let Value::Bool(b) = evaluate(cond, env) {
                    if b { run_block(true_blk, env); } else if let Some(fb) = false_blk { run_block(fb, env); }
                } else { abort("[OMNIA-ERR-21] Assess requires boolean."); }
            }
            Statement::Cycle(cond, block) => {
                while let Value::Bool(true) = evaluate(cond, env) { run_block(block, env); }
            }
            Statement::Invoke(target, expr) => {
                // Host communication logic
                let payload = evaluate(expr, env);
                let payload_str = match payload { Value::Num(n) => n.to_string(), Value::Text(t) => t, Value::Bool(b) => b.to_string() };
                println!("  [HOST] Targeting: '{}' | Payload: {}", target, payload_str);
            }
        }
    }
    env.exit_dimension(); // Purge scope
}

// --- BOOT SEQUENCE ---
fn main() {
    let omnia_code = r#"
        sequence genesis [
            display "Omni-Drive Universal Engine Online."
            invoke "system.log" with "Bridging eternal logic with legacy host."
        ]
    "#;
    println!("[SYS] Booting Universal Architecture V1.0...\n");
    let tokens = lex(omnia_code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse_program();
    let mut memory = Environment::new();
    run_block(&ast, &mut memory);
    println!("\n[OK] Process Complete. Interoperability successful.");
}


