use regex::Regex;
use std::collections::HashMap;
use std::sync::Arc; // Ensure you add this crate in your Cargo.toml

use crate::utils::singleton::AppState;

pub struct ValidationChain {
    data: Arc<AppState>,
    rules: HashMap<String, Vec<String>>,
}

impl ValidationChain {
    pub fn new(data: Arc<AppState>) -> Self {
        ValidationChain {
            data,
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, field: &str, rule: &str, value: Option<&str>) {
        let mut rule_string = rule.to_string();
        if let Some(val) = value {
            rule_string.push_str(&format!(":{}", val));
        }

        self.rules
            .entry(field.to_string())
            .or_insert_with(Vec::new)
            .push(rule_string);
    }

    pub fn get_rules(&self, field: &str) -> Option<&Vec<String>> {
        self.rules.get(field)
    }

    pub async fn validate(&self, field: &str, value: &str, table: &str) -> Result<(), String> {
        let rules = match self.get_rules(field) {
            Some(rules) => rules,
            None => return Ok(()), // No rules to validate
        };

        for rule in rules {
            let parts: Vec<&str> = rule.split(':').collect();
            let rule_name = parts[0];
            let rule_value = parts.get(1).map(|v| *v);

            match rule_name {
                "required" => {
                    if value.is_empty() {
                        return Err(format!("{} is required", field));
                    }
                }
                "min_length" => {
                    if let Some(min_length_str) = rule_value {
                        let min_length = min_length_str
                            .parse::<usize>()
                            .map_err(|_| format!("Invalid min_length value for {}", field))?;
                        if value.len() < min_length {
                            return Err(format!(
                                "{} must be at least {} characters long",
                                field, min_length
                            ));
                        }
                    }
                }
                "max_length" => {
                    if let Some(max_length_str) = rule_value {
                        let max_length = max_length_str
                            .parse::<usize>()
                            .map_err(|_| format!("Invalid max_length value for {}", field))?;
                        if value.len() > max_length {
                            return Err(format!(
                                "{} must be at most {} characters long",
                                field, max_length
                            ));
                        }
                    }
                }
                "email" => {
                    let re =
                        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
                    if !re.is_match(value) {
                        return Err(format!("{} is not a valid email", field));
                    }
                }
                "contains" => {
                    let mut has_uppercase = false;
                    let mut has_lowercase = false;
                    let mut has_number = false;
                    let mut has_special_char = false;
                    for c in value.chars() {
                        if c.is_ascii_uppercase() {
                            has_uppercase = true;
                        } else if c.is_ascii_lowercase() {
                            has_lowercase = true;
                        } else if c.is_ascii_digit() {
                            has_number = true;
                        } else if !c.is_ascii_alphanumeric() {
                            has_special_char = true;
                        }
                    }
                    if !has_uppercase {
                        return Err(format!("{} must contain an uppercase letter", field));
                    }
                    if !has_lowercase {
                        return Err(format!("{} must contain a lowercase letter", field));
                    }
                    if !has_number {
                        return Err(format!("{} must contain a number", field));
                    }
                    if !has_special_char {
                        return Err(format!("{} must contain a special character", field));
                    }
                }
                "unique" => {
                    let count: (i64,) = sqlx::query_as(&format!(
                        "SELECT COUNT(*) FROM {} WHERE {} = ?",
                        table, field
                    ))
                    .bind(value)
                    .fetch_one(&self.data.db)
                    .await
                    .map_err(|_| format!("Failed to check uniqueness of {}", field))?;

                    if count.0 > 0 {
                        return Err(format!("{} is already taken", field));
                    }
                }
                "phone" => {
                    let re = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap(); // Simple phone number regex
                    if !re.is_match(value) {
                        return Err(format!("{} is not a valid phone number", field));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
