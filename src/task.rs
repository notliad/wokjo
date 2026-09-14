use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub message: String,
    pub done: bool,
}

fn generate_id() -> String {
    Uuid::new_v4().to_string()[..6].to_string()
}

impl Task {
    pub fn new(message: &str) -> Result<Task, &'static str> {
        if message.trim().is_empty() {
            return Err("Message cannot be empty");
        }

        Ok(Task {
            id: generate_id(),
            message: message.to_string(),
            done: false,
        })
    }
}
