use crate::config::ConfigParser;
use crate::engine::Engine;
use crate::error::Result;
use crate::types::{DataFakeConfig, GenerationContext};
use serde_json::Value;

pub struct DataGenerator {
    config: DataFakeConfig,
}

impl DataGenerator {
    pub fn new(config: DataFakeConfig) -> Self {
        Self { config }
    }

    pub fn from_json(json_str: &str) -> Result<Self> {
        let config = ConfigParser::parse(json_str)?;
        Ok(Self::new(config))
    }

    pub fn from_value(json_value: Value) -> Result<Self> {
        let config = ConfigParser::parse_value(json_value)?;
        Ok(Self::new(config))
    }

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

    pub fn generate_batch(&self, count: usize) -> Result<Vec<Value>> {
        let mut results = Vec::with_capacity(count);

        for _ in 0..count {
            results.push(self.generate()?);
        }

        Ok(results)
    }

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
}
