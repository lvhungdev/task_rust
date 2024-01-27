use chrono::{DateTime, Local};

pub struct Task {
    pub id: usize,
    pub description: String,
    pub created_date: DateTime<Local>,
    pub completed_date: Option<DateTime<Local>>,
    pub is_completed: bool,
    pub due_date: Option<DateTime<Local>>,
}

impl Task {
    pub fn new(name: &str) -> Self {
        return Self {
            id: 0,
            description: name.to_string(),
            created_date: Local::now(),
            completed_date: None,
            is_completed: false,
            due_date: None,
        };
    }
}
