//! WebAssembly bindings for datafake-rs fake data generator.
//!
//! This crate provides WASM bindings that allow using datafake-rs from JavaScript/TypeScript.
//!
//! # Usage
//!
//! ```javascript
//! import init, { generate, FakeDataGenerator } from 'datafake-wasm';
//!
//! await init();
//!
//! // One-off generation
//! const config = JSON.stringify({
//!     schema: {
//!         id: { fake: ["uuid"] },
//!         name: { fake: ["name"] }
//!     }
//! });
//! const result = JSON.parse(generate(config));
//!
//! // Reusable generator
//! const gen = new FakeDataGenerator(config);
//! const data1 = JSON.parse(gen.generate());
//! const batch = JSON.parse(gen.generate_batch(10));
//! gen.free();
//! ```

use datafake_rs::DataGenerator;
use wasm_bindgen::prelude::*;

/// Initialize the WASM module.
///
/// This is automatically called when the module loads.
/// Sets up the panic hook for better error messages.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Generate fake data from a JSON configuration (one-off).
///
/// # Arguments
/// * `config` - JSON string containing the datafake configuration
///
/// # Returns
/// JSON string of the generated data, or an error message
///
/// # Example
/// ```javascript
/// const config = '{"schema": {"id": {"fake": ["uuid"]}}}';
/// const result = generate(config);
/// console.log(JSON.parse(result));
/// ```
#[wasm_bindgen]
pub fn generate(config: &str) -> Result<String, String> {
    let generator = DataGenerator::from_json(config).map_err(|e| e.to_string())?;
    generator
        .generate()
        .map(|v| v.to_string())
        .map_err(|e| e.to_string())
}

/// A reusable fake data generator from a parsed configuration.
///
/// Use this when you need to generate multiple records with the same schema,
/// as it avoids re-parsing the configuration on each generation.
#[wasm_bindgen]
pub struct FakeDataGenerator {
    inner: DataGenerator,
}

#[wasm_bindgen]
impl FakeDataGenerator {
    /// Create a new FakeDataGenerator from a JSON configuration.
    ///
    /// # Arguments
    /// * `config` - JSON string containing the datafake configuration
    ///
    /// # Example
    /// ```javascript
    /// const gen = new FakeDataGenerator('{"schema": {"id": {"fake": ["uuid"]}}}');
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(config: &str) -> Result<FakeDataGenerator, String> {
        let inner = DataGenerator::from_json(config).map_err(|e| e.to_string())?;
        Ok(FakeDataGenerator { inner })
    }

    /// Generate a single fake data record.
    ///
    /// # Returns
    /// JSON string of the generated data
    pub fn generate(&self) -> Result<String, String> {
        self.inner
            .generate()
            .map(|v| v.to_string())
            .map_err(|e| e.to_string())
    }

    /// Generate multiple fake data records.
    ///
    /// # Arguments
    /// * `count` - Number of records to generate
    ///
    /// # Returns
    /// JSON string of an array containing the generated records
    pub fn generate_batch(&self, count: usize) -> Result<String, String> {
        self.inner
            .generate_batch(count)
            .map(|v| serde_json::to_string(&v).unwrap())
            .map_err(|e| e.to_string())
    }
}
