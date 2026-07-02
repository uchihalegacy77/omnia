# Omnia Language - The Universal Architecture

## Overview

**Omnia** is a zero-bloat, Turing-complete programming language engineered for:

- **Infinite Precision Mathematics** - Perform cosmic-scale calculations without overflow
- **Dimensional Memory Scoping** - Prevent memory leaks through structured memory isolation  
- **Pure ASCII Core** - Clean, minimal, human-readable syntax
- **Zero External Dependencies** - Pure Rust implementation, uncompromising security

## Quick Start

### Installation

```bash
git clone https://github.com/uchihalegacy77/omnia.git
cd omnia
cargo build --release
```

### Interactive REPL

```bash
cargo run
```

### Run a Script

```bash
cargo run -- script.omnia
```

## Language Guide

### Variables

```omnia
set x to 42
set message to "Hello, Omnia!"
set pi to 3.14159

// Update variable
evolve x to x + 1
```

### Display Output

```omnia
display "Welcome!"
display 42
display true
```

### Arithmetic

```omnia
set a to 100
set b to 25
display a + b  // 125
display a - b  // 75
display a * b  // 2500
display a / b  // 4
display a % b  // 0
```

### Comparisons

```omnia
if x == 5 { display "x equals 5" }
if x != 5 { display "x is not 5" }
if x < 10 { display "x is less than 10" }
if x > 0 { display "x is positive" }
```

### Logic

```omnia
if x > 5 && y < 10 { display "both true" }
if x == 5 || y == 5 { display "at least one true" }
if !done { display "not done" }
```

### Conditionals

```omnia
if age >= 18 {
    display "You are an adult"
} else {
    display "You are a minor"
}
```

### Arrays

```omnia
set numbers to [1, 2, 3, 4, 5]
set mixed to [42, "hello", true]
```

### Functions

```omnia
sequence greet {
    display "Hello, World!"
}
```

## REPL Commands

- `help` - Show help information
- `clear` - Reset interpreter state
- `exit` or `quit` - Exit the REPL

## Features

✅ Infinite precision integers (no overflow)
✅ IEEE 754 floating point numbers
✅ String operations (concatenation)
✅ Arrays and objects
✅ Conditional statements (if/else)
✅ Logical operators (&&, ||, !)
✅ Function definitions (sequence)
✅ Variable scoping
✅ Comprehensive error messages
✅ Memory bounds checking
✅ Stack overflow protection
✅ Safe arithmetic operations
✅ Division by zero detection

## Safety & Limits

Omnia implements safety guardrails:

- **Recursion Depth**: Max 1,000 levels
- **Token Count**: Max 1,000,000 per file
- **String Length**: Max 10,000,000 characters
- **Number Digits**: Max 100,000 digits
- **Array Size**: Max 10,000,000 elements
- **Object Size**: Max 1,000,000 entries
- **Instructions**: Protected from infinite loops

## Error Types

- `[LEXER]` - Invalid token or syntax
- `[PARSER]` - Parse error
- `[RUNTIME]` - Execution error
- `[TYPE]` - Type mismatch
- `[VALUE]` - Invalid value
- `[NAME]` - Undefined variable
- `[OVERFLOW]` - Arithmetic overflow
- `[STACK]` - Stack/recursion overflow
- `[MEMORY]` - Memory allocation error

## Examples

### Example 1: Arithmetic

```omnia
set cosmic_scale to 999999999999999999999999999999999999
set multiplier to 1000000000000000000000000000000000000
set total to cosmic_scale * multiplier
display total
```

### Example 2: Conditional Logic

```omnia
set age to 25
if age >= 18 {
    display "You can vote"
} else {
    display "You cannot vote yet"
}
```

### Example 3: Arrays

```omnia
set scores to [95, 87, 92, 88, 91]
set first_score to scores
display first_score
```

## Project Structure

```
omnia/
├── src/
│   ├── lib.rs           # Library root
│   ├── main.rs          # CLI entry point
│   ├── lexer.rs         # Tokenization
│   ├── parser.rs        # AST generation
│   ├── interpreter.rs   # Execution engine
│   ├── value.rs         # Value system
│   ├── error.rs         # Error types
│   ├── validator.rs     # Validation
│   └── runtime.rs       # Runtime context
├── Cargo.toml           # Package manifest
├── README.md            # User guide
└── LICENSE              # MIT License
```

## Performance

Omnia prioritizes **correctness and safety** over speed:

- Build with `--release` for optimizations
- Large number operations have CPU overhead due to string representation
- Suitable for correctness-critical applications

## Contributing

Contributions welcome!

1. Fork the repository
2. Create a feature branch
3. Add tests for new functionality
4. Ensure `cargo test` and `cargo clippy` pass
5. Submit a pull request

## License

Omnia is licensed under the MIT License - see LICENSE file for details.

---

**Omnia: Yesterday's dreams, today's reality, tomorrow's infinity.**
