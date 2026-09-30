use chrono::{DateTime, Local, NaiveDate};
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
    fn validate_message(message: &str) -> Result<(), &'static str> {
        if message.trim().is_empty() {
            return Err("Message cannot be empty");
        }

        Ok(())
    }

    pub fn formatted_tags(&self) -> String {
        self.tags
            .iter()
            .map(|tag| format!("[{}]", tag))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn new(
        message: &str,
        tags: Vec<String>,
        date: Option<NaiveDate>,
    ) -> Result<Entry, &'static str> {
        Self::validate_message(message)?;

        let mut entry = Entry {
            id: generate_id(),
            timestamp: Local::now(),
            message: message.to_string(),
            tags,
        };

        if let Some(date) = date {
            entry.set_date(date)?;
        }

        Ok(entry)
    }

    pub fn set_message(&mut self, message: &str) -> Result<(), &'static str> {
        Self::validate_message(message)?;
        self.message = message.to_string();

        Ok(())
    }

    pub fn set_date(&mut self, date: NaiveDate) -> Result<(), &'static str> {
        self.timestamp = date
            .and_time(self.timestamp.time())
            .and_local_timezone(Local)
            .single()
            .ok_or("Date and current time are not valid in the local timezone")?;

        Ok(())
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
        let date = NaiveDate::from_ymd_opt(2026, 9, 29).unwrap();
        let mut entry = Entry::new("Bug fixed", vec!["backend".to_string()], None).unwrap();
        entry.set_message("Bug reviewed").unwrap();
        entry.set_date(date).unwrap();

        assert_eq!(entry.message, "Bug reviewed");
        assert_eq!(entry.tags, vec!["backend"]);
        assert_eq!(entry.timestamp.date_naive(), date);
    }

    #[test]
    fn should_reject_empty_message() {
        let result = Entry::new("", Vec::new(), None);

        assert!(result.is_err());
    }

    #[test]
    fn should_reject_whitespace_only_message() {
        let result = Entry::new("      ", Vec::new(), None);

        assert!(result.is_err());
    }
}
