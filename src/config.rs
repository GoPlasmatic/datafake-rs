use crate::error::{DataFakeError, Result};
use crate::types::{DataFakeConfig, GenerationContext};
use serde_json::Value;
use std::collections::HashMap;

pub struct ConfigParser;

impl ConfigParser {
    pub fn parse(json_str: &str) -> Result<DataFakeConfig> {
        let config: DataFakeConfig = serde_json::from_str(json_str)
            .map_err(|e| DataFakeError::ConfigParse(format!("Failed to parse JSON: {e}")))?;

        Self::validate_config(&config)?;
        Ok(config)
    }

    pub fn parse_value(json_value: Value) -> Result<DataFakeConfig> {
        let config: DataFakeConfig = serde_json::from_value(json_value)
            .map_err(|e| DataFakeError::ConfigParse(format!("Failed to parse JSON value: {e}")))?;

        Self::validate_config(&config)?;
        Ok(config)
    }

    fn validate_config(config: &DataFakeConfig) -> Result<()> {
        if config.schema.is_null() {
            return Err(DataFakeError::InvalidConfig(
                "Schema cannot be null".to_string(),
            ));
        }

        Self::validate_variables(&config.variables)?;
        Self::validate_schema(&config.schema)?;

        Ok(())
    }

    fn validate_variables(variables: &HashMap<String, Value>) -> Result<()> {
        for (name, value) in variables {
            if name.is_empty() {
                return Err(DataFakeError::InvalidConfig(
                    "Variable name cannot be empty".to_string(),
                ));
            }

            if value.is_null() {
                return Err(DataFakeError::InvalidConfig(format!(
                    "Variable '{name}' cannot be null"
                )));
            }

            Self::validate_jsonlogic_expression(value)?;
        }
        Ok(())
    }

    fn validate_schema(schema: &Value) -> Result<()> {
        match schema {
            Value::Object(map) => {
                // Check if this is a JSONLogic expression
                if map.contains_key("fake") || map.contains_key("var") {
                    Self::validate_jsonlogic_expression(schema)?;
                } else {
                    // Regular object, validate each property
                    for (key, value) in map {
                        if key.is_empty() {
                            return Err(DataFakeError::InvalidConfig(
                                "Schema key cannot be empty".to_string(),
                            ));
                        }
                        Self::validate_schema(value)?;
                    }
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    Self::validate_schema(item)?;
                }
            }
            Value::Null => {
                return Err(DataFakeError::InvalidConfig(
                    "Schema values cannot be null".to_string(),
                ));
            }
            _ => {}
        }
        Ok(())
    }

    fn validate_jsonlogic_expression(value: &Value) -> Result<()> {
        if let Value::Object(map) = value {
            if map.contains_key("fake") {
                Self::validate_fake_operator(map.get("fake").unwrap())?;
            } else if map.contains_key("var")
                && let Some(Value::String(var_name)) = map.get("var")
                && var_name.is_empty()
            {
                return Err(DataFakeError::InvalidConfig(
                    "Variable reference cannot be empty".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn validate_fake_operator(args: &Value) -> Result<()> {
        match args {
            Value::Array(arr) => {
                if arr.is_empty() {
                    return Err(DataFakeError::InvalidConfig(
                        "Fake operator requires at least one argument".to_string(),
                    ));
                }

                if let Some(Value::String(method)) = arr.first() {
                    if method.is_empty() {
                        return Err(DataFakeError::InvalidConfig(
                            "Fake method name cannot be empty".to_string(),
                        ));
                    }

                    match method.as_str() {
                        "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32"
                        | "f64" => {
                            if arr.len() == 3 {
                                let min = Self::extract_number(arr.get(1))?;
                                let max = Self::extract_number(arr.get(2))?;
                                if min > max {
                                    return Err(DataFakeError::InvalidRange { min, max });
                                }
                            } else if arr.len() != 1 {
                                return Err(DataFakeError::InvalidConfig(format!(
                                    "Numeric type '{method}' requires either 0 or 2 arguments (min, max)"
                                )));
                            }
                        }
                        _ => {}
                    }
                } else {
                    return Err(DataFakeError::InvalidConfig(
                        "First argument of fake operator must be a string".to_string(),
                    ));
                }
            }
            _ => {
                return Err(DataFakeError::InvalidConfig(
                    "Fake operator arguments must be an array".to_string(),
                ));
            }
        }
        Ok(())
    }

    fn extract_number(value: Option<&Value>) -> Result<f64> {
        match value {
            Some(Value::Number(n)) => n
                .as_f64()
                .ok_or_else(|| DataFakeError::TypeConversion("Invalid number format".to_string())),
            _ => Err(DataFakeError::TypeConversion(
                "Expected a number".to_string(),
            )),
        }
    }

    pub fn create_context(config: &DataFakeConfig) -> GenerationContext {
        GenerationContext::with_variables(config.variables.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_config() {
        let config_json = r#"{
            "metadata": {
                "name": "Test Config",
                "version": "1.0.0"
            },
            "variables": {
                "userId": {"fake": ["uuid"]}
            },
            "schema": {
                "id": {"var": "userId"},
                "name": {"fake": ["name", "en_US"]}
            }
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.metadata.is_some());
        assert_eq!(config.variables.len(), 1);
    }

    #[test]
    fn test_parse_minimal_config() {
        let config_json = r#"{
            "schema": {
                "name": {"fake": ["name"]}
            }
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_empty_schema() {
        let config_json = r#"{
            "schema": null
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_fake_operator_no_args() {
        let config_json = r#"{
            "schema": {
                "field": {"fake": []}
            }
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_numeric_range() {
        let config_json = r#"{
            "schema": {
                "age": {"fake": ["u8", 100, 0]}
            }
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_numeric_range() {
        let config_json = r#"{
            "schema": {
                "age": {"fake": ["u8", 0, 100]}
            }
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_variable_name() {
        let config_json = r#"{
            "variables": {
                "": {"fake": ["uuid"]}
            },
            "schema": {}
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_nested_schema() {
        let config_json = r#"{
            "variables": {
                "country": {"fake": ["country_code"]}
            },
            "schema": {
                "users": [
                    {
                        "id": {"fake": ["uuid"]},
                        "profile": {
                            "name": {"fake": ["name", "en_US"]},
                            "address": {
                                "street": {"fake": ["street_address"]},
                                "country": {"var": "country"}
                            }
                        }
                    }
                ]
            }
        }"#;

        let result = ConfigParser::parse(config_json);
        assert!(result.is_ok());
    }
}
