# Introduction

**datafake-rs** is a high-performance Rust library for generating realistic fake JSON data using JSONLogic-based configuration. It's designed for testing, prototyping, and development scenarios where you need realistic mock data.

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
