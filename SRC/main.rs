// 🌌 Omni-Drive Engine: The Complete Mini-Interpreter
// This file Lexes, Parses, and Executes Omnia code.

#[derive(Debug, PartialEq)]
enum Token {
    Fn, Print, OpenBrace, CloseBrace,
    Identifier(String), StringLiteral(String),
}

// ---------------------------------------------------------
// 1. THE LEXER: Turns raw text into bite-sized Tokens
// ---------------------------------------------------------
fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '{' => { tokens.push(Token::OpenBrace); chars.next(); }
            '}' => { tokens.push(Token::CloseBrace); chars.next(); }
            '"' => {
                chars.next(); // Skip opening quote
                let mut text = String::new();
                while let Some(&next) = chars.peek() {
                    if next == '"' { chars.next(); break; }
                    text.push(chars.next().unwrap());
                }
                tokens.push(Token::StringLiteral(text));
            }
            ' ' | '\n' | '\r' | '\t' => { chars.next(); } // Skip whitespace
            _ if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_alphabetic() { ident.push(chars.next().unwrap()); }
                    else { break; }
                }
                match ident.as_str() {
                    "fn" => tokens.push(Token::Fn),
                    "print" => tokens.push(Token::Print),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            _ => { chars.next(); } // Skip unknown characters
        }
    }
    tokens
}

// ---------------------------------------------------------
// 2. THE PARSER: Builds an Abstract Syntax Tree (AST)
// ---------------------------------------------------------
#[derive(Debug)]
enum Statement {
    Print(String),
}

#[derive(Debug)]
struct Function {
    name: String,
    body: Vec<Statement>,
}

fn parse(tokens: Vec<Token>) -> Vec<Function> {
    let mut functions = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        // Look for: fn <name> { ... }
        if tokens[i] == Token::Fn {
            if let Token::Identifier(ref name) = tokens[i+1] {
                if tokens[i+2] == Token::OpenBrace {
                    let mut body = Vec::new();
                    i += 3; // Jump inside the function body
                    
                    // Parse the statements inside
                    while i < tokens.len() && tokens[i] != Token::CloseBrace {
                        if tokens[i] == Token::Print {
                            if let Token::StringLiteral(ref text) = tokens[i+1] {
                                body.push(Statement::Print(text.clone()));
                                i += 1; // Skip the string
                            }
                        }
                        i += 1;
                    }
                    functions.push(Function { name: name.clone(), body });
                }
            }
        }
        i += 1;
    }
    functions
}

// ---------------------------------------------------------
// 3. THE INTERPRETER: Executes the logic
// ---------------------------------------------------------
fn run(functions: Vec<Function>) {
    for func in functions {
        // Find the main function and run its contents
        if func.name == "main" {
            for stmt in func.body {
                match stmt {
                    Statement::Print(text) => println!("  > {}", text),
                }
            }
        }
    }
}

// ---------------------------------------------------------
// IGNITION: Tie it all together
// ---------------------------------------------------------
fn main() {
    // Here is a piece of raw Omnia code!
    let omnia_code = r#"
        fn main {
            print "Event Horizon Ignited!"
            print "Welcome to the universe, Omnia."
        }
    "#;

    println!("🚀 Booting Omni-Drive Engine...");
    
    // Step 1: Lex
    let tokens = lex(omnia_code);
    
    // Step 2: Parse
    let ast = parse(tokens);
    
    // Step 3: Execute!
    println!("⚙️ Executing Omnia Code:\n");
    run(ast);
    
    println!("\n✅ Process finished with zero bloat.");
}
