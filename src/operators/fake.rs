use crate::error::{DataFakeError, Result};
use chrono::Utc;
use datalogic_rs::{ContextStack, Evaluator, Operator};
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

impl Operator for FakeOperator {
    fn evaluate(
        &self,
        args: &[Value],
        _context: &mut ContextStack,
        _evaluator: &dyn Evaluator,
    ) -> std::result::Result<Value, datalogic_rs::Error> {
        // Call the existing generate method and convert error
        FakeOperator::generate(args).map_err(|e| datalogic_rs::Error::Custom(e.to_string()))
    }
}

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

        // NOTE: Locale parameter is parsed but not currently used for most operations.
        // The underlying `fake-rs` crate has limited locale support - most generators
        // only support English (en). The locale parameter is accepted for API compatibility
        // and potential future expansion when fake-rs adds broader locale support.
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
            "bool" | "boolean" => Ok(Value::Bool(rand::rng().random())),

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
            "post_code" | "postcode" | "postal_code" => Ok(Value::String(PostCode().fake())),
            "latitude" => Ok(Value::Number(
                serde_json::Number::from_f64(Latitude().fake::<f64>()).unwrap(),
            )),
            "longitude" => Ok(Value::Number(
                serde_json::Number::from_f64(Longitude().fake::<f64>()).unwrap(),
            )),
            "street_name" => Ok(Value::String(StreetName().fake())),
            "street_suffix" => Ok(Value::String(StreetSuffix().fake())),

            // Name related
            "name" | "full_name" => {
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
            "domain_name" => {
                // Generate a domain name
                let words: Vec<String> = Words(1..2).fake();
                let suffix = DomainSuffix().fake::<String>();
                Ok(Value::String(format!(
                    "{}.{}",
                    words.join("").to_lowercase(),
                    suffix
                )))
            }
            "ipv4" => Ok(Value::String(IPv4().fake())),
            "ipv6" => Ok(Value::String(IPv6().fake())),
            "mac_address" => Ok(Value::String(MACAddress().fake())),
            "user_agent" => Ok(Value::String(UserAgent().fake())),

            // Phone
            "phone_number" => Ok(Value::String(PhoneNumber().fake())),
            "cell_number" => Ok(Value::String(CellNumber().fake())),

            // Finance
            "bic" => {
                // Support optional length parameter (8 or 11)
                let length = args.get(1).and_then(|v| v.as_u64());
                match length {
                    Some(8) => Self::generate_bic_fixed(8),
                    Some(11) => Self::generate_bic_fixed(11),
                    Some(_) => Err(DataFakeError::FakeOperatorError(
                        "BIC length must be 8 or 11".to_string(),
                    )),
                    None => Ok(Value::String(Bic().fake())), // Default: random 8 or 11
                }
            }
            "bic8" => Self::generate_bic_fixed(8),
            "bic11" => Self::generate_bic_fixed(11),
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

            // Date/Time - Generate ISO8601 formatted datetime
            "datetime" | "iso8601_datetime" => {
                let now = Utc::now();
                Ok(Value::String(now.to_rfc3339()))
            }
            "date" => {
                // Generate date in specified format
                let format = args.get(1).and_then(|v| v.as_str()).unwrap_or("%Y-%m-%d");
                let now = Utc::now();
                Ok(Value::String(now.format(format).to_string()))
            }
            "time" => {
                // Generate time in HH:MM:SS format
                let mut rng = rand::rng();
                let hour = rng.random_range(0..24);
                let minute = rng.random_range(0..60);
                let second = rng.random_range(0..60);
                Ok(Value::String(format!("{hour:02}:{minute:02}:{second:02}")))
            }
            "month_name" => {
                // Generate a month name
                let months = [
                    "January",
                    "February",
                    "March",
                    "April",
                    "May",
                    "June",
                    "July",
                    "August",
                    "September",
                    "October",
                    "November",
                    "December",
                ];
                let mut rng = rand::rng();
                let idx = rng.random_range(0..months.len());
                Ok(Value::String(months[idx].to_string()))
            }

            // Financial - Custom types for MX messages
            "iban" => {
                let country = args.get(1).and_then(|v| v.as_str()).unwrap_or("DE");
                let mut rng = rand::rng();
                let check = format!("{:02}", rng.random_range(10..99));
                let account: String = (0..18)
                    .map(|_| rng.random_range(0..10).to_string())
                    .collect();
                Ok(Value::String(format!("{country}{check}{account}")))
            }
            "lei" => {
                // Generate a realistic LEI (18 uppercase alphanumeric + 2 check digits)
                let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                let mut rng = rand::rng();
                let lei: String = (0..18)
                    .map(|_| chars.chars().nth(rng.random_range(0..chars.len())).unwrap())
                    .collect();
                let check = format!("{:02}", rng.random_range(10..99));
                Ok(Value::String(format!("{lei}{check}")))
            }
            "alphanumeric" => {
                let min_len = args.get(1).and_then(|v| v.as_u64()).unwrap_or(10) as usize;
                let max_len = args
                    .get(2)
                    .and_then(|v| v.as_u64())
                    .unwrap_or(min_len as u64) as usize;
                let mut rng = rand::rng();
                let len = if min_len == max_len {
                    min_len
                } else {
                    rng.random_range(min_len..=max_len)
                };
                let chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
                let result: String = (0..len)
                    .map(|_| chars.chars().nth(rng.random_range(0..chars.len())).unwrap())
                    .collect();
                Ok(Value::String(result))
            }

            // Enum/Choice - pick one value from provided options
            "enum" | "pick" | "choice" => {
                if args.len() < 2 {
                    return Err(DataFakeError::FakeOperatorError(
                        "enum requires at least one option".to_string(),
                    ));
                }
                let options = &args[1..];
                let mut rng = rand::rng();
                let idx = rng.random_range(0..options.len());
                Ok(options[idx].clone())
            }

            // Regex - Handle simple regex patterns for selecting from options
            //
            // LIMITATION: This implementation only supports simple alternation patterns
            // in the form "(A|B|C)" where options are separated by pipe characters.
            // Complex regex patterns (character classes, quantifiers, anchors, etc.)
            // are NOT supported and will return the placeholder "REGEX_PATTERN".
            //
            // For full regex-based generation, consider using the "enum" or "pick"
            // methods instead, which allow explicit listing of options.
            "regex" => {
                if let Some(Value::String(pattern)) = args.get(1) {
                    // Simple handling for common patterns like "(A|B|C)"
                    if pattern.starts_with('(') && pattern.ends_with(')') {
                        let inner = &pattern[1..pattern.len() - 1];
                        let options: Vec<&str> = inner.split('|').collect();
                        if !options.is_empty() {
                            let mut rng = rand::rng();
                            let idx = rng.random_range(0..options.len());
                            return Ok(Value::String(options[idx].to_string()));
                        }
                    }
                    // For complex patterns, return a placeholder (see LIMITATION above)
                    Ok(Value::String("REGEX_PATTERN".to_string()))
                } else {
                    Err(DataFakeError::FakeOperatorError(
                        "regex requires a pattern argument".to_string(),
                    ))
                }
            }

            _ => Err(DataFakeError::FakeOperatorError(format!(
                "Unknown fake method: {method}"
            ))),
        }
    }
}

/// Macro to generate numeric type handlers, reducing code duplication.
/// Generates functions for integer types (u8, u16, u32, u64, i8, i16, i32, i64).
macro_rules! impl_integer_generator {
    ($fn_name:ident, $type:ty, $extract_fn:ident, $default_min:expr, $default_max:expr) => {
        fn $fn_name(args: &[Value]) -> Result<Value> {
            match args.len() {
                1 => Ok(Value::Number(serde_json::Number::from(
                    Faker.fake::<$type>(),
                ))),
                3 => {
                    let min = args[1].$extract_fn().unwrap_or($default_min as _) as $type;
                    let max = args[2].$extract_fn().unwrap_or($default_max as _) as $type;
                    Ok(Value::Number(serde_json::Number::from(
                        rand::rng().random_range(min..=max),
                    )))
                }
                _ => Err(DataFakeError::FakeOperatorError(
                    concat!(stringify!($type), " requires either 1 or 3 arguments").to_string(),
                )),
            }
        }
    };
}

/// Macro to generate floating-point type handlers.
macro_rules! impl_float_generator {
    ($fn_name:ident, $type:ty) => {
        fn $fn_name(args: &[Value]) -> Result<Value> {
            match args.len() {
                1 => Ok(Value::Number(
                    serde_json::Number::from_f64(Faker.fake::<$type>() as f64)
                        .expect("Generated float should be a valid JSON number"),
                )),
                3 => {
                    let min = args[1].as_f64().unwrap_or(0.0) as $type;
                    let max = args[2].as_f64().unwrap_or(1.0) as $type;
                    let value = rand::rng().random_range(min..=max);
                    Ok(Value::Number(
                        serde_json::Number::from_f64(value as f64)
                            .expect("Generated float should be a valid JSON number"),
                    ))
                }
                _ => Err(DataFakeError::FakeOperatorError(
                    concat!(stringify!($type), " requires either 1 or 3 arguments").to_string(),
                )),
            }
        }
    };
}

impl FakeOperator {
    // Generate integer type handlers using the macro
    impl_integer_generator!(generate_u8, u8, as_u64, 0, u8::MAX);
    impl_integer_generator!(generate_u16, u16, as_u64, 0, u16::MAX);
    impl_integer_generator!(generate_u32, u32, as_u64, 0, u32::MAX);
    impl_integer_generator!(generate_u64, u64, as_u64, 0, u64::MAX);
    impl_integer_generator!(generate_i8, i8, as_i64, i8::MIN, i8::MAX);
    impl_integer_generator!(generate_i16, i16, as_i64, i16::MIN, i16::MAX);
    impl_integer_generator!(generate_i32, i32, as_i64, i32::MIN, i32::MAX);
    impl_integer_generator!(generate_i64, i64, as_i64, i64::MIN, i64::MAX);

    // Generate floating-point type handlers using the macro
    impl_float_generator!(generate_f32, f32);
    impl_float_generator!(generate_f64, f64);

    fn generate_bic_fixed(length: u64) -> Result<Value> {
        use rand::seq::IndexedRandom;
        let mut rng = rand::rng();

        // Constants for BIC generation
        const ALPHABET: &[char; 26] = &[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];
        const VOWELS: &[char; 5] = &['A', 'E', 'I', 'O', 'U'];
        const ISO3166: &[&str] = &[
            "AC", "AD", "AE", "AF", "AG", "AI", "AL", "AM", "AN", "AO", "AQ", "AR", "AS", "AT",
            "AU", "AW", "AX", "AZ", "BA", "BB", "BD", "BE", "BF", "BG", "BH", "BI", "BJ", "BL",
            "BM", "BN", "BO", "BQ", "BR", "BS", "BT", "BU", "BV", "BW", "BY", "BZ", "CA", "CC",
            "CD", "CE", "CF", "CG", "CH", "CI", "CK", "CL", "CM", "CN", "CO", "CP", "CR", "CS",
            "CU", "CV", "CW", "CX", "CY", "CZ", "DD", "DE", "DG", "DJ", "DK", "DM", "DO", "DZ",
            "EA", "EC", "EE", "EG", "EH", "ER", "ES", "ET", "EU", "FI", "FJ", "FK", "FM", "FO",
            "FR", "FX", "GA", "GB", "GD", "GE", "GF", "GG", "GH", "GI", "GL", "GM", "GN", "GP",
            "GQ", "GR", "GS", "GT", "GU", "GW", "GY", "HK", "HM", "HN", "HR", "HT", "HU", "IC",
            "ID", "IE", "IL", "IM", "IN", "IO", "IQ", "IR", "IS", "IT", "JE", "JM", "JO", "JP",
            "KE", "KG", "KH", "KI", "KM", "KN", "KP", "KR", "KW", "KY", "KZ", "LA", "LB", "LC",
            "LI", "LK", "LR", "LS", "LT", "LU", "LV", "LY", "MA", "MC", "MD", "ME", "MF", "MG",
            "MH", "MK", "ML", "MM", "MN", "MO", "MP", "MQ", "MR", "MS", "MT", "MU", "MV", "MW",
            "MX", "MY", "MZ", "NA", "NC", "NE", "NF", "NG", "NI", "NL", "NO", "NP", "NR", "NT",
            "NU", "NZ", "OM", "PA", "PE", "PF", "PG", "PH", "PK", "PL", "PM", "PN", "PR", "PS",
            "PT", "PW", "PY", "QA", "RE", "RO", "RS", "RU", "RW", "SA", "SB", "SC", "SD", "SE",
            "SG", "SH", "SI", "SJ", "SK", "SL", "SM", "SN", "SO", "SR", "SS", "ST", "SU", "SV",
            "SX", "SY", "SZ", "TA", "TC", "TD", "TF", "TG", "TH", "TJ", "TK", "TL", "TM", "TN",
            "TO", "TR", "TT", "TV", "TW", "TZ", "UA", "UG", "UM", "US", "UY", "UZ", "VA", "VC",
            "VE", "VG", "VI", "VN", "VU", "WF", "WS", "YE", "YT", "YU", "ZA", "ZM", "ZR", "ZW",
        ];

        // Generate base 8-character BIC: AAAAVCC1
        let base_bic = format!(
            "{}{}{}{}{}{}1",
            *ALPHABET.choose(&mut rng).unwrap(),
            *ALPHABET.choose(&mut rng).unwrap(),
            *ALPHABET.choose(&mut rng).unwrap(),
            *VOWELS.choose(&mut rng).unwrap(),
            *ISO3166.choose(&mut rng).unwrap(),
            *ALPHABET.choose(&mut rng).unwrap(),
        );

        let bic = if length == 11 {
            // Add branch code suffix (3 chars)
            let prob: i8 = rng.random_range(0..100);
            let suffix = if prob < 70 {
                // 70% chance: numeric branch code
                format!(
                    "{}{}{}",
                    rng.random_range('0'..='9'),
                    rng.random_range('0'..='9'),
                    rng.random_range('0'..='9'),
                )
            } else {
                // 30% chance: alphabetic branch code
                format!(
                    "{}{}{}",
                    *ALPHABET.choose(&mut rng).unwrap(),
                    *VOWELS.choose(&mut rng).unwrap(),
                    *ALPHABET.choose(&mut rng).unwrap(),
                )
            };
            format!("{}{}", base_bic, suffix)
        } else {
            base_bic
        };

        Ok(Value::String(bic))
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

    #[test]
    fn test_generate_bic_default() {
        let args = vec![json!("bic")];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        let bic = result.as_str().unwrap();
        assert!(bic.len() == 8 || bic.len() == 11);
    }

    #[test]
    fn test_generate_bic8() {
        let args = vec![json!("bic8")];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        let bic = result.as_str().unwrap();
        assert_eq!(bic.len(), 8);
    }

    #[test]
    fn test_generate_bic11() {
        let args = vec![json!("bic11")];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        let bic = result.as_str().unwrap();
        assert_eq!(bic.len(), 11);
    }

    #[test]
    fn test_generate_bic_with_length_8() {
        let args = vec![json!("bic"), json!(8)];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        let bic = result.as_str().unwrap();
        assert_eq!(bic.len(), 8);
    }

    #[test]
    fn test_generate_bic_with_length_11() {
        let args = vec![json!("bic"), json!(11)];
        let result = FakeOperator::generate(&args).unwrap();
        assert!(result.is_string());
        let bic = result.as_str().unwrap();
        assert_eq!(bic.len(), 11);
    }

    #[test]
    fn test_generate_bic_with_invalid_length() {
        let args = vec![json!("bic"), json!(10)];
        let result = FakeOperator::generate(&args);
        assert!(result.is_err());
        match result {
            Err(DataFakeError::FakeOperatorError(msg)) => {
                assert!(msg.contains("BIC length must be 8 or 11"));
            }
            _ => panic!("Expected FakeOperatorError with specific message"),
        }
    }
}
