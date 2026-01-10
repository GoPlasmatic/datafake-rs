//! Error types for datafake-rs operations.
//!
//! This module defines [`DataFakeError`], which covers all error cases
//! that can occur during configuration parsing and data generation.

use thiserror::Error;

/// All possible errors that can occur in datafake-rs operations.
#[derive(Error, Debug)]
pub enum DataFakeError {
    /// Failed to parse the JSON configuration.
    #[error("Configuration parsing error: {0}")]
    ConfigParse(String),

    /// The configuration structure is invalid (e.g., null schema, empty variable names).
    #[error("Invalid configuration structure: {0}")]
    InvalidConfig(String),

    /// A referenced variable was not found in the context.
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    /// JSON serialization/deserialization failed.
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// An error occurred in the fake data operator.
    #[error("Fake operator error: {0}")]
    FakeOperatorError(String),

    /// Failed to convert between types.
    #[error("Type conversion error: {0}")]
    TypeConversion(String),

    /// The specified locale is not supported.
    #[error("Invalid locale: {0}")]
    InvalidLocale(String),

    /// Numeric range is invalid (min > max).
    #[error("Invalid numeric range: min={min}, max={max}")]
    InvalidRange {
        /// The minimum value specified.
        min: f64,
        /// The maximum value specified.
        max: f64,
    },
}

/// A specialized Result type for datafake-rs operations.
pub type Result<T> = std::result::Result<T, DataFakeError>;
