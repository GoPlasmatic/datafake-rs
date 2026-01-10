# API Reference

## Rust API

### DataGenerator

The main entry point for generating fake data.

```rust
use datafake_rs::DataGenerator;
```

#### Constructors

##### from_json

Create a generator from a JSON string:

```rust
pub fn from_json(json_str: &str) -> Result<Self>
```

**Example:**
```rust
let config = r#"{"schema": {"id": {"fake": ["uuid"]}}}"#;
let generator = DataGenerator::from_json(config)?;
```

##### from_value

Create a generator from a `serde_json::Value`:

```rust
pub fn from_value(json_value: Value) -> Result<Self>
```

**Example:**
```rust
use serde_json::json;

let config = json!({
    "schema": {
        "id": {"fake": ["uuid"]}
    }
});
let generator = DataGenerator::from_value(config)?;
```

##### new

Create a generator from a `DataFakeConfig`:

```rust
pub fn new(config: DataFakeConfig) -> Self
```

#### Methods

##### generate

Generate a single record:

```rust
pub fn generate(&self) -> Result<Value>
```

**Example:**
```rust
let result = generator.generate()?;
println!("{}", serde_json::to_string_pretty(&result)?);
```

##### generate_batch

Generate multiple records:

```rust
pub fn generate_batch(&self, count: usize) -> Result<Vec<Value>>
```

**Example:**
```rust
let results = generator.generate_batch(100)?;
for result in results {
    println!("{}", result);
}
```

### DataFakeConfig

Configuration structure for the generator.

```rust
pub struct DataFakeConfig {
    pub metadata: Option<Metadata>,
    pub variables: HashMap<String, Value>,
    pub schema: Value,
}
```

### Metadata

Optional metadata for the configuration.

```rust
pub struct Metadata {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub extra: HashMap<String, Value>,
}
```

### Error Types

```rust
pub enum DataFakeError {
    ConfigParse(String),
    InvalidConfig(String),
    VariableNotFound(String),
    JsonError(serde_json::Error),
    FakeOperatorError(String),
    TypeConversion(String),
    InvalidLocale(String),
    InvalidRange(String),
}
```

| Error | Description |
|-------|-------------|
| `ConfigParse` | Failed to parse JSON configuration |
| `InvalidConfig` | Configuration is missing required fields |
| `VariableNotFound` | Referenced variable doesn't exist |
| `JsonError` | JSON serialization/deserialization error |
| `FakeOperatorError` | Invalid fake operator or arguments |
| `TypeConversion` | Type conversion failed |
| `InvalidLocale` | Invalid locale specified |
| `InvalidRange` | Invalid numeric range |

---

## WebAssembly API

### generate

One-off generation from a configuration string:

```typescript
function generate(config: string): string
```

**Parameters:**
- `config` - JSON string containing the datafake configuration

**Returns:**
- JSON string of the generated data

**Throws:**
- Error if configuration is invalid

**Example:**
```javascript
import init, { generate } from 'datafake-wasm';

await init();

const config = JSON.stringify({
    schema: { id: { fake: ["uuid"] } }
});

const result = JSON.parse(generate(config));
```

### FakeDataGenerator

Reusable generator class for multiple generations.

#### Constructor

```typescript
new FakeDataGenerator(config: string)
```

**Parameters:**
- `config` - JSON string containing the datafake configuration

**Throws:**
- Error if configuration is invalid

#### Methods

##### generate

Generate a single record:

```typescript
generate(): string
```

**Returns:**
- JSON string of the generated data

##### generate_batch

Generate multiple records:

```typescript
generate_batch(count: number): string
```

**Parameters:**
- `count` - Number of records to generate

**Returns:**
- JSON string of an array containing the generated records

##### free

Release WASM memory:

```typescript
free(): void
```

**Important:** Always call `free()` when done to prevent memory leaks.

**Example:**
```javascript
import init, { FakeDataGenerator } from 'datafake-wasm';

await init();

const config = JSON.stringify({
    schema: {
        id: { fake: ["uuid"] },
        name: { fake: ["name"] }
    }
});

const gen = new FakeDataGenerator(config);

try {
    const single = JSON.parse(gen.generate());
    const batch = JSON.parse(gen.generate_batch(10));
} finally {
    gen.free();
}
```

---

## Configuration Schema

### Full Configuration

```json
{
    "metadata": {
        "name": "string (optional)",
        "version": "string (optional)",
        "description": "string (optional)",
        "...": "any additional fields"
    },
    "variables": {
        "variableName": "<JSONLogic expression>"
    },
    "schema": "<JSONLogic expression>"
}
```

### fake Operator

```json
{"fake": ["type"]}
{"fake": ["type", arg1, arg2, ...]}
```

See [Fake Data Types](../fake-data-types/overview.md) for all available types.

### var Operator

Reference a variable:

```json
{"var": "variableName"}
{"var": "nested.path"}
```

---

## Complete Example

```rust
use datafake_rs::{DataGenerator, Result};

fn main() -> Result<()> {
    let config = r#"{
        "metadata": {
            "name": "User Generator",
            "version": "1.0.0"
        },
        "variables": {
            "userId": {"fake": ["uuid"]},
            "createdAt": {"fake": ["datetime"]}
        },
        "schema": {
            "id": {"var": "userId"},
            "profile": {
                "name": {"fake": ["name"]},
                "email": {"fake": ["email"]},
                "age": {"fake": ["u8", 18, 65]}
            },
            "metadata": {
                "createdAt": {"var": "createdAt"},
                "createdBy": {"var": "userId"}
            }
        }
    }"#;

    let generator = DataGenerator::from_json(config)?;

    // Generate single record
    let user = generator.generate()?;
    println!("Single user: {}", serde_json::to_string_pretty(&user)?);

    // Generate batch
    let users = generator.generate_batch(10)?;
    println!("Generated {} users", users.len());

    Ok(())
}
```

---

## Links

- [docs.rs/datafake-rs](https://docs.rs/datafake-rs) - Full Rust API documentation
- [GitHub Repository](https://github.com/GoPlasmatic/datafake-rs)
- [npm Package](https://www.npmjs.com/package/datafake-wasm)
