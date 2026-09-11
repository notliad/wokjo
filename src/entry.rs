use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub timestamp: DateTime<Local>,
    pub message: String,
}

impl Entry {
    pub fn new(message: &str) -> Result<Entry, &'static str> {
        if message.trim().is_empty() {
            return Err("Message cannot be empty");
        }

        Ok(Entry {
            timestamp: Local::now(),
            message: message.to_string(),
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
