use rand::{distributions::Alphanumeric, Rng};
use std::collections::HashMap;

pub fn generate_token(data: &str) -> String {
    let random_string: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    format!("{}-{}", data.len(), random_string)
}

pub fn validate_data_format(data: &HashMap<String, String>, format: &HashMap<String, String>) -> Result<(), String> {
    for (key, expected_type) in format {
        if let Some(value) = data.get(key) {
            match expected_type.as_str() {
                "string" => continue,
                "number" => {
                    if value.parse::<f64>().is_err() {
                        return Err(format!("Field '{}' expected to be a number", key));
                    }
                }
                _ => return Err(format!("Unknown format type: {}", expected_type)),
            }
        }
    }
    Ok(())
}
