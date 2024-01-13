use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::file::JsonSerializer;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_date: DateTime<Utc>,
    pub name: String,
    pub is_done: bool,
}

impl Task {
    pub fn new(name: String) -> Self {
        return Self {
            id: Uuid::new_v4().to_string(),
            created_date: Utc::now(),
            name,
            is_done: false,
        };
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(
            f,
            "{} - {} - {} - {}",
            self.id, self.name, self.created_date, self.is_done
        );
    }
}

impl JsonSerializer for Task {
    fn to_json(&self) -> String {
        // TODO handle errors
        return serde_json::to_string(self).unwrap();
    }

    fn from_json(json: &str) -> Self {
        // TODO handle errors
        return serde_json::from_str(json).unwrap();
    }
}

impl JsonSerializer for Vec<Task> {
    fn to_json(&self) -> String {
        // TODO handle errors
        return serde_json::to_string(self).unwrap();
    }

    fn from_json(json: &str) -> Self {
        // TODO handle errors

        return if json.is_empty() {
            Vec::new()
        } else {
            serde_json::from_str(json).unwrap()
        };
    }
}
