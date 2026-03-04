// 🌌 Omni-Drive Engine: Distinct Syntax Edition
// This engine reads the unique 'pulse' and 'emit' commands of Omnia.

#[derive(Debug, PartialEq)]
enum Token {
    Pulse, Emit, OpenBracket, CloseBracket,
    Identifier(String), StringLiteral(String),
}

// ---------------------------------------------------------
// 1. THE LEXER: Translates the alien syntax
// ---------------------------------------------------------
fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '[' => { tokens.push(Token::OpenBracket); chars.next(); }
            ']' => { tokens.push(Token::CloseBracket); chars.next(); }
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
                // The new Omnia unique keywords!
                match ident.as_str() {
                    "pulse" => tokens.push(Token::Pulse),
                    "emit" => tokens.push(Token::Emit),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            _ => { chars.next(); } // Skip unknown
        }
    }
    tokens
}

// ---------------------------------------------------------
// 2. THE PARSER: Understands the structural logic
// ---------------------------------------------------------
#[derive(Debug)]
enum Statement {
    Emit(String),
}

#[derive(Debug)]
struct PulseFunction {
    name: String,
    body: Vec<Statement>,
}

fn parse(tokens: Vec<Token>) -> Vec<PulseFunction> {
    let mut functions = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        // Look for: pulse <name> [ ... ]
        if tokens[i] == Token::Pulse {
            if let Token::Identifier(ref name) = tokens[i+1] {
                if tokens[i+2] == Token::OpenBracket {
                    let mut body = Vec::new();
                    i += 3; 
                    
                    // Parse statements inside the brackets
                    while i < tokens.len() && tokens[i] != Token::CloseBracket {
                        if tokens[i] == Token::Emit {
                            if let Token::StringLiteral(ref text) = tokens[i+1] {
                                body.push(Statement::Emit(text.clone()));
                                i += 1; 
                            }
                        }
                        i += 1;
                    }
                    functions.push(PulseFunction { name: name.clone(), body });
                }
            }
        }
        i += 1;
    }
    functions
}

// ---------------------------------------------------------
// 3. THE INTERPRETER: Executes the Genesis block
// ---------------------------------------------------------
fn run(functions: Vec<PulseFunction>) {
    for func in functions {
        // Programs now start at 'genesis', not 'main'
        if func.name == "genesis" {
            for stmt in func.body {
                match stmt {
                    Statement::Emit(text) => println!("  📡 {}", text),
                }
            }
        }
    }
}

// ---------------------------------------------------------
// IGNITION: Tie it all together
// ---------------------------------------------------------
fn main() {
    // Look at how distinct and clean the Omnia language is now!
    let omnia_code = r#"
        pulse genesis [
            emit "Signal received."
            emit "Omnia engine is online."
        ]
    "#;

    println!("🚀 Booting Omni-Drive Engine...");
    
    let tokens = lex(omnia_code);
    let ast = parse(tokens);
    
    println!("⚙️ Executing Omnia Code:\n");
    run(ast);
    
    println!("\n✅ Process finished. Zero bloat detected.");
}
