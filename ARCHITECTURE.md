# Omnia Architecture

## Overview

Omnia is a multi-layered interpreter built with safety and correctness as primary concerns.

## Components

### Lexer (`src/lexer.rs`)
- **Purpose**: Converts source code into tokens
- **Safety**:
  - Token limit: 1,000,000
  - String length limit: 10,000,000
  - Number digit limit: 100,000
  - Identifier length limit: 10,000
  - Comment handling
  - Error recovery

### Parser (`src/parser.rs`)
- **Purpose**: Builds Abstract Syntax Tree from tokens
- **Features**:
  - Recursive descent parsing
  - Operator precedence
  - Error messages
  - Support for: sequences, set/evolve, display, if/else, arrays, blocks

### Interpreter (`src/interpreter.rs`)
- **Purpose**: Executes AST nodes
- **Safety**:
  - Recursion depth limit: 1,000
  - Overflow detection (i128 arithmetic)
  - Safe division operations
  - Memory bounds checking
  - Type validation

### Value (`src/value.rs`)
- **Purpose**: Represents runtime values
- **Types**: Integer, Float, Bool, String, Array, Object, Null
- **Features**:
  - Infinite precision integers (as strings)
  - Type checking
  - Truthy/falsy evaluation
  - Safe length operations

### Validator (`src/validator.rs`)
- **Purpose**: Enforces safety limits
- **Checks**:
  - Integer digit count
  - String length
  - Array size
  - Object size
  - Recursion depth

### Runtime (`src/runtime.rs`)
- **Purpose**: Manages execution context
- **Features**:
  - Instruction counting
  - Resource tracking
  - Custom limits support

### Error (`src/error.rs`)
- **Purpose**: Error types and handling
- **Error Types**: Lexer, Parser, Runtime, Type, Value, Name, Dimension, Validation, Overflow, Index, Stack, Memory

## Execution Flow

```
Source Code
    ↓
[Lexer] → Tokens
    ↓
[Parser] → AST
    ↓
[Validator] → Check bounds
    ↓
[Interpreter] → Execute with depth tracking
    ↓
Result
```

## Safety Guarantees

### Bounds Protection
- All collections have size limits
- Numbers have digit limits
- Strings have length limits

### Recursion Protection
- Maximum depth: 1,000
- Checked on every recursive call
- Prevents stack overflow

### Arithmetic Safety
- i128 integers for intermediate calculations
- Overflow detection with checked_add/mul
- Division by zero detection
- Float NaN/Inf handling

### Memory Safety
- Rust's ownership system
- No unsafe code
- Zero external dependencies

## Future Improvements

- [ ] Streaming execution for large files
- [ ] Garbage collection
- [ ] Module system
- [ ] More built-in functions
- [ ] Loop constructs (cycle)
- [ ] Pattern matching
- [ ] Better error recovery
