use crate::error::{DataFakeError, Result};
use datalogic_rs::DataValue;
use fake::faker::address::en::{
    CityName, CountryCode, CountryName, Latitude, Longitude, PostCode, StateAbbr, StateName,
    StreetName, StreetSuffix, ZipCode,
};
use fake::faker::barcode::en::{Isbn10, Isbn13};
use fake::faker::company::en::{
    Bs, BsAdj, BsNoun, BsVerb, CatchPhrase, CompanyName, CompanySuffix, Industry, Profession,
};
use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::currency::en::{CurrencyCode, CurrencyName, CurrencySymbol};
use fake::faker::filesystem::en::{DirPath, FileExtension, FileName, FilePath};
use fake::faker::finance::en::Bic;
use fake::faker::internet::en::{
    DomainSuffix, FreeEmail, IPv4, IPv6, MACAddress, Password, SafeEmail, UserAgent, Username,
};
use fake::faker::lorem::en::{Paragraph, Sentence, Word, Words};
use fake::faker::name::en::{FirstName, LastName, NameWithTitle, Suffix, Title};
use fake::faker::phone_number::en::{CellNumber, PhoneNumber};
use fake::{Fake, Faker};
use rand::Rng;
use serde_json::Value;

pub struct FakeOperator;

impl FakeOperator {
    pub fn generate(args: &[Value]) -> Result<Value> {
        if args.is_empty() {
            return Err(DataFakeError::FakeOperatorError(
                "Fake operator requires at least one argument".to_string(),
            ));
        }

        let method = args[0].as_str().ok_or_else(|| {
            DataFakeError::FakeOperatorError("First argument must be a string".to_string())
        })?;

        let _locale = args.get(1).and_then(|v| v.as_str()).unwrap_or("en");

        match method {
            // Numeric types with optional range
            "u8" => Self::generate_u8(args),
            "u16" => Self::generate_u16(args),
            "u32" => Self::generate_u32(args),
            "u64" => Self::generate_u64(args),
            "i8" => Self::generate_i8(args),
            "i16" => Self::generate_i16(args),
            "i32" => Self::generate_i32(args),
            "i64" => Self::generate_i64(args),
            "f32" => Self::generate_f32(args),
            "f64" => Self::generate_f64(args),

            // Boolean
            "bool" | "boolean" => Ok(Value::Bool(rand::thread_rng().gen())),

            // UUID
            "uuid" => Ok(Value::String(fake::uuid::UUIDv4.fake())),

            // Address related
            "street_address" => {
                // Generate a street address by combining components
                let street_num: u16 = (1..9999).fake();
                let street = StreetName().fake::<String>();
                let suffix = StreetSuffix().fake::<String>();
                Ok(Value::String(format!("{street_num} {street} {suffix}")))
            }
            "city" | "city_name" => Ok(Value::String(CityName().fake())),
            "country_name" => Ok(Value::String(CountryName().fake())),
            "country_code" => Ok(Value::String(CountryCode().fake())),
            "state_name" => Ok(Value::String(StateName().fake())),
            "state_abbr" => Ok(Value::String(StateAbbr().fake())),
            "zip_code" | "zip" => Ok(Value::String(ZipCode().fake())),
            "post_code" => Ok(Value::String(PostCode().fake())),
            "latitude" => Ok(Value::Number(
                serde_json::Number::from_f64(Latitude().fake::<f64>()).unwrap(),
            )),
            "longitude" => Ok(Value::Number(
                serde_json::Number::from_f64(Longitude().fake::<f64>()).unwrap(),
            )),
            "street_name" => Ok(Value::String(StreetName().fake())),
            "street_suffix" => Ok(Value::String(StreetSuffix().fake())),

            // Name related
            "name" => {
                // For now, use English locale only
                use fake::faker::name::en::Name;
                Ok(Value::String(Name().fake()))
            }
            "first_name" => Ok(Value::String(FirstName().fake())),
            "last_name" => Ok(Value::String(LastName().fake())),
            "name_with_title" => Ok(Value::String(NameWithTitle().fake())),
            "title" => Ok(Value::String(Title().fake())),
            "suffix" => Ok(Value::String(Suffix().fake())),

            // Company related
            "company_name" => Ok(Value::String(CompanyName().fake())),
            "company_suffix" => Ok(Value::String(CompanySuffix().fake())),
            "industry" => Ok(Value::String(Industry().fake())),
            "profession" => Ok(Value::String(Profession().fake())),
            "catch_phrase" => Ok(Value::String(CatchPhrase().fake())),
            "bs" => Ok(Value::String(Bs().fake())),
            "bs_adj" => Ok(Value::String(BsAdj().fake())),
            "bs_noun" => Ok(Value::String(BsNoun().fake())),
            "bs_verb" => Ok(Value::String(BsVerb().fake())),

            // Internet related
            "email" | "safe_email" => Ok(Value::String(SafeEmail().fake())),
            "free_email" => Ok(Value::String(FreeEmail().fake())),
            "username" => Ok(Value::String(Username().fake())),
            "password" => {
                let min_len = args.get(1).and_then(|v| v.as_u64()).unwrap_or(8) as usize;
                let max_len = args.get(2).and_then(|v| v.as_u64()).unwrap_or(20) as usize;
                Ok(Value::String(Password(min_len..max_len).fake()))
            }
            "domain_suffix" => Ok(Value::String(DomainSuffix().fake())),
            "ipv4" => Ok(Value::String(IPv4().fake())),
            "ipv6" => Ok(Value::String(IPv6().fake())),
            "mac_address" => Ok(Value::String(MACAddress().fake())),
            "user_agent" => Ok(Value::String(UserAgent().fake())),

            // Phone
            "phone_number" => Ok(Value::String(PhoneNumber().fake())),
            "cell_number" => Ok(Value::String(CellNumber().fake())),

            // Finance
            "bic" => Ok(Value::String(Bic().fake())),
            "credit_card_number" => Ok(Value::String(CreditCardNumber().fake())),

            // Currency
            "currency_code" => Ok(Value::String(CurrencyCode().fake())),
            "currency_name" => Ok(Value::String(CurrencyName().fake())),
            "currency_symbol" => Ok(Value::String(CurrencySymbol().fake())),

            // Lorem
            "word" => Ok(Value::String(Word().fake())),
            "words" => {
                let count = args.get(1).and_then(|v| v.as_u64()).unwrap_or(5) as usize;
                let words: Vec<String> = Words(count..count + 1).fake();
                Ok(Value::String(words.join(" ")))
            }
            "sentence" => {
                let min_words = args.get(1).and_then(|v| v.as_u64()).unwrap_or(4) as usize;
                let max_words = args.get(2).and_then(|v| v.as_u64()).unwrap_or(10) as usize;
                Ok(Value::String(Sentence(min_words..max_words).fake()))
            }
            "paragraph" => {
                let min_sentences = args.get(1).and_then(|v| v.as_u64()).unwrap_or(3) as usize;
                let max_sentences = args.get(2).and_then(|v| v.as_u64()).unwrap_or(7) as usize;
                Ok(Value::String(
                    Paragraph(min_sentences..max_sentences).fake(),
                ))
            }

            // Barcode
            "isbn10" => Ok(Value::String(Isbn10().fake())),
            "isbn13" => Ok(Value::String(Isbn13().fake())),

            // Filesystem
            "file_name" => Ok(Value::String(FileName().fake())),
            "file_extension" => Ok(Value::String(FileExtension().fake())),
            "dir_path" => Ok(Value::String(DirPath().fake())),
            "file_path" => Ok(Value::String(FilePath().fake())),

            _ => Err(DataFakeError::FakeOperatorError(format!(
                "Unknown fake method: {method}"
            ))),
        }
    }

    fn generate_u8(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<u8>()))),
            3 => {
                let min = args[1].as_u64().unwrap_or(0) as u8;
                let max = args[2].as_u64().unwrap_or(255) as u8;
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "u8 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_u16(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<u16>()))),
            3 => {
                let min = args[1].as_u64().unwrap_or(0) as u16;
                let max = args[2].as_u64().unwrap_or(65535) as u16;
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "u16 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_u32(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<u32>()))),
            3 => {
                let min = args[1].as_u64().unwrap_or(0) as u32;
                let max = args[2].as_u64().unwrap_or(u32::MAX as u64) as u32;
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "u32 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_u64(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<u64>()))),
            3 => {
                let min = args[1].as_u64().unwrap_or(0);
                let max = args[2].as_u64().unwrap_or(u64::MAX);
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "u64 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_i8(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<i8>()))),
            3 => {
                let min = args[1].as_i64().unwrap_or(i8::MIN as i64) as i8;
                let max = args[2].as_i64().unwrap_or(i8::MAX as i64) as i8;
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "i8 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_i16(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<i16>()))),
            3 => {
                let min = args[1].as_i64().unwrap_or(i16::MIN as i64) as i16;
                let max = args[2].as_i64().unwrap_or(i16::MAX as i64) as i16;
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "i16 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_i32(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<i32>()))),
            3 => {
                let min = args[1].as_i64().unwrap_or(i32::MIN as i64) as i32;
                let max = args[2].as_i64().unwrap_or(i32::MAX as i64) as i32;
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "i32 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_i64(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(serde_json::Number::from(Faker.fake::<i64>()))),
            3 => {
                let min = args[1].as_i64().unwrap_or(i64::MIN);
                let max = args[2].as_i64().unwrap_or(i64::MAX);
                Ok(Value::Number(serde_json::Number::from(
                    rand::thread_rng().gen_range(min..=max),
                )))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "i64 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_f32(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(
                serde_json::Number::from_f64(Faker.fake::<f32>() as f64).unwrap(),
            )),
            3 => {
                let min = args[1].as_f64().unwrap_or(0.0) as f32;
                let max = args[2].as_f64().unwrap_or(1.0) as f32;
                let value = rand::thread_rng().gen_range(min..=max);
                Ok(Value::Number(
                    serde_json::Number::from_f64(value as f64).unwrap(),
                ))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "f32 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }

    fn generate_f64(args: &[Value]) -> Result<Value> {
        match args.len() {
            1 => Ok(Value::Number(
                serde_json::Number::from_f64(Faker.fake::<f64>()).unwrap(),
            )),
            3 => {
                let min = args[1].as_f64().unwrap_or(0.0);
                let max = args[2].as_f64().unwrap_or(1.0);
                let value = rand::thread_rng().gen_range(min..=max);
                Ok(Value::Number(serde_json::Number::from_f64(value).unwrap()))
            }
            _ => Err(DataFakeError::FakeOperatorError(
                "f64 requires either 1 or 3 arguments".to_string(),
            )),
        }
    }
}

/// Handler function for the fake operator that integrates with datalogic-rs
pub fn fake_operator_handler<'r>(
    args: Vec<DataValue<'r>>,
    _data: DataValue<'r>,
) -> std::result::Result<DataValue<'r>, String> {
    // Convert DataValue args to serde_json::Value for compatibility with existing FakeOperator
    let mut json_args = Vec::new();

    for arg in args {
        let json_value = if let Some(s) = arg.as_str() {
            Value::String(s.to_string())
        } else if let Some(n) = arg.as_i64() {
            Value::Number(serde_json::Number::from(n))
        } else if let Some(n) = arg.as_f64() {
            Value::Number(serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0)))
        } else if let Some(b) = arg.as_bool() {
            Value::Bool(b)
        } else if let Some(arr) = arg.as_array() {
            // Handle array arguments - recursively convert each element
            let mut arr_values = Vec::new();
            for item in arr {
                if let Some(s) = item.as_str() {
                    arr_values.push(Value::String(s.to_string()));
                } else if let Some(n) = item.as_i64() {
                    arr_values.push(Value::Number(serde_json::Number::from(n)));
                } else if let Some(n) = item.as_f64() {
                    arr_values.push(Value::Number(
                        serde_json::Number::from_f64(n).unwrap_or(serde_json::Number::from(0)),
                    ));
                } else if let Some(b) = item.as_bool() {
                    arr_values.push(Value::Bool(b));
                } else {
                    return Err("Unsupported array element type in fake operator".to_string());
                }
            }
            // If we have an array as the first argument, use its elements as the args
            if json_args.is_empty() {
                json_args = arr_values;
                continue;
            }
            Value::Array(arr_values)
        } else {
            return Err(format!(
                "Unsupported argument type for fake operator: {arg:?}"
            ));
        };
        json_args.push(json_value);
    }

    // Call the existing FakeOperator
    match FakeOperator::generate(&json_args) {
        Ok(value) => {
            // Convert the result back to DataValue
            match value {
                Value::String(s) => {
                    let leaked_str = Box::leak(s.into_boxed_str());
                    Ok(DataValue::String(leaked_str))
                }
                Value::Number(n) => {
                    if let Some(i) = n.as_i64() {
                        Ok(DataValue::Number(
                            datalogic_rs::value::NumberValue::from_i64(i),
                        ))
                    } else if let Some(f) = n.as_f64() {
                        Ok(DataValue::Number(
                            datalogic_rs::value::NumberValue::from_f64(f),
                        ))
                    } else {
                        Err("Invalid number format".to_string())
                    }
                }
                Value::Bool(b) => Ok(DataValue::Bool(b)),
                _ => Err("Unsupported return type from fake operator".to_string()),
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_uuid() {
        let args = vec![json!("uuid")];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        assert_eq!(result.as_str().unwrap().len(), 36); // UUID v4 format
    }

    #[test]
    fn test_generate_numeric_no_range() {
        let args = vec![json!("u8")];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_number());
    }

    #[test]
    fn test_generate_numeric_with_range() {
        let args = vec![json!("u8"), json!(10), json!(20)];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_number());
        let value = result.as_u64().unwrap();
        assert!((10..=20).contains(&value));
    }

    #[test]
    fn test_generate_name_with_locale() {
        let args = vec![json!("name"), json!("en_US")];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        assert!(!result.as_str().unwrap().is_empty());
    }

    #[test]
    fn test_generate_email() {
        let args = vec![json!("email")];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        let email = result.as_str().unwrap();
        assert!(email.contains('@'));
    }

    #[test]
    fn test_generate_password_with_length() {
        let args = vec![json!("password"), json!(10), json!(15)];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        let password = result.as_str().unwrap();
        assert!(password.len() >= 10 && password.len() <= 15);
    }

    #[test]
    fn test_invalid_method() {
        let args = vec![json!("invalid_method")];
        let result = FakeOperator::generate(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_args() {
        let args = vec![];
        let result = FakeOperator::generate(&args);
        assert!(result.is_err());
    }
}
