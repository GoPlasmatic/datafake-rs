<div align="center">
<img src="[https://avatars.githubusercontent.com/u/207296579?s=200\&v=4](https://avatars.githubusercontent.com/u/207296579?s=200&v=4)" alt="Plasmatic Logo" width="120" height="120">

# datafake-rs

**A high-performance mock JSON data generation library for Rust.**

*Uses JSONLogic for flexible and powerful fake data generation.*

[](https://opensource.org/licenses/Apache-2.0)
[](https://www.rust-lang.org)
[](https://crates.io/crates/datafake-rs)
[](https://github.com/GoPlasmatic/datafake-rs/actions/workflows/ci.yml)
[](https://codecov.io/gh/GoPlasmatic/datafake-rs)

<p>
<a href="[https://github.com/GoPlasmatic](https://github.com/GoPlasmatic)">🏢 Organization</a> •
<a href="[https://docs.rs/datafake-rs](https://docs.rs/datafake-rs)">📖 Docs</a> •
<a href="[https://github.com/GoPlasmatic/datafake-rs/issues](https://github.com/GoPlasmatic/datafake-rs/issues)">🐛 Report a Bug</a>  
</p>
</div>

-----

`datafake-rs` is a Rust library for generating realistic mock JSON data. It uses JSONLogic for its configuration, allowing you to define complex data structures with ease. The library extends JSONLogic with a custom `fake` operator, powered by the `fake-rs` crate, to generate a wide variety of data types for testing, development, and other use cases.

## 🚀 Key Features

  * **JSONLogic Based:** Define data generation rules using JSONLogic expressions.
  * **Rich Fake Data:** Over 50 fake data types, including names, addresses, and financial data.
  * **Variable System:** Pre-generate and reuse values throughout your schema.
  * **Type-Safe:** Ensures strong typing with thorough validation.
  * **Batch Generation:** Easily create multiple, unique data records.
  * **High Performance:** Built on `datalogic-rs` for efficient evaluation.
  * **Flexible Configuration:** Use JSON for easy setup and integration.
  * **Thread-Safe:** Designed for safe concurrent data generation.

## 🏗️ How It Works

### Configuration-Driven Design

You define your data schema using a JSONLogic configuration with custom operators.

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

The library integrates with `datalogic-rs` and adds a `fake` operator, while still supporting all standard JSONLogic operators.

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

### Supported Data Types

The `fake` operator can generate over 50 different types of data.

#### Numeric

```json
{"fake": ["u8"]}
{"fake": ["u16", 100, 1000]}
{"fake": ["i32", -50, 50]}
{"fake": ["f64", 0.0, 1.0]}
```

#### Personal

```json
{"fake": ["name"]}
{"fake": ["first_name"]}
{"fake": ["last_name"]}
{"fake": ["title"]}
{"fake": ["email"]}
{"fake": ["phone_number"]}
```

#### Address

```json
{"fake": ["street_address"]}
{"fake": ["city_name"]}
{"fake": ["state_name"]}
{"fake": ["country_name"]}
{"fake": ["zip_code"]}
{"fake": ["latitude"]}
{"fake": ["longitude"]}
```

#### Financial

```json
{"fake": ["bic"]}
{"fake": ["credit_card_number"]}
{"fake": ["currency_code"]}
{"fake": ["currency_symbol"]}
```

#### Internet

```json
{"fake": ["username"]}
{"fake": ["password", 10, 20]}
{"fake": ["ipv4"]}
{"fake": ["ipv6"]}
{"fake": ["mac_address"]}
{"fake": ["user_agent"]}
{"fake": ["domain_suffix"]}
```

#### Company

```json
{"fake": ["company_name"]}
{"fake": ["industry"]}
{"fake": ["profession"]}
{"fake": ["catch_phrase"]}
```

#### Content

```json
{"fake": ["word"]}
{"fake": ["words", 5]}
{"fake": ["sentence", 5, 10]}
{"fake": ["paragraph", 3, 5]}
{"fake": ["uuid"]}
```

### Variable System

You can pre-generate values and reuse them in your schema.

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

Add `datafake-rs` to your `Cargo.toml`:

```toml
[dependencies]
datafake-rs = "0.1.0"
```

Or use `cargo`:

```bash
cargo add datafake-rs
```

## 📖 Usage Examples

### Basic Example

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
```

### Nested Structures

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
            "employees": [
                {
                    "id": {"fake": ["uuid"]},
                    "name": {"fake": ["name"]},
                    "email": {"fake": ["email"]}
                }
            ]
        }
    }
});

let generator = DataGenerator::from_value(config)?;
let company_data = generator.generate()?;
```

### Batch Generation

Generate a list of unique records.

```rust
let config = json!({
    "schema": {
        "id": {"fake": ["uuid"]},
        "transaction": {
            "amount": {"fake": ["f64", 10.0, 1000.0]},
            "currency": {"fake": ["currency_code"]}
        }
    }
});

let generator = DataGenerator::from_value(config)?;
let transactions = generator.generate_batch(100)?;
```

### Conditional Logic

Use JSONLogic conditions to shape your data.

```rust
let config = json!({
    "variables": {
        "age": {"fake": ["u8", 10, 80]}
    },
    "schema": {
        "age": {"var": "age"},
        "ageGroup": {
            "if": [
                {"<": [{"var": "age"}, 18]}, "minor",
                {"<": [{"var": "age"}, 65]}, "adult",
                "senior"
            ]
        }
    }
});
```

## 🧪 Testing

To run the test suite:

```bash
# Run all tests
cargo test

# Run a specific test with output
cargo test test_generate_batch -- --nocapture
```

## 🤝 Contributing

Contributions are welcome\! If you're submitting a change, please ensure it has good test coverage and documentation.

## 🏢 About Plasmatic

`datafake-rs` is an open-source project from [Plasmatic](https://github.com/GoPlasmatic). We build developer tools that are performant and based on open standards.

## 📄 License

This project is licensed under the Apache License, Version 2.0. See the [LICENSE](https://www.google.com/search?q=LICENSE) file for details.

## 🔗 See Also

  * [datalogic-rs](https://github.com/GoPlasmatic/datalogic-rs): A high-performance JSONLogic implementation for Rust.
  * [fake-rs](https://github.com/cksac/fake-rs): A library for generating fake data in Rust.
  * [JSONLogic](https://jsonlogic.com/): The official JSONLogic specification.

-----

<div align="center">
<p>Built with ❤️ by the <a href="[https://github.com/GoPlasmatic](https://github.com/GoPlasmatic)">Plasmatic</a> team</p>
</div>