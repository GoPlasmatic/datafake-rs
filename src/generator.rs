//! High-level API for generating fake data.
//!
//! This module provides the [`DataGenerator`] struct, which is the main entry point
//! for generating fake JSON data from a configuration.

use crate::config::ConfigParser;
use crate::engine::Engine;
use crate::error::Result;
use crate::types::{DataFakeConfig, GenerationContext};
use serde_json::Value;

/// The main struct for generating fake JSON data.
///
/// `DataGenerator` takes a configuration and provides methods to generate
/// single records or batches of fake data based on the schema.
///
/// # Example
///
/// ```rust
/// use datafake_rs::DataGenerator;
///
/// let config = r#"{
///     "schema": {
///         "id": {"fake": ["uuid"]},
///         "name": {"fake": ["name"]}
///     }
/// }"#;
///
/// let generator = DataGenerator::from_json(config).unwrap();
/// let data = generator.generate().unwrap();
/// ```
pub struct DataGenerator {
    config: DataFakeConfig,
}

impl DataGenerator {
    /// Creates a new `DataGenerator` from an already-parsed configuration.
    pub fn new(config: DataFakeConfig) -> Self {
        Self { config }
    }

    /// Creates a new `DataGenerator` by parsing a JSON configuration string.
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON is invalid or the configuration fails validation.
    pub fn from_json(json_str: &str) -> Result<Self> {
        let config = ConfigParser::parse(json_str)?;
        Ok(Self::new(config))
    }

    /// Creates a new `DataGenerator` from a `serde_json::Value`.
    ///
    /// # Errors
    ///
    /// Returns an error if the value cannot be converted to a valid configuration.
    pub fn from_value(json_value: Value) -> Result<Self> {
        let config = ConfigParser::parse_value(json_value)?;
        Ok(Self::new(config))
    }

    /// Generates a single fake data record based on the schema.
    ///
    /// Each call generates fresh random data. Variables are evaluated once
    /// per call and can be referenced multiple times within the schema.
    ///
    /// # Errors
    ///
    /// Returns an error if data generation fails (e.g., invalid operator usage).
    pub fn generate(&self) -> Result<Value> {
        // First, convert HashMap to Map for engine
        let variables_map: serde_json::Map<String, Value> = self
            .config
            .variables
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Generate all variables
        let generated_vars = Engine::generate_variables(&variables_map)?;

        // Create context with generated variables (convert back to HashMap)
        let context = GenerationContext::with_variables(generated_vars.into_iter().collect());

        // Process the schema with the context
        Engine::process_schema(&self.config.schema, &context)
    }

    /// Generates multiple fake data records.
    ///
    /// Each record is independently generated with fresh random data.
    ///
    /// # Arguments
    ///
    /// * `count` - The number of records to generate.
    ///
    /// # Errors
    ///
    /// Returns an error if any record generation fails.
    pub fn generate_batch(&self, count: usize) -> Result<Vec<Value>> {
        let mut results = Vec::with_capacity(count);

        for _ in 0..count {
            results.push(self.generate()?);
        }

        Ok(results)
    }

    /// Returns a reference to the underlying configuration.
    pub fn config(&self) -> &DataFakeConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_from_json() {
        let config_json = r#"{
            "schema": {
                "id": {"fake": ["uuid"]},
                "name": {"fake": ["name"]}
            }
        }"#;

        let generator = DataGenerator::from_json(config_json).unwrap();
        let result = generator.generate().unwrap();

        assert!(result["id"].is_string());
        assert!(result["name"].is_string());
    }

    #[test]
    fn test_with_variables() {
        let config_json = r#"{
            "variables": {
                "userId": {"fake": ["uuid"]},
                "country": {"fake": ["country_code"]}
            },
            "schema": {
                "id": {"var": "userId"},
                "location": {
                    "country": {"var": "country"},
                    "city": {"fake": ["city_name"]}
                }
            }
        }"#;

        let generator = DataGenerator::from_json(config_json).unwrap();
        let result = generator.generate().unwrap();

        assert!(result["id"].is_string());
        assert_eq!(result["id"], result["id"]); // Should be same for single generation
        assert!(result["location"]["country"].is_string());
        assert!(result["location"]["city"].is_string());
    }

    #[test]
    fn test_generate_batch() {
        let config_json = r#"{
            "schema": {
                "id": {"fake": ["uuid"]},
                "timestamp": {"fake": ["u64"]}
            }
        }"#;

        let generator = DataGenerator::from_json(config_json).unwrap();
        let results = generator.generate_batch(5).unwrap();

        assert_eq!(results.len(), 5);

        // Each result should have unique values
        let mut ids = std::collections::HashSet::new();
        for result in results {
            assert!(result["id"].is_string());
            assert!(result["timestamp"].is_number());
            ids.insert(result["id"].as_str().unwrap().to_string());
        }
        assert_eq!(ids.len(), 5); // All IDs should be unique
    }

    #[test]
    fn test_complex_schema() {
        let config = json!({
            "metadata": {
                "name": "User Profile Generator",
                "version": "1.0.0"
            },
            "variables": {
                "userId": {"fake": ["uuid"]},
                "createdAt": {"fake": ["u64", 1000000000, 1700000000]}
            },
            "schema": {
                "id": {"var": "userId"},
                "profile": {
                    "firstName": {"fake": ["first_name"]},
                    "lastName": {"fake": ["last_name"]},
                    "email": {"fake": ["email"]},
                    "age": {"fake": ["u8", 18, 65]}
                },
                "address": {
                    "street": {"fake": ["street_address"]},
                    "city": {"fake": ["city_name"]},
                    "zipCode": {"fake": ["zip_code"]}
                },
                "metadata": {
                    "createdAt": {"var": "createdAt"},
                    "updatedAt": {"fake": ["u64", 1700000000, 1800000000]}
                }
            }
        });

        let generator = DataGenerator::from_value(config).unwrap();
        let result = generator.generate().unwrap();

        // Verify structure
        assert!(result["id"].is_string());
        assert!(result["profile"]["firstName"].is_string());
        assert!(result["profile"]["email"].as_str().unwrap().contains('@'));
        assert!(result["address"]["street"].is_string());
        assert!(result["metadata"]["createdAt"].is_number());

        // Verify that variable references work
        assert_eq!(result["id"], result["id"]);
        assert_eq!(
            result["metadata"]["createdAt"],
            result["metadata"]["createdAt"]
        );
    }

    #[test]
    fn test_concurrent_generation() {
        use std::sync::Arc;
        use std::thread;

        let config_json = r#"{
            "schema": {
                "id": {"fake": ["uuid"]},
                "name": {"fake": ["name"]},
                "email": {"fake": ["email"]},
                "age": {"fake": ["u8", 18, 65]}
            }
        }"#;

        let generator = Arc::new(DataGenerator::from_json(config_json).unwrap());
        let num_threads = 4;
        let generations_per_thread = 100;

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let data_gen = Arc::clone(&generator);
                thread::spawn(move || {
                    let mut results = Vec::with_capacity(generations_per_thread);
                    for _ in 0..generations_per_thread {
                        let result = data_gen.generate().expect("Generation should succeed");
                        assert!(result["id"].is_string());
                        assert!(result["name"].is_string());
                        assert!(result["email"].as_str().unwrap().contains('@'));
                        results.push(result["id"].as_str().unwrap().to_string());
                    }
                    results
                })
            })
            .collect();

        // Collect all generated IDs
        let mut all_ids: Vec<String> = Vec::new();
        for handle in handles {
            let ids = handle.join().expect("Thread should complete successfully");
            all_ids.extend(ids);
        }

        // Verify we got the expected number of results
        assert_eq!(all_ids.len(), num_threads * generations_per_thread);

        // Verify all IDs are unique (UUIDs should not collide)
        let unique_ids: std::collections::HashSet<_> = all_ids.iter().collect();
        assert_eq!(
            unique_ids.len(),
            all_ids.len(),
            "All generated UUIDs should be unique"
        );
    }

    #[test]
    fn test_concurrent_batch_generation() {
        use std::sync::Arc;
        use std::thread;

        let config_json = r#"{
            "variables": {
                "baseId": {"fake": ["uuid"]}
            },
            "schema": {
                "id": {"var": "baseId"},
                "timestamp": {"fake": ["u64"]}
            }
        }"#;

        let generator = Arc::new(DataGenerator::from_json(config_json).unwrap());
        let num_threads = 4;
        let batch_size = 50;

        let handles: Vec<_> = (0..num_threads)
            .map(|_| {
                let data_gen = Arc::clone(&generator);
                thread::spawn(move || {
                    data_gen
                        .generate_batch(batch_size)
                        .expect("Batch generation should succeed")
                })
            })
            .collect();

        let mut total_count = 0;
        for handle in handles {
            let batch = handle.join().expect("Thread should complete successfully");
            assert_eq!(batch.len(), batch_size);
            total_count += batch.len();
        }

        assert_eq!(total_count, num_threads * batch_size);
    }
}
