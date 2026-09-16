use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub id: String,
    pub timestamp: DateTime<Local>,
    pub message: String,

    #[serde(default)]
    pub tags: Vec<String>,
}

fn generate_id() -> String {
    Uuid::new_v4().to_string()[..6].to_string()
}

impl Entry {
    pub fn formatted_tags(&self) -> String {
        self.tags
            .iter()
            .map(|tag| format!("[{}]", tag))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn new(message: &str, tags: Vec<String>) -> Result<Entry, &'static str> {
        if message.trim().is_empty() {
            return Err("Message cannot be empty");
        }

        Ok(Entry {
            id: generate_id(),
            timestamp: Local::now(),
            message: message.to_string(),
            tags,
        })
    }

    pub fn formatted_time(&self) -> String {
        self.timestamp.format("%H:%M").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create_entry_with_message() {
        let entry = Entry::new("Bug fixed").unwrap();

        assert_eq!(entry.message, "Bug fixed");
    }

    #[test]
    fn should_reject_empty_message() {
        let result = Entry::new("");

        assert!(result.is_err());
    }

    #[test]
    fn should_reject_whitespace_only_message() {
        let result = Entry::new("      ");

        assert!(result.is_err());
    }
}
