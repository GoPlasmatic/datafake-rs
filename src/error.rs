use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataFakeError {
    #[error("Configuration parsing error: {0}")]
    ConfigParse(String),

    #[error("Invalid configuration structure: {0}")]
    InvalidConfig(String),

    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Fake operator error: {0}")]
    FakeOperatorError(String),

    #[error("Type conversion error: {0}")]
    TypeConversion(String),

    #[error("Invalid locale: {0}")]
    InvalidLocale(String),

    #[error("Invalid numeric range: min={min}, max={max}")]
    InvalidRange { min: f64, max: f64 },
}

pub type Result<T> = std::result::Result<T, DataFakeError>;
