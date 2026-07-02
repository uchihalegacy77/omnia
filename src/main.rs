//! Omnia Language CLI
//! 
//! Command-line interface for the Omnia language interpreter

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

use omnia::{Lexer, Parser, Interpreter, Result};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            if let Err(e) = run_repl() {
                eprintln!("REPL Error: {}", e);
                process::exit(1);
            }
        }
        2 => {
            if let Err(e) = run_file(&args[1]) {
                eprintln!("File Execution Error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Usage: omnia [script.omnia]");
            process::exit(1);
        }
    }
}

/// Run the interactive REPL (Read-Eval-Print Loop)
fn run_repl() -> Result<()> {
    println!("\n╔══════════════════════════════════════════════╗");
    println!("║   Omnia v1.0.0 - The Universal Architecture  ║");
    println!("║   Type 'help' for commands, 'exit' to quit    ║");
    println!("╚══════════════════════════════════════════════╝\n");

    let mut interpreter = Interpreter::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();
    let mut line_count = 0usize;

    loop {
        write!(stdout, "omnia({:04}) > ", line_count).ok();
        let _ = stdout.flush();

        buffer.clear();
        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                println!("\nGoodbye.");
                break;
            }
            Ok(_) => {
                line_count = line_count.saturating_add(1);
                let input = buffer.trim();
                
                match input {
                    "exit" | "quit" => {
                        println!("Goodbye.");
                        break;
                    }
                    "help" => print_help(),
                    "clear" => {
                        interpreter = Interpreter::new();
                        println!("✓ Interpreter state cleared.");
                    }
                    "" => continue,
                    _ => {
                        if let Err(e) = execute_repl_line(&mut interpreter, input) {
                            eprintln!("✗ Error: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("\n✗ Input Error: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

/// Print help information
fn print_help() {
    println!("\n┌─ Available Commands ──────────────────────┐");
    println!("│  help         - Show this help message      │");
    println!("│  clear        - Clear interpreter state     │");
    println!("│  exit/quit    - Exit the REPL               │");
    println!("└───────────────────────────────────────────┘");
    println!("\n┌─ Language Features ───────────────────────┐");
    println!("│  set VAR to VALUE      - Create variable   │");
    println!("│  evolve VAR to VALUE   - Update variable   │");
    println!("│  display EXPR          - Print expression  │");
    println!("│  sequence NAME { ... } - Define function   │");
    println!("│  if COND { ... }       - Conditional       │");
    println!("│  if C { ... } else {}  - If-else block     │");
    println!("└───────────────────────────────────────────┘\n");
}

/// Execute a single line in the REPL
fn execute_repl_line(interpreter: &mut Interpreter, input: &str) -> Result<()> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let result = interpreter.execute(&ast)?;
    
    match result {
        omnia::value::Value::Null => {},
        _ => println!("✓ {}", result),
    }
    
    Ok(())
}

/// Run a script file
fn run_file(path: &str) -> Result<()> {
    if !Path::new(path).exists() {
        return Err(omnia::error::OmniaError::RuntimeError(
            format!("File not found: {}", path)
        ));
    }

    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            return Err(omnia::error::OmniaError::RuntimeError(
                format!("Failed to read file: {}", e)
            ));
        }
    };
    
    let mut interpreter = Interpreter::new();
    let mut lexer = Lexer::new(&source);
    let tokens = lexer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    let _ = interpreter.execute(&ast)?;

    Ok(())
}
