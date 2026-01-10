<div align="center">
<img src="https://avatars.githubusercontent.com/u/207296579?s=200&v=4" alt="Plasmatic Logo" width="120" height="120">

# datafake-rs

**A high-performance mock JSON data generation library for Rust.**

*Uses JSONLogic for flexible and powerful fake data generation.*

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/datafake-rs.svg)](https://crates.io/crates/datafake-rs)

<p>
<a href="https://github.com/GoPlasmatic">Organization</a> •
<a href="https://docs.rs/datafake-rs">Docs</a> •
<a href="https://github.com/GoPlasmatic/datafake-rs/issues">Report a Bug</a>
</p>
</div>

---

**datafake-rs** is designed for testing, prototyping, and development scenarios where you need realistic mock data.

## Features

- **JSONLogic Integration** - Use powerful JSONLogic expressions to define your data schema
- **50+ Fake Data Types** - Generate names, addresses, emails, phone numbers, financial data, and more
- **Variables** - Pre-generate values and reuse them across your schema
- **Batch Generation** - Generate multiple records efficiently
- **WebAssembly Support** - Use in browsers and Node.js via WASM

## Quick Example

Here's a simple example that generates a user profile:

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"name":{"fake":["name"]},"email":{"fake":["email"]},"age":{"fake":["u8",18,65]}}}'>
</div>

## How It Works

1. **Define a Configuration** - Create a JSON configuration with metadata, variables, and schema
2. **Use the `fake` Operator** - Specify which fake data types to generate using the `{"fake": ["type"]}` syntax
3. **Generate Data** - The library evaluates the schema and produces realistic fake data

## Configuration Structure

A datafake configuration has three optional sections:

```json
{
    "metadata": {
        "name": "User Generator",
        "version": "1.0.0"
    },
    "variables": {
        "userId": {"fake": ["uuid"]}
    },
    "schema": {
        "id": {"var": "userId"},
        "name": {"fake": ["name"]},
        "email": {"fake": ["email"]}
    }
}
```

- **metadata** - Optional information about the configuration
- **variables** - Pre-generate values that can be referenced in the schema
- **schema** - The structure of the output data with fake data operators

## Next Steps

- Try the [Playground](playground.md) to experiment with configurations
- Read [Quick Start](getting-started/quick-start.md) for usage examples
- Browse [Fake Data Types](fake-data-types/overview.md) to see all available generators
