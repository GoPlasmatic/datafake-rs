# datafake-wasm

WebAssembly bindings for [datafake-rs](https://github.com/GoPlasmatic/datafake-rs) fake data generator.

## Installation

### Using npm (after publishing)

```bash
npm install datafake-wasm
```

### Building from source

```bash
# Prerequisites
cargo install wasm-pack
rustup target add wasm32-unknown-unknown

# Build for web (ES modules)
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers (webpack, rollup, etc.)
wasm-pack build --target bundler
```

## Usage

### Browser (ES Modules)

```html
<script type="module">
import init, { generate, FakeDataGenerator } from './pkg/datafake_wasm.js';

async function run() {
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
    // { id: "550e8400-e29b-41d4-a716-446655440000", name: "John Doe", email: "john@example.com" }

    // Reusable generator (more efficient for multiple generations)
    const gen = new FakeDataGenerator(config);

    // Generate single record
    const user = JSON.parse(gen.generate());
    console.log(user);

    // Generate batch of records
    const users = JSON.parse(gen.generate_batch(10));
    console.log(users); // Array of 10 user objects

    // Clean up when done
    gen.free();
}

run();
</script>
```

### Node.js

```javascript
const { generate, FakeDataGenerator } = require('./pkg/datafake_wasm.js');

const config = JSON.stringify({
    variables: {
        userId: { fake: ["uuid"] }
    },
    schema: {
        id: { var: "userId" },
        profile: {
            firstName: { fake: ["first_name"] },
            lastName: { fake: ["last_name"] },
            age: { fake: ["u8", 18, 65] }
        },
        address: {
            street: { fake: ["street_address"] },
            city: { fake: ["city_name"] },
            country: { fake: ["country_code"] }
        }
    }
});

// One-off generation
const result = JSON.parse(generate(config));
console.log(result);

// Batch generation
const gen = new FakeDataGenerator(config);
const batch = JSON.parse(gen.generate_batch(100));
console.log(`Generated ${batch.length} records`);
gen.free();
```

### TypeScript

TypeScript definitions are automatically generated. The API is:

```typescript
export function generate(config: string): string;
export function init(): void;

export class FakeDataGenerator {
    constructor(config: string);
    generate(): string;
    generate_batch(count: number): string;
    free(): void;
}
```

## API

### `generate(config: string): string`

One-off generation from a JSON configuration string. Returns a JSON string of the generated data.

### `FakeDataGenerator`

Reusable generator class for efficient multiple generations with the same schema.

- `constructor(config: string)` - Create a new generator from a JSON configuration
- `generate(): string` - Generate a single record
- `generate_batch(count: number): string` - Generate multiple records as a JSON array
- `free(): void` - Free the generator's memory (called automatically when garbage collected)

## Configuration Format

See the [datafake-rs documentation](https://github.com/GoPlasmatic/datafake-rs) for the full configuration format. Basic structure:

```json
{
    "metadata": {
        "name": "My Generator",
        "version": "1.0.0"
    },
    "variables": {
        "userId": {"fake": ["uuid"]}
    },
    "schema": {
        "id": {"var": "userId"},
        "name": {"fake": ["name"]},
        "email": {"fake": ["email"]},
        "age": {"fake": ["u8", 18, 65]}
    }
}
```

## Supported Fake Data Types

- **Identifiers**: `uuid`
- **Names**: `name`, `first_name`, `last_name`, `title`
- **Internet**: `email`, `username`, `password`, `domain_name`, `ipv4`, `ipv6`
- **Address**: `street_address`, `city_name`, `country_code`, `zip_code`, `latitude`, `longitude`
- **Company**: `company_name`, `industry`, `profession`
- **Finance**: `bic`, `credit_card_number`, `currency_code`, `iban`
- **Numbers**: `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64` (with optional range)
- **Text**: `word`, `sentence`, `paragraph`
- **Other**: `bool`, `file_name`, `datetime`, `date`

## Testing

```bash
# Run tests in headless Chrome
wasm-pack test --headless --chrome

# Run tests in headless Firefox
wasm-pack test --headless --firefox
```

## License

Apache-2.0
