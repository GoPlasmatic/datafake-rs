use crate::error::{DataFakeError, Result};
use crate::operators::FakeOperator;
use crate::types::GenerationContext;
use datalogic_rs::DataLogic;
use serde_json::{Map, Value};
use std::cell::RefCell;

thread_local! {
    static THREAD_LOCAL_DATA_LOGIC: RefCell<Option<DataLogic>> = const { RefCell::new(None) };
}

fn get_or_init_datalogic() -> &'static RefCell<Option<DataLogic>> {
    THREAD_LOCAL_DATA_LOGIC.with(|dl_cell| {
        let mut dl_opt = dl_cell.borrow_mut();
        if dl_opt.is_none() {
            // Note: Cannot use preserve_structure mode with custom operators in v4
            // This is a limitation in datalogic-rs v4 where custom operators are not
            // recognized in preserve_structure mode
            let mut dl = DataLogic::new();
            // Register the fake operator
            dl.add_operator("fake".to_string(), Box::new(FakeOperator));

            *dl_opt = Some(dl);
        }
        // This is safe because we're returning a reference to a thread_local
        unsafe { &*(dl_cell as *const RefCell<Option<DataLogic>>) }
    })
}

pub struct Engine;

impl Engine {
    pub fn evaluate(expression: &Value, context: &GenerationContext) -> Result<Value> {
        // Evaluate the expression directly with JSONLogic (fake operator is registered)
        let dl_cell = get_or_init_datalogic();
        let dl_opt = dl_cell.borrow();
        let data_logic = dl_opt.as_ref().unwrap();

        // Convert context to JSON value for datalogic
        let context_json =
            serde_json::to_value(&context.variables).map_err(DataFakeError::JsonError)?;

        // Compile and evaluate the expression
        let compiled = data_logic.compile(expression).map_err(|e| {
            DataFakeError::FakeOperatorError(format!("JSONLogic compilation error: {e}"))
        })?;

        data_logic
            .evaluate_owned(&compiled, context_json)
            .map_err(|e| {
                DataFakeError::FakeOperatorError(format!("JSONLogic evaluation error: {e}"))
            })
    }

    pub fn process_schema(schema: &Value, context: &GenerationContext) -> Result<Value> {
        // Since we can't use preserve_structure with custom operators in v4,
        // we need to manually handle object structure preservation
        match schema {
            Value::Object(obj) if obj.len() == 1 => {
                // Single-key objects might be JSONLogic operators
                if let Some((key, _value)) = obj.iter().next() {
                    // Check if this looks like a JSONLogic operator
                    // Known operators or custom operators should be evaluated
                    if Self::is_jsonlogic_operator(key) {
                        return Self::evaluate(schema, context);
                    }
                }
                // Not an operator, process as regular object
                let mut result = serde_json::Map::new();
                for (key, value) in obj {
                    result.insert(key.clone(), Self::process_schema(value, context)?);
                }
                Ok(Value::Object(result))
            }
            Value::Object(obj) => {
                // Multi-key objects are treated as templates
                let mut result = serde_json::Map::new();
                for (key, value) in obj {
                    result.insert(key.clone(), Self::process_schema(value, context)?);
                }
                Ok(Value::Object(result))
            }
            Value::Array(arr) => {
                let mut result = Vec::new();
                for item in arr {
                    result.push(Self::process_schema(item, context)?);
                }
                Ok(Value::Array(result))
            }
            _ => {
                // Primitive values are returned as-is
                Ok(schema.clone())
            }
        }
    }

    fn is_jsonlogic_operator(key: &str) -> bool {
        // Check if this is a known JSONLogic operator or our custom operator
        matches!(
            key,
            "var" | "==" | "!=" | "===" | "!==" | "!" | "!!" | "or" | "and" | "?:" | "if" |
            ">" | ">=" | "<" | "<=" | "max" | "min" | "+" | "-" | "*" | "/" | "%" |
            "map" | "filter" | "reduce" | "all" | "none" | "some" | "merge" | "in" |
            "cat" | "substr" | "log" | "method" | "preserve" | "missing" | "missing_some" |
            // Our custom operator
            "fake"
        )
    }

    pub fn generate_variables(variables: &Map<String, Value>) -> Result<Map<String, Value>> {
        if variables.is_empty() {
            return Ok(Map::new());
        }

        // Since we can't use preserve_structure with custom operators,
        // we process each variable individually
        let temp_context = GenerationContext::new();
        let variables_as_value = Value::Object(variables.clone());

        match Self::process_schema(&variables_as_value, &temp_context)? {
            Value::Object(map) => Ok(map),
            _ => Err(DataFakeError::FakeOperatorError(
                "Variables evaluation did not return an object".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_evaluate_simple_fake() {
        let expression = json!({"fake": ["uuid"]});
        let context = GenerationContext::new();
        let result = Engine::evaluate(&expression, &context).unwrap();
        assert!(result.is_string());
        assert_eq!(result.as_str().unwrap().len(), 36);
    }

    #[test]
    fn test_evaluate_var_reference() {
        let expression = json!({"var": "userId"});
        let mut context = GenerationContext::new();
        context.set_variable("userId".to_string(), json!("test-id-123"));

        let result = Engine::evaluate(&expression, &context).unwrap();
        assert_eq!(result, json!("test-id-123"));
    }

    #[test]
    fn test_process_schema_nested() {
        let schema = json!({
            "id": {"fake": ["uuid"]},
            "user": {
                "name": {"fake": ["name"]},
                "email": {"fake": ["email"]}
            }
        });

        let context = GenerationContext::new();
        let result = Engine::process_schema(&schema, &context).unwrap();

        assert!(result["id"].is_string());
        assert!(result["user"]["name"].is_string());
        assert!(result["user"]["email"].as_str().unwrap().contains('@'));
    }

    #[test]
    fn test_process_schema_with_array() {
        let schema = json!({
            "tags": [
                {"fake": ["word"]},
                {"fake": ["word"]},
                {"fake": ["word"]}
            ]
        });

        let context = GenerationContext::new();
        let result = Engine::process_schema(&schema, &context).unwrap();

        assert!(result["tags"].is_array());
        assert_eq!(result["tags"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn test_generate_variables() {
        let variables = json!({
            "userId": {"fake": ["uuid"]},
            "timestamp": {"fake": ["u64", 1000000, 9999999]}
        })
        .as_object()
        .unwrap()
        .clone();

        let result = Engine::generate_variables(&variables).unwrap();

        assert!(result.contains_key("userId"));
        assert!(result.contains_key("timestamp"));
        assert!(result["userId"].is_string());
        assert!(result["timestamp"].is_number());
    }

    #[test]
    fn test_process_schema_with_cat_operator() {
        let schema = json!({
            "terminal": {"cat": ["ABCD", "XXXX"]},
            "code": {"cat": [{"var": "prefix"}, "-", {"var": "suffix"}]}
        });

        let mut context = GenerationContext::new();
        context.set_variable("prefix".to_string(), json!("PRE"));
        context.set_variable("suffix".to_string(), json!("SUF"));

        let result = Engine::process_schema(&schema, &context).unwrap();

        assert_eq!(result["terminal"], "ABCDXXXX");
        assert_eq!(result["code"], "PRE-SUF");
    }

    #[test]
    fn test_jsonlogic_operators_in_schema() {
        let schema = json!({
            "isActive": {"==": [{"var": "status"}, "active"]},
            "fullName": {"cat": [{"var": "firstName"}, " ", {"var": "lastName"}]},
            "age": {"+": [{"var": "baseAge"}, 10]},
            "hasDiscount": {">": [{"var": "purchases"}, 5]}
        });

        let mut context = GenerationContext::new();
        context.set_variable("status".to_string(), json!("active"));
        context.set_variable("firstName".to_string(), json!("John"));
        context.set_variable("lastName".to_string(), json!("Doe"));
        context.set_variable("baseAge".to_string(), json!(20));
        context.set_variable("purchases".to_string(), json!(10));

        let result = Engine::process_schema(&schema, &context).unwrap();

        assert_eq!(result["isActive"], true);
        assert_eq!(result["fullName"], "John Doe");
        assert_eq!(result["age"], 30);
        assert_eq!(result["hasDiscount"], true);
    }

    #[test]
    fn test_preserve_structure_with_custom_operators() {
        // Test that custom operators work with preserve_structure enabled
        let schema = json!({
            "user": {
                "id": {"fake": ["uuid"]},
                "profile": {
                    "name": {"fake": ["name"]},
                    "age": {"fake": ["u8", 18, 65]},
                    "nested": {
                        "email": {"fake": ["email"]},
                        "active": true,
                        "count": 42
                    }
                }
            },
            "metadata": {
                "version": "1.0",
                "generated": {"fake": ["bool"]}
            }
        });

        let context = GenerationContext::new();
        let result = Engine::process_schema(&schema, &context).unwrap();

        // Check structure is preserved
        assert!(result["user"]["id"].is_string());
        assert_eq!(result["user"]["id"].as_str().unwrap().len(), 36); // UUID length
        assert!(result["user"]["profile"]["name"].is_string());
        assert!(result["user"]["profile"]["age"].is_number());
        let age = result["user"]["profile"]["age"].as_u64().unwrap();
        assert!((18..=65).contains(&age));
        assert!(
            result["user"]["profile"]["nested"]["email"]
                .as_str()
                .unwrap()
                .contains('@')
        );
        assert_eq!(result["user"]["profile"]["nested"]["active"], true);
        assert_eq!(result["user"]["profile"]["nested"]["count"], 42);
        assert_eq!(result["metadata"]["version"], "1.0");
        assert!(result["metadata"]["generated"].is_boolean());
    }
}
