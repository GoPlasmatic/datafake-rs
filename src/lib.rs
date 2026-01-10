//! # datafake-rs
//!
//! A high-performance Rust library for generating realistic mock JSON data using JSONLogic-based configuration.
//!
//! ## Features
//!
//! - Generate fake data using a declarative JSON configuration
//! - Support for 50+ fake data types (names, emails, addresses, etc.)
//! - JSONLogic integration for conditional and dynamic data generation
//! - Variable system for reusing generated values across the schema
//! - Batch generation for creating multiple records efficiently
//!
//! ## Quick Start
//!
//! ```rust
//! use datafake_rs::DataGenerator;
//!
//! let config = r#"{
//!     "schema": {
//!         "id": {"fake": ["uuid"]},
//!         "name": {"fake": ["name"]},
//!         "email": {"fake": ["email"]}
//!     }
//! }"#;
//!
//! let generator = DataGenerator::from_json(config).unwrap();
//! let data = generator.generate().unwrap();
//! println!("{}", data);
//! ```
//!
//! ## Configuration Structure
//!
//! The configuration JSON has three main sections:
//! - `metadata`: Optional metadata about the configuration (name, version, description)
//! - `variables`: Pre-generated values that can be referenced in the schema
//! - `schema`: The structure of the output JSON with fake data operators
//!
//! ## Supported Fake Data Types
//!
//! - **Identity**: `uuid`, `name`, `first_name`, `last_name`, `email`, `username`
//! - **Address**: `street_address`, `city`, `country_code`, `zip_code`, `latitude`, `longitude`
//! - **Company**: `company_name`, `industry`, `profession`, `catch_phrase`
//! - **Finance**: `bic`, `iban`, `credit_card_number`, `currency_code`
//! - **Numeric**: `u8`, `u16`, `u32`, `u64`, `i8`, `i16`, `i32`, `i64`, `f32`, `f64` (with optional ranges)
//! - **Text**: `word`, `sentence`, `paragraph`
//! - **Date/Time**: `datetime`, `date`, `time`

pub mod config;
pub mod engine;
pub mod error;
pub mod generator;
pub mod operators;
pub mod types;

pub use config::ConfigParser;
pub use error::{DataFakeError, Result};
pub use generator::DataGenerator;
pub use types::{DataFakeConfig, GenerationContext, Metadata};
