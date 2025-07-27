use datafake_rs::DataGenerator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic configuration with the example from README
    let config = r#"{
        "metadata": {
            "name": "User Profile Generator",
            "version": "1.0.0",
            "description": "Generates realistic user profile data"
        },
        "variables": {
            "userId": {"fake": ["uuid"]},
            "country": {"fake": ["country_code"]}
        },
        "schema": {
            "id": {"var": "userId"},
            "profile": {
                "name": {"fake": ["name", "en_US"]},
                "age": {"fake": ["u8", 18, 65]},
                "address": {
                    "street": {"fake": ["street_address", "en_US"]},
                    "country": {"var": "country"}
                },
                "bankAccount": {
                    "bic": {"fake": ["bic"]},
                    "balance": {"fake": ["f32", 0, 10000.00]}
                }
            }
        }
    }"#;

    // Create the generator
    let generator = DataGenerator::from_json(config)?;

    // Generate fake data
    let fake_data = generator.generate()?;

    // Print the generated data
    println!("Generated User Profile:");
    println!("{}", serde_json::to_string_pretty(&fake_data)?);

    // Generate multiple records
    println!("\nGenerating 3 user profiles:");
    let batch = generator.generate_batch(3)?;
    for (i, data) in batch.iter().enumerate() {
        println!("\nProfile {}:", i + 1);
        println!("{}", serde_json::to_string_pretty(data)?);
    }

    Ok(())
}
