//! Core type definitions for datafake-rs configuration and context.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// The main configuration structure for fake data generation.
///
/// This struct represents the complete configuration parsed from JSON,
/// containing optional metadata, pre-defined variables, and the schema
/// that defines the structure of generated data.
///
/// # Example
///
/// ```json
/// {
///     "metadata": { "name": "User Generator", "version": "1.0" },
///     "variables": { "userId": {"fake": ["uuid"]} },
///     "schema": { "id": {"var": "userId"}, "name": {"fake": ["name"]} }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFakeConfig {
    /// Optional metadata about this configuration (name, version, description).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Pre-defined variables that are evaluated once and can be referenced in the schema.
    /// Use `{"var": "variableName"}` in the schema to reference these values.
    #[serde(default)]
    pub variables: HashMap<String, Value>,

    /// The schema defining the structure and fake data types for the output.
    pub schema: Value,
}

/// Optional metadata about the configuration.
///
/// This can include a name, version, description, and any additional
/// custom fields which are captured in the `extra` field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// Human-readable name for this configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Version string for this configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Description of what this configuration generates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Additional custom metadata fields.
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Context for data generation, holding resolved variable values.
///
/// The context is created for each generation run and contains
/// the resolved values of all variables defined in the configuration.
/// These can be referenced using `{"var": "variableName"}` in the schema.
#[derive(Debug, Clone)]
pub struct GenerationContext {
    /// Map of variable names to their resolved JSON values.
    pub variables: HashMap<String, Value>,
}

impl GenerationContext {
    /// Creates a new empty generation context.
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Creates a new context with pre-populated variables.
    pub fn with_variables(variables: HashMap<String, Value>) -> Self {
        Self { variables }
    }

    /// Retrieves a variable value by name.
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }

    /// Sets or updates a variable value.
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }
}

impl Default for GenerationContext {
    fn default() -> Self {
        Self::new()
    }
}
