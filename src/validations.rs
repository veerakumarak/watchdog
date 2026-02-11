use serde_json::Value;
use validator::ValidationError;
use validify::validate_email;

pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') && (4..=32).contains(&name.len()) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid_name"))
    }
}

pub fn validate_email_list(emails: &Vec<String>) -> Result<(), ValidationError> {
    for (index, address) in emails.iter().enumerate() {
        if !validate_email(address) {
            return Err(ValidationError {
                code: "email_format".into(),
                message: Some(format!("Invalid email format at index {}: {}", index, address).into()),
                params: std::collections::HashMap::new(),
            });
        }
    }

    Ok(())
}

pub fn validate_url(url: &String) -> Result<(), ValidationError> {
    // Check if it starts with http/https and contains "://"
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(ValidationError::new("invalid_url"));
    }

    // Check if it's too short to contain a domain
    if url.len() < 8 { // "http://x" is 8 chars
        return Err(ValidationError::new("invalid_url"));
    }

    // Basic check for whitespace (URLs should not have spaces)
    if url.contains(char::is_whitespace) {
        return Err(ValidationError::new("invalid_url"));
    }
    Ok(())
}

pub fn validate_config_json(config_str: &str) -> Result<(), ValidationError> {
    // Step 1: Is it valid JSON?
    let _json: Value = serde_json::from_str(config_str)
        .map_err(|_| ValidationError::new("invalid_json"))?;

    // Step 2: Extract the specific field you need to validate
    // Let's assume the JSON looks like: {"tags": "job_one, job_two"}
    // if let Some(tags_val) = json.get("tags").and_then(|v| v.as_str()) {
    //     if !is_valid_comma_snake(tags_val) {
    //         return Err(ValidationError::new("invalid_snake_case_csv"));
    //     }
    // }

    Ok(())
}