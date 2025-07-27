# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Build
```bash
cargo build
cargo build --release
```

### Test
```bash
cargo test
cargo test -- --nocapture  # Show println! output
cargo test <test_name>     # Run specific test
```

### Lint and Format
```bash
cargo fmt              # Format code
cargo fmt --check      # Check formatting
cargo clippy           # Lint code
cargo clippy -- -D warnings  # Treat warnings as errors
```

### Documentation
```bash
cargo doc --open       # Generate and open documentation
```

## Architecture

This is a Rust library that generates fake JSON data using JSONLogic-based configuration. The architecture consists of:

### Core Components

1. **Configuration Parser** (`config.rs`): Parses and validates JSON configuration containing metadata, variables, and schema
2. **JSONLogic Engine** (`engine.rs`): Integrates with `datalogic-rs` to evaluate JSONLogic expressions with preserve_structure mode
3. **Custom Operator** (`operators/fake.rs`): Extends JSONLogic with a `fake` operator that interfaces with `fake-rs`
4. **Variable Resolver**: Manages pre-generated variables that can be referenced across the schema
5. **Data Generator** (`generator.rs`): Orchestrates the generation process and produces the final JSON output

### Key Dependencies

- `datalogic-rs`: Provides high-performance JSONLogic expression evaluation with custom operator support
- `fake`: Generates realistic fake data (addresses, names, numbers, etc.)
- `serde`/`serde_json`: JSON serialization and deserialization
- `rand`: Random number generation (used by fake-rs)

### Configuration Flow

1. JSON configuration is parsed into three sections: metadata, variables, and schema
2. Variables are evaluated as a single expression using preserve_structure mode for efficiency
3. Schema is processed as JSONLogic expressions with the custom `fake` operator registered
4. Custom `fake` operator is invoked when encountered in expressions
5. Variable references are resolved from the context
6. Final JSON structure is assembled and returned

### Custom Operator Implementation

The `fake` operator is registered with datalogic-rs using `register_simple_operator` and supports:
- Locale-specific: `{"fake": ["method_name", "locale"]}`
- Numeric ranges: `{"fake": ["type", min, max]}`
- Simple methods: `{"fake": ["method_name"]}`

The operator handler (`fake_operator_handler`) converts between `DataValue` and `serde_json::Value` types.

### Recent Improvements

- Moved custom operator handler to `operators/fake.rs` for better code organization
- Optimized variable generation to evaluate all variables in a single pass
- Leverages datalogic-rs preserve_structure mode for efficient object evaluation
- Thread-local DataLogic instances for safe concurrent usage