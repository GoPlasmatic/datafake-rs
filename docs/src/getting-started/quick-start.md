# Quick Start

## Rust Usage

### Basic Generation

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
    let result = generator.generate()?;

    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
```

### Batch Generation

Generate multiple records efficiently:

```rust
use datafake_rs::DataGenerator;

fn main() -> datafake_rs::Result<()> {
    let config = r#"{
        "schema": {
            "id": {"fake": ["uuid"]},
            "name": {"fake": ["name"]}
        }
    }"#;

    let generator = DataGenerator::from_json(config)?;
    let results = generator.generate_batch(100)?;

    println!("Generated {} records", results.len());
    Ok(())
}
```

## JavaScript/TypeScript Usage

### Browser (ES Modules)

```javascript
import init, { generate, FakeDataGenerator } from 'datafake-wasm';

async function main() {
    await init();

    // One-off generation
    const config = JSON.stringify({
        schema: {
            id: { fake: ["uuid"] },
            name: { fake: ["name"] },
            email: { fake: ["email"] }
        }
    });

    const result = JSON.parse(generate(config));
    console.log(result);
}

main();
```

### Reusable Generator

For generating multiple records, use the `FakeDataGenerator` class:

```javascript
import init, { FakeDataGenerator } from 'datafake-wasm';

async function main() {
    await init();

    const config = JSON.stringify({
        schema: {
            id: { fake: ["uuid"] },
            name: { fake: ["name"] }
        }
    });

    const gen = new FakeDataGenerator(config);

    // Generate single records
    const user1 = JSON.parse(gen.generate());
    const user2 = JSON.parse(gen.generate());

    // Generate batch
    const users = JSON.parse(gen.generate_batch(10));

    // Clean up when done
    gen.free();
}

main();
```

### Node.js

```javascript
const { generate, FakeDataGenerator } = require('datafake-wasm');

const config = JSON.stringify({
    schema: {
        id: { fake: ["uuid"] },
        name: { fake: ["name"] }
    }
});

const result = JSON.parse(generate(config));
console.log(result);
```

## Try It Now

<div class="playground-widget" data-config='{"schema":{"id":{"fake":["uuid"]},"name":{"fake":["name"]},"email":{"fake":["email"]},"createdAt":{"fake":["datetime"]}}}'>
</div>
