# Development Guide

## Setup

```bash
git clone https://github.com/uchihalegacy77/omnia.git
cd omnia
cargo build
```

## Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With all warnings
cargo build -- -D warnings
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

## Code Quality

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Lint code
cargo clippy -- -D warnings

# Check all
cargo check
```

## Adding Features

### Adding a New Token Type

1. Add to `TokenType` enum in `src/lexer.rs`
2. Handle in `next_token()` method
3. Add to keyword list if applicable
4. Write tests

### Adding a New AST Node

1. Add to `AstNode` enum in `src/parser.rs`
2. Add parsing logic in `Parser` impl
3. Add execution logic in `Interpreter` impl
4. Add validator checks if needed
5. Write tests

### Adding a New Language Construct

1. Token type (lexer)
2. AST node (parser)
3. Parsing rule (parser)
4. Execution logic (interpreter)
5. Validation (validator if needed)
6. Documentation
7. Tests

## Debugging

### Enable Debug Output

```bash
RUST_LOG=debug cargo run
```

### Using LLDB

```bash
cargo build
lldb ./target/debug/omnia
break set -n main
run
```

### Backtrace

```bash
RUST_BACKTRACE=1 cargo run
```

## Performance Profiling

```bash
# Build with optimizations
cargo build --release

# Run with perf (Linux)
perf record ./target/release/omnia script.omnia
perf report
```

## Safety Checklist

- [ ] Bounds checking on collections
- [ ] Overflow detection on arithmetic
- [ ] Division by zero protection
- [ ] Recursion depth limit checked
- [ ] No unsafe code
- [ ] Error types defined
- [ ] Tests written
- [ ] Documentation updated

## Security Considerations

1. **Input Validation**: Always validate in lexer
2. **Resource Limits**: Enforce bounds on collections
3. **Arithmetic Safety**: Use checked operations
4. **Stack Safety**: Limit recursion depth
5. **No Dependencies**: Reduces attack surface

## Release Process

1. Update version in `Cargo.toml`
2. Run full test suite: `cargo test`
3. Run clippy: `cargo clippy -- -D warnings`
4. Build release: `cargo build --release`
5. Test binary works
6. Commit changes
7. Tag release: `git tag v1.x.x`
8. Create GitHub release

## Known Limitations

- Large number arithmetic is slower (string-based)
- No JIT compilation
- Single-threaded execution
- No external module system
- Limited standard library

## Future Optimizations

- [ ] BigInt library for faster arithmetic
- [ ] Caching for parsed expressions
- [ ] Bytecode compilation
- [ ] Threading support
- [ ] Streaming execution
