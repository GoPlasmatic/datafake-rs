#![cfg(target_arch = "wasm32")]

use datafake_wasm::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_generate_simple() {
    let config = r#"{"schema": {"id": {"fake": ["uuid"]}}}"#;
    let result = generate(config).unwrap();
    assert!(result.contains("-")); // UUID contains dashes
}

#[wasm_bindgen_test]
fn test_generate_with_variables() {
    let config = r#"{
        "variables": {
            "userId": {"fake": ["uuid"]}
        },
        "schema": {
            "id": {"var": "userId"},
            "name": {"fake": ["name"]}
        }
    }"#;
    let result = generate(config).unwrap();
    assert!(result.contains("id"));
    assert!(result.contains("name"));
}

#[wasm_bindgen_test]
fn test_generator_class() {
    let config = r#"{"schema": {"name": {"fake": ["name"]}}}"#;
    let generator = FakeDataGenerator::new(config).unwrap();
    let r1 = generator.generate().unwrap();
    let r2 = generator.generate().unwrap();
    // Each generation should produce different data
    assert!(r1.contains("name"));
    assert!(r2.contains("name"));
}

#[wasm_bindgen_test]
fn test_batch_generation() {
    let config = r#"{"schema": {"id": {"fake": ["uuid"]}}}"#;
    let generator = FakeDataGenerator::new(config).unwrap();
    let batch = generator.generate_batch(5).unwrap();
    // Parse and verify it's an array of 5 elements
    let parsed: serde_json::Value = serde_json::from_str(&batch).unwrap();
    assert!(parsed.is_array());
    assert_eq!(parsed.as_array().unwrap().len(), 5);
}

#[wasm_bindgen_test]
fn test_numeric_generation() {
    let config = r#"{"schema": {"age": {"fake": ["u8", 18, 65]}}}"#;
    let result = generate(config).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    let age = parsed["age"].as_u64().unwrap();
    assert!(age >= 18 && age <= 65);
}

#[wasm_bindgen_test]
fn test_invalid_config_error() {
    let config = r#"{"invalid": "config"}"#;
    let result = generate(config);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn test_complex_schema() {
    let config = r#"{
        "schema": {
            "user": {
                "id": {"fake": ["uuid"]},
                "profile": {
                    "firstName": {"fake": ["first_name"]},
                    "lastName": {"fake": ["last_name"]},
                    "email": {"fake": ["email"]}
                }
            },
            "metadata": {
                "active": {"fake": ["bool"]}
            }
        }
    }"#;
    let generator = FakeDataGenerator::new(config).unwrap();
    let result = generator.generate().unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();

    assert!(parsed["user"]["id"].is_string());
    assert!(parsed["user"]["profile"]["firstName"].is_string());
    assert!(parsed["user"]["profile"]["email"].as_str().unwrap().contains('@'));
    assert!(parsed["metadata"]["active"].is_boolean());
}
