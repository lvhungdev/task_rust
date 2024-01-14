use crate::error::{Error, ErrorKind, Result};
use crate::{file::FileUtils, task::Task};

pub struct TaskManager {
    db_path: String,
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new(db_path: &str) -> Self {
        return Self {
            db_path: db_path.to_string(),
            tasks: Vec::new(),
        };
    }

    pub fn load(&mut self) -> Result<()> {
        self.tasks = FileUtils::load(&self.db_path)?;

        return Ok(());
    }

    pub fn list_task(&self) {
        for task in &self.tasks {
            println!("{}", task);
        }
    }

    pub fn add_task(&mut self, name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(Error(ErrorKind::Input("name cannot be blank".to_string())));
        }

        self.tasks.push(Task::new(name));

        return Ok(());
    }

    pub fn save(&self) -> Result<()> {
        FileUtils::save(&self.db_path, &self.tasks)?;

        return Ok(());
    }
}
