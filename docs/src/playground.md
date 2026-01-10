# Playground

Generate fake data right in your browser! This playground uses the same WebAssembly-compiled engine that powers the Rust library.

<div id="full-playground"></div>

## How to Use

1. **Configuration** - Enter your datafake configuration in the editor
2. **Count** - Set the number of records to generate (1-100)
3. **Generate** - Press the Generate button or use **Ctrl+Enter** (Cmd+Enter on Mac)
4. **Load Examples** - Use the dropdown to load pre-built example configurations

## Configuration Structure

| Field | Description |
|-------|-------------|
| `metadata` | Optional name/version info |
| `variables` | Pre-generate values to reuse |
| `schema` | Define the output structure |

## The `fake` Operator

The `fake` operator is the core of datafake-rs. Use it to generate various types of fake data:

```json
{"fake": ["type"]}
{"fake": ["type", arg1, arg2]}
```

### Common Examples

| Type | Syntax | Description |
|------|--------|-------------|
| UUID | `{"fake": ["uuid"]}` | UUID v4 identifier |
| Name | `{"fake": ["name"]}` | Full name |
| Email | `{"fake": ["email"]}` | Email address |
| Integer | `{"fake": ["u8", 18, 65]}` | Integer with range |
| Float | `{"fake": ["f64", 0.0, 100.0]}` | Float with range |
| Choice | `{"fake": ["enum", "A", "B", "C"]}` | Random selection |

## Quick Reference

### Personal Data
- `name`, `first_name`, `last_name`, `title`
- `email`, `phone_number`, `username`

### Address
- `street_address`, `city`, `state_name`, `country_name`
- `zip_code`, `latitude`, `longitude`

### Internet
- `ipv4`, `ipv6`, `mac_address`, `domain_name`
- `user_agent`, `password`

### Finance
- `bic`, `bic8`, `bic11`, `iban`
- `credit_card_number`, `currency_code`

### Content
- `word`, `words`, `sentence`, `paragraph`
- `uuid`, `bool`

See [Fake Data Types](fake-data-types/overview.md) for the complete list.
