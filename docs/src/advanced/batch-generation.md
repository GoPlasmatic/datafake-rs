# Batch Generation

When you need to generate multiple records, batch generation is more efficient than calling generate multiple times.

## Rust API

### Basic Batch Generation

```rust
use datafake_rs::DataGenerator;

fn main() -> datafake_rs::Result<()> {
    let config = r#"{
        "schema": {
            "id": {"fake": ["uuid"]},
            "name": {"fake": ["name"]},
            "email": {"fake": ["email"]}
        }
    }"#;

    let generator = DataGenerator::from_json(config)?;

    // Generate 100 records
    let users = generator.generate_batch(100)?;

    println!("Generated {} users", users.len());
    for user in &users {
        println!("{}", user);
    }

    Ok(())
}
```

### Reusing the Generator

The `DataGenerator` can be reused for multiple batch generations:

```rust
let generator = DataGenerator::from_json(config)?;

// Generate different batches
let batch1 = generator.generate_batch(10)?;
let batch2 = generator.generate_batch(20)?;
let batch3 = generator.generate_batch(50)?;
```

## JavaScript/TypeScript API

### Using FakeDataGenerator

```javascript
import init, { FakeDataGenerator } from 'datafake-wasm';

async function main() {
    await init();

    const config = JSON.stringify({
        schema: {
            id: { fake: ["uuid"] },
            name: { fake: ["name"] },
            email: { fake: ["email"] }
        }
    });

    const gen = new FakeDataGenerator(config);

    // Generate 100 records
    const users = JSON.parse(gen.generate_batch(100));

    console.log(`Generated ${users.length} users`);

    // Clean up when done
    gen.free();
}
```

### Memory Management

When using `FakeDataGenerator` in JavaScript, always call `free()` when you're done to release WASM memory:

```javascript
const gen = new FakeDataGenerator(config);
try {
    const batch = gen.generate_batch(1000);
    // Process batch...
} finally {
    gen.free();
}
```

## Try It

Use the Count field to generate multiple records:

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"name":{"fake":["name"]},"email":{"fake":["email"]},"age":{"fake":["u8",18,65]},"city":{"fake":["city_name"]}}}' data-batch="5">
</div>

## Performance Tips

### 1. Reuse the Generator

Creating a generator parses and validates the configuration. Reuse the same generator for multiple batches:

```rust
// Good - parse once, generate many times
let generator = DataGenerator::from_json(config)?;
for _ in 0..100 {
    let batch = generator.generate_batch(1000)?;
}

// Bad - parsing on every iteration
for _ in 0..100 {
    let generator = DataGenerator::from_json(config)?;
    let batch = generator.generate_batch(1000)?;
}
```

### 2. Use Appropriate Batch Sizes

Larger batches are more efficient due to reduced function call overhead:

```rust
// Better - one batch of 10,000
let batch = generator.generate_batch(10000)?;

// Worse - 100 batches of 100
for _ in 0..100 {
    let batch = generator.generate_batch(100)?;
}
```

### 3. Minimize Complex Expressions

Simple schemas generate faster than complex nested expressions:

```json
// Fast
{"schema": {"id": {"fake": ["uuid"]}, "name": {"fake": ["name"]}}}

// Slower (many nested operations)
{"schema": {"data": {"if": [{"==": [{"fake": ["bool"]}, true]}, ...]}}}
```

## Streaming Large Batches

For very large datasets, consider generating in chunks to manage memory:

```rust
let generator = DataGenerator::from_json(config)?;

// Generate 1 million records in chunks
let chunk_size = 10000;
let total_records = 1_000_000;

for chunk in 0..(total_records / chunk_size) {
    let batch = generator.generate_batch(chunk_size)?;
    // Process or write batch to file/database
    process_batch(&batch)?;
}
```

## Output Format

Batch generation returns a JSON array:

```json
[
    {"id": "...", "name": "Alice Smith", "email": "alice@example.com"},
    {"id": "...", "name": "Bob Jones", "email": "bob@example.com"},
    {"id": "...", "name": "Carol Brown", "email": "carol@example.com"}
]
```

Each record in the batch has independently generated values - no two records share fake data (unless using variables, which are regenerated for each record in the batch).
