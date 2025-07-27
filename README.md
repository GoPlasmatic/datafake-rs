<div align="center">
  <img src="https://avatars.githubusercontent.com/u/207296579?s=200&v=4" alt="Plasmatic Logo" width="120" height="120">
  
  # datafake-rs
  
  **High-Performance JSON Mock Data Generation Library**
  
  *JSONLogic-driven configuration with comprehensive fake data generation*
  
  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
  [![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
  [![Crates.io](https://img.shields.io/crates/v/datafake-rs.svg)](https://crates.io/crates/datafake-rs)
  [![CI](https://github.com/GoPlasmatic/datafake-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/GoPlasmatic/datafake-rs/actions/workflows/ci.yml)
  [![codecov](https://codecov.io/gh/GoPlasmatic/datafake-rs/branch/main/graph/badge.svg)](https://codecov.io/gh/GoPlasmatic/datafake-rs)

  <p>
    <a href="https://github.com/GoPlasmatic">🏢 Organization</a> •
    <a href="https://docs.rs/datafake-rs">📖 Documentation</a> •
    <a href="https://github.com/GoPlasmatic/datafake-rs/issues">🐛 Issues</a>  
  </p>
</div>

---

A high-performance Rust library for generating realistic mock JSON data using **JSONLogic-based configuration**. datafake-rs extends JSONLogic with a custom `fake` operator powered by the comprehensive fake-rs library, enabling type-safe, flexible mock data generation for testing, development, and demonstration purposes.

## 🚀 Key Features

- **JSONLogic-Driven**: Use JSONLogic expressions to define complex data generation rules
- **Comprehensive Fake Data**: 50+ fake data types including names, addresses, financial data, and more
- **Variable System**: Pre-generate values that can be reused across your schema
- **Type-Safe Generation**: Strong typing with comprehensive validation
- **Batch Generation**: Generate multiple records with unique data
- **High Performance**: Leverages datalogic-rs with preserve_structure mode for efficient evaluation
- **Flexible Configuration**: JSON-based configuration for easy integration
- **Thread-Safe**: Thread-local JSONLogic instances for safe concurrent usage

## 🏗️ Architecture Overview

### Configuration-Driven Design

Define your data schema using JSONLogic expressions with custom operators:

```rust
use datafake_rs::DataGenerator;
use serde_json::json;

let config = json!({
    "variables": {
        "userId": {"fake": ["uuid"]},
        "country": {"fake": ["country_code"]}
    },
    "schema": {
        "id": {"var": "userId"},
        "profile": {
            "name": {"fake": ["name"]},
            "email": {"fake": ["email"]},
            "age": {"fake": ["u8", 18, 65]}
        },
        "location": {
            "country": {"var": "country"},
            "city": {"fake": ["city_name"]},
            "coordinates": {
                "lat": {"fake": ["latitude"]},
                "lng": {"fake": ["longitude"]}
            }
        }
    }
});

let generator = DataGenerator::from_value(config)?;
let mock_data = generator.generate()?;
```

### JSONLogic Integration

The library integrates with datalogic-rs and extends it with a powerful `fake` operator while maintaining compatibility with all standard JSONLogic operators:

```json
{
    "schema": {
        "active": {"==": [{"var": "status"}, "active"]},
        "discount": {"if": [
            {">": [{"var": "age"}, 65]},
            0.2,
            0.0
        ]},
        "code": {"fake": ["u32", 1000, 9999]}
    }
}
```

## 🎯 Fake Data Generation

### Comprehensive Data Types

The `fake` operator supports 50+ data generation methods:

#### Numeric Types
```json
{"fake": ["u8"]}              // 0-255
{"fake": ["u16", 100, 1000]}  // 100-1000
{"fake": ["i32", -50, 50]}    // -50 to 50
{"fake": ["f64", 0.0, 1.0]}   // 0.0-1.0
```

#### Personal Data
```json
{"fake": ["name"]}            // "John Smith"
{"fake": ["first_name"]}      // "Jane"
{"fake": ["last_name"]}       // "Doe"
{"fake": ["title"]}           // "Dr."
{"fake": ["email"]}           // "user@example.com"
{"fake": ["phone_number"]}    // "+1-555-0123"
```

#### Address Data
```json
{"fake": ["street_address"]}  // "123 Main St"
{"fake": ["city_name"]}       // "New York"
{"fake": ["state_name"]}      // "California"
{"fake": ["country_name"]}    // "United States"
{"fake": ["zip_code"]}        // "10001"
{"fake": ["latitude"]}        // 40.7128
{"fake": ["longitude"]}       // -74.0060
```

#### Financial Data
```json
{"fake": ["bic"]}             // "DEUTDEFF"
{"fake": ["credit_card_number"]} // "4532-1234-5678-9012"
{"fake": ["currency_code"]}   // "USD"
{"fake": ["currency_symbol"]} // "$"
```

#### Internet Data
```json
{"fake": ["username"]}        // "cooluser123"
{"fake": ["password", 10, 20]} // "xK9#mP2$qR5@nL8"
{"fake": ["ipv4"]}            // "192.168.1.1"
{"fake": ["ipv6"]}            // "2001:0db8:85a3::8a2e:0370:7334"
{"fake": ["mac_address"]}     // "00:1B:44:11:3A:B7"
{"fake": ["user_agent"]}      // "Mozilla/5.0..."
{"fake": ["domain_suffix"]}   // "com"
```

#### Company Data
```json
{"fake": ["company_name"]}    // "Acme Corporation"
{"fake": ["industry"]}        // "Technology"
{"fake": ["profession"]}      // "Software Engineer"
{"fake": ["catch_phrase"]}    // "Innovative solutions for tomorrow"
```

#### Content Generation
```json
{"fake": ["word"]}            // "example"
{"fake": ["words", 5]}        // "lorem ipsum dolor sit amet"
{"fake": ["sentence", 5, 10]} // 5-10 word sentence
{"fake": ["paragraph", 3, 5]} // 3-5 sentence paragraph
{"fake": ["uuid"]}            // "550e8400-e29b-41d4-a716-446655440000"
```

### Variable System

Pre-generate values that can be referenced throughout your schema:

```json
{
    "variables": {
        "sessionId": {"fake": ["uuid"]},
        "timestamp": {"fake": ["u64", 1600000000, 1700000000]},
        "userId": {"fake": ["u32", 1000, 9999]}
    },
    "schema": {
        "session": {"var": "sessionId"},
        "events": [
            {
                "id": {"fake": ["uuid"]},
                "session": {"var": "sessionId"},
                "user": {"var": "userId"},
                "timestamp": {"var": "timestamp"},
                "action": {"fake": ["word"]}
            }
        ]
    }
}
```

## 🔧 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
datafake-rs = "0.1.0"
```

Or use cargo:

```bash
cargo add datafake-rs
```

## 📖 Usage Examples

### Basic Mock Data Generation

```rust
use datafake_rs::DataGenerator;

let config = r#"{
    "schema": {
        "id": {"fake": ["uuid"]},
        "name": {"fake": ["name"]},
        "email": {"fake": ["email"]},
        "age": {"fake": ["u8", 18, 65]},
        "active": {"fake": ["bool"]}
    }
}"#;

let generator = DataGenerator::from_json(config)?;
let mock_user = generator.generate()?;

println!("{}", serde_json::to_string_pretty(&mock_user)?);
// Output:
// {
//   "id": "550e8400-e29b-41d4-a716-446655440000",
//   "name": "John Smith",
//   "email": "john.smith@example.com",
//   "age": 32,
//   "active": true
// }
```

### Complex Nested Structures

```rust
use datafake_rs::DataGenerator;
use serde_json::json;

let config = json!({
    "variables": {
        "companyId": {"fake": ["uuid"]},
        "createdAt": {"fake": ["u64", 1600000000, 1700000000]}
    },
    "schema": {
        "company": {
            "id": {"var": "companyId"},
            "name": {"fake": ["company_name"]},
            "industry": {"fake": ["industry"]},
            "employees": [
                {
                    "id": {"fake": ["uuid"]},
                    "name": {"fake": ["name"]},
                    "title": {"fake": ["profession"]},
                    "email": {"fake": ["email"]},
                    "department": {"fake": ["bs_noun"]}
                }
            ],
            "address": {
                "street": {"fake": ["street_address"]},
                "city": {"fake": ["city_name"]},
                "state": {"fake": ["state_name"]},
                "zip": {"fake": ["zip_code"]},
                "country": {"fake": ["country_name"]}
            },
            "metadata": {
                "createdAt": {"var": "createdAt"},
                "updatedAt": {"fake": ["u64", 1700000000, 1800000000]}
            }
        }
    }
});

let generator = DataGenerator::from_value(config)?;
let company_data = generator.generate()?;
```

### Batch Generation

Generate multiple unique records:

```rust
let config = json!({
    "schema": {
        "id": {"fake": ["uuid"]},
        "transaction": {
            "amount": {"fake": ["f64", 10.0, 1000.0]},
            "currency": {"fake": ["currency_code"]},
            "bic": {"fake": ["bic"]},
            "timestamp": {"fake": ["u64"]},
            "status": {"fake": ["word"]}
        }
    }
});

let generator = DataGenerator::from_value(config)?;
let transactions = generator.generate_batch(100)?;

// Each transaction has unique values
for transaction in &transactions {
    println!("Transaction {}: ${:.2} {}", 
        transaction["id"].as_str().unwrap(),
        transaction["transaction"]["amount"].as_f64().unwrap(),
        transaction["transaction"]["currency"].as_str().unwrap()
    );
}
```

### Using JSONLogic Conditions

Combine fake data generation with JSONLogic conditions:

```rust
let config = json!({
    "variables": {
        "age": {"fake": ["u8", 10, 80]},
        "country": {"fake": ["country_code"]}
    },
    "schema": {
        "age": {"var": "age"},
        "ageGroup": {
            "if": [
                {"<": [{"var": "age"}, 18]}, "minor",
                {"<": [{"var": "age"}, 65]}, "adult",
                "senior"
            ]
        },
        "discount": {
            "if": [
                {"or": [
                    {"<": [{"var": "age"}, 18]},
                    {">": [{"var": "age"}, 65]}
                ]},
                0.2,
                0.0
            ]
        },
        "location": {
            "country": {"var": "country"},
            "requiresVisa": {
                "!": {"in": [{"var": "country"}, ["US", "CA", "UK", "AU"]]}
            }
        }
    }
});
```

### API Testing Mock Server

```rust
use datafake_rs::DataGenerator;
use serde_json::json;

// Define API response schema
let user_schema = json!({
    "schema": {
        "data": {
            "id": {"fake": ["uuid"]},
            "type": "user",
            "attributes": {
                "username": {"fake": ["username"]},
                "email": {"fake": ["email"]},
                "profile": {
                    "firstName": {"fake": ["first_name"]},
                    "lastName": {"fake": ["last_name"]},
                    "avatar": {"fake": ["uuid"]},
                    "bio": {"fake": ["sentence", 10, 20]}
                },
                "settings": {
                    "theme": {"fake": ["word"]},
                    "notifications": {"fake": ["bool"]},
                    "language": {"fake": ["word"]}
                }
            },
            "relationships": {
                "posts": {
                    "data": [
                        {"id": {"fake": ["uuid"]}, "type": "post"},
                        {"id": {"fake": ["uuid"]}, "type": "post"},
                        {"id": {"fake": ["uuid"]}, "type": "post"}
                    ]
                }
            }
        },
        "meta": {
            "requestId": {"fake": ["uuid"]},
            "timestamp": {"fake": ["u64"]}
        }
    }
});

// Generate mock API responses
let generator = DataGenerator::from_value(user_schema)?;
let api_response = generator.generate()?;
```

## 🏗️ Architecture Details

### Core Components

```
datafake-rs/
├── src/
│   ├── lib.rs              # Public API exports
│   ├── config.rs           # Configuration parsing and validation
│   ├── generator.rs        # Main DataGenerator implementation
│   ├── engine.rs           # JSONLogic evaluation engine
│   ├── operators/          # Custom operator implementations
│   │   ├── mod.rs
│   │   └── fake.rs         # Fake data operator (50+ methods)
│   ├── error.rs            # Error types and handling
│   └── types.rs            # Type definitions
```

### JSONLogic Extension Architecture

The library integrates with datalogic-rs by registering a custom operator:

```rust
// Internal flow
1. Register custom 'fake' operator with datalogic-rs
2. Parse JSON configuration containing JSONLogic expressions
3. Evaluate expressions with preserve_structure enabled
4. Custom operator generates fake data on-demand
5. Return fully-evaluated JSON structure
```

### Performance Characteristics

- **Zero-Copy Parsing**: Efficient memory usage during configuration parsing
- **Thread-Local DataLogic**: Avoids mutex contention in multi-threaded environments
- **Preserve Structure Mode**: Evaluates entire objects efficiently without recursion
- **Arena Allocation**: Uses datalogic-rs's arena allocator for efficient memory management
- **Optimized Variable Generation**: Evaluates all variables in a single pass

## 🧪 Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_generate_batch
```

## 🔍 Advanced Features

### Custom Variable Context

Variables can reference other variables for complex relationships:

```json
{
    "variables": {
        "basePrice": {"fake": ["f64", 100.0, 1000.0]},
        "taxRate": {"fake": ["f64", 0.05, 0.15]}
    },
    "schema": {
        "price": {"var": "basePrice"},
        "tax": {"*": [{"var": "basePrice"}, {"var": "taxRate"}]},
        "total": {"+": [
            {"var": "basePrice"},
            {"*": [{"var": "basePrice"}, {"var": "taxRate"}]}
        ]}
    }
}
```

### Error Handling

Comprehensive error reporting with context:

```rust
match DataGenerator::from_json(config) {
    Ok(generator) => {
        // Use generator
    }
    Err(e) => {
        eprintln!("Configuration error: {}", e);
        // Detailed error information available
        match e {
            DataFakeError::ConfigParse(msg) => {
                eprintln!("Parse error: {}", msg);
            }
            DataFakeError::InvalidConfig(msg) => {
                eprintln!("Invalid configuration: {}", msg);
            }
            _ => {}
        }
    }
}
```

## 🤝 Contributing

We welcome contributions! Please ensure:
- Comprehensive test coverage
- Documentation for new fake data types
- Performance benchmarks for new features
- Backwards compatibility

## 🏢 About Plasmatic

datafake-rs is developed by [Plasmatic](https://github.com/GoPlasmatic), a technology organization focused on building open-source developer tools. We believe in:

- **🔓 Open Source**: Transparent, community-driven development
- **⚡ Performance**: High-performance solutions for real-world needs
- **🛠️ Developer Experience**: Tools that developers love to use
- **🌍 Standards-Based**: Building on established standards like JSONLogic

## 📄 License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE))

## 🔗 See Also

- [datalogic-rs](https://github.com/GoPlasmatic/datalogic-rs) - High-performance JSONLogic implementation for Rust
- [fake-rs](https://github.com/cksac/fake-rs) - Comprehensive fake data generation library
- [JSONLogic](https://jsonlogic.com/) - The logical rules specification

---

<div align="center">
  <p>Built with ❤️ by the <a href="https://github.com/GoPlasmatic">Plasmatic</a> team</p>
</div>