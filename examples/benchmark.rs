use datafake_rs::{DataFakeError, DataGenerator};
use serde_json::json;
use std::time::Instant;

fn main() -> Result<(), DataFakeError> {
    let config = json!({
        "metadata": {
            "name": "Complex Benchmark Test",
            "description": "Complex data generation benchmark with JSONLogic and fake operators"
        },
        "variables": {
            "user_id": {"fake": ["uuid"]},
            "base_timestamp": {"fake": ["u64", 1704067200, 1735689600]},
            "country_code": {"fake": ["country_code"]},
            "currency": {"fake": ["currency_code"]},
            "age": {"fake": ["u8", 18, 65]},
            "is_premium": {"fake": ["bool"]},
            "base_salary": {"fake": ["f64", 30000, 150000]},
            "company_name": {"fake": ["company_name"]},
            "random_score": {"fake": ["f64", 0, 1]}
        },
        "schema": {
            "id": {"var": "user_id"},
            "timestamp": {"var": "base_timestamp"},
            "user": {
                "personal": {
                    "first_name": {"fake": ["first_name"]},
                    "last_name": {"fake": ["last_name"]},
                    "full_name": {
                        "cat": [
                            {"fake": ["first_name"]},
                            " ",
                            {"fake": ["last_name"]}
                        ]
                    },
                    "email": {
                        "if": [
                            {"var": "is_premium"},
                            {"fake": ["email"]},
                            {"fake": ["free_email"]}
                        ]
                    },
                    "phone": {"fake": ["phone_number"]},
                    "age": {"var": "age"},
                    "is_adult": {">=": [{"var": "age"}, 18]},
                    "age_group": {
                        "if": [
                            {"<": [{"var": "age"}, 25]},
                            "young",
                            {"if": [
                                {"<": [{"var": "age"}, 40]},
                                "adult",
                                {"if": [
                                    {"<": [{"var": "age"}, 60]},
                                    "middle-aged",
                                    "senior"
                                ]}
                            ]}
                        ]
                    },
                    "username": {
                        "cat": [
                            {"fake": ["word"]},
                            "_",
                            {"fake": ["word"]},
                            "_",
                            {"fake": ["u16", 100, 999]}
                        ]
                    },
                    "password_hash": {"fake": ["password", 12, 16]},
                    "bio": {"fake": ["paragraph", 2, 4]}
                },
                "address": {
                    "street": {"fake": ["street_name"]},
                    "building": {"fake": ["u16", 1, 999]},
                    "full_address": {
                        "cat": [
                            {"fake": ["u16", 1, 999]},
                            " ",
                            {"fake": ["street_name"]},
                            " ",
                            {"fake": ["street_suffix"]}
                        ]
                    },
                    "city": {"fake": ["city"]},
                    "state": {"fake": ["state_name"]},
                    "country": {"var": "country_code"},
                    "postal_code": {"fake": ["post_code"]},
                    "coordinates": {
                        "lat": {"fake": ["latitude"]},
                        "lng": {"fake": ["longitude"]},
                        "formatted": {
                            "cat": [
                                {"fake": ["latitude"]},
                                ", ",
                                {"fake": ["longitude"]}
                            ]
                        }
                    }
                },
                "professional": {
                    "company": {"var": "company_name"},
                    "job_title": {"fake": ["profession"]},
                    "department": {"fake": ["industry"]},
                    "years_experience": {
                        "max": [
                            0,
                            {"-": [
                                {"var": "age"},
                                18
                            ]}
                        ]
                    },
                    "email": {
                        "cat": [
                            {"fake": ["word"]},
                            ".",
                            {"fake": ["word"]},
                            "@",
                            {"fake": ["word"]},
                            ".com"
                        ]
                    },
                    "company_suffix": {"fake": ["company_suffix"]},
                    "catch_phrase": {"fake": ["catch_phrase"]}
                },
                "financial": {
                    "currency": {"var": "currency"},
                    "credit_card": {"fake": ["credit_card_number"]},
                    "bank_account": {"fake": ["bic"]},
                    "base_salary": {"var": "base_salary"},
                    "bonus_percentage": {
                        "if": [
                            {"var": "is_premium"},
                            {"fake": ["f64", 15, 30]},
                            {"fake": ["f64", 5, 15]}
                        ]
                    },
                    "total_compensation": {
                        "*": [
                            {"var": "base_salary"},
                            {"+": [
                                1,
                                {"/": [
                                    {"if": [
                                        {"var": "is_premium"},
                                        0.225,
                                        0.1
                                    ]},
                                    1
                                ]}
                            ]}
                        ]
                    },
                    "credit_score": {
                        "if": [
                            {"var": "is_premium"},
                            {"fake": ["u16", 700, 850]},
                            {"fake": ["u16", 500, 700]}
                        ]
                    },
                    "has_debt": {"<": [{"var": "random_score"}, 0.6]},
                    "debt_amount": {
                        "if": [
                            {"<": [{"var": "random_score"}, 0.6]},
                            {"fake": ["f64", 1000, 50000]},
                            0
                        ]
                    }
                },
                "internet": {
                    "ip_v4": {"fake": ["ipv4"]},
                    "ip_v6": {"fake": ["ipv6"]},
                    "mac_address": {"fake": ["mac_address"]},
                    "user_agent": {"fake": ["user_agent"]},
                    "domain": {
                        "cat": [
                            {"fake": ["word"]},
                            ".",
                            {"fake": ["domain_suffix"]}
                        ]
                    },
                    "bandwidth_mbps": {
                        "if": [
                            {"var": "is_premium"},
                            {"fake": ["u16", 100, 1000]},
                            {"fake": ["u16", 10, 100]}
                        ]
                    }
                },
                "preferences": {
                    "theme": {
                        "if": [
                            {"<": [{"var": "random_score"}, 0.7]},
                            "dark",
                            "light"
                        ]
                    },
                    "language": {
                        "if": [
                            {"==": [{"var": "country_code"}, "US"]},
                            "en-US",
                            {"if": [
                                {"==": [{"var": "country_code"}, "GB"]},
                                "en-GB",
                                {"if": [
                                    {"==": [{"var": "country_code"}, "FR"]},
                                    "fr-FR",
                                    "en-US"
                                ]}
                            ]}
                        ]
                    },
                    "notifications": {
                        "email": {"var": "is_premium"},
                        "push": {"<": [{"var": "random_score"}, 0.8]},
                        "sms": {"and": [
                            {"var": "is_premium"},
                            {"<": [{"var": "random_score"}, 0.5]}
                        ]}
                    },
                    "privacy_level": {
                        "if": [
                            {"var": "is_premium"},
                            {"if": [
                                {"<": [{"var": "random_score"}, 0.7]},
                                "high",
                                "medium"
                            ]},
                            {"if": [
                                {"<": [{"var": "random_score"}, 0.3]},
                                "high",
                                {"if": [
                                    {"<": [{"var": "random_score"}, 0.7]},
                                    "medium",
                                    "low"
                                ]}
                            ]}
                        ]
                    }
                },
                "metadata": {
                    "account_created": {"fake": ["u64", 1577836800, 1704067200]},
                    "last_login": {"fake": ["u64", 1735689600, 1738368000]},
                    "login_count": {
                        "*": [
                            {"fake": ["u16", 10, 365]},
                            {"-": [
                                2024,
                                {"+": [
                                    2020,
                                    {"fake": ["u8", 0, 3]}
                                ]}
                            ]}
                        ]
                    },
                    "is_verified": {"or": [
                        {"var": "is_premium"},
                        {"<": [{"var": "random_score"}, 0.7]}
                    ]},
                    "subscription_tier": {
                        "if": [
                            {"var": "is_premium"},
                            {"if": [
                                {"<": [{"var": "random_score"}, 0.5]},
                                "pro",
                                "enterprise"
                            ]},
                            "free"
                        ]
                    },
                    "risk_score": {
                        "max": [
                            0,
                            {"min": [
                                100,
                                {"+": [
                                    {"if": [{"var": "is_verified"}, 0, 20]},
                                    {"if": [{"var": "is_premium"}, 0, 15]},
                                    {"*": [
                                        {"var": "random_score"},
                                        65
                                    ]}
                                ]}
                            ]}
                        ]
                    }
                }
            },
            "activity": {
                "recent_transactions": [
                    {
                        "id": {"fake": ["uuid"]},
                        "amount": {
                            "*": [
                                {"fake": ["f64", 10, 500]},
                                {"if": [{"var": "is_premium"}, 1.5, 1]}
                            ]
                        },
                        "merchant": {"fake": ["company_name"]},
                        "category": {"fake": ["industry"]},
                        "timestamp": {"fake": ["u64", 1735000000, 1738000000]},
                        "approved": {"or": [
                            {"var": "is_premium"},
                            {">": [{"var": "random_score"}, 0.1]}
                        ]}
                    },
                    {
                        "id": {"fake": ["uuid"]},
                        "amount": {
                            "*": [
                                {"fake": ["f64", 10, 500]},
                                {"if": [{"var": "is_premium"}, 1.5, 1]}
                            ]
                        },
                        "merchant": {"fake": ["company_name"]},
                        "category": {"fake": ["industry"]},
                        "timestamp": {"fake": ["u64", 1735000000, 1738000000]},
                        "approved": {"or": [
                            {"var": "is_premium"},
                            {">": [{"var": "random_score"}, 0.1]}
                        ]}
                    },
                    {
                        "id": {"fake": ["uuid"]},
                        "amount": {
                            "*": [
                                {"fake": ["f64", 10, 500]},
                                {"if": [{"var": "is_premium"}, 1.5, 1]}
                            ]
                        },
                        "merchant": {"fake": ["company_name"]},
                        "category": {"fake": ["industry"]},
                        "timestamp": {"fake": ["u64", 1735000000, 1738000000]},
                        "approved": {"or": [
                            {"var": "is_premium"},
                            {">": [{"var": "random_score"}, 0.1]}
                        ]}
                    }
                ],
                "engagement_score": {
                    "*": [
                        100,
                        {"min": [
                            1,
                            {"max": [
                                0,
                                {"+": [
                                    {"if": [{"var": "is_premium"}, 0.3, 0]},
                                    {"if": [{"var": "is_verified"}, 0.2, 0]},
                                    {"*": [{"var": "random_score"}, 0.5]}
                                ]}
                            ]}
                        ]}
                    ]
                },
                "features_used": {
                    "if": [
                        {"var": "is_premium"},
                        ["dashboard", "profile", "settings", "analytics", "api", "export", "import"],
                        ["dashboard", "profile", "settings"]
                    ]
                }
            }
        }
    });

    let generator = DataGenerator::from_value(config)?;

    println!("Starting benchmark: generating complex data 100,000 times...");
    println!("This benchmark uses a mix of:");
    println!("- Fake data operators");
    println!("- JSONLogic expressions (if/then/else, arithmetic, string operations)");
    println!("- Variable references");
    println!("- Complex nested structures");
    println!();

    let iterations = 100_000;
    let mut last_progress = 0;

    let start = Instant::now();

    for i in 0..iterations {
        let _ = generator.generate()?;

        let progress = (i * 100) / iterations;
        if progress > last_progress && progress % 10 == 0 {
            println!("Progress: {progress}%");
            last_progress = progress;
        }
    }

    let duration = start.elapsed();

    println!("\n=== Benchmark Results ===");
    println!("Total iterations: {iterations}");
    println!("Total time: {duration:.2?}");
    println!(
        "Average time per generation: {:.2?}",
        duration / iterations as u32
    );
    println!(
        "Generations per second: {:.2}",
        iterations as f64 / duration.as_secs_f64()
    );

    let sample = generator.generate()?;
    println!(
        "\nSample output size: {} bytes",
        serde_json::to_string(&sample)?.len()
    );
    println!("\nSample output (pretty printed):");
    println!("{}", serde_json::to_string_pretty(&sample)?);

    Ok(())
}
