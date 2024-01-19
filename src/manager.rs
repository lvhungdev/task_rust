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

    pub fn get_tasks(&self) -> &Vec<Task> {
        return &self.tasks;
    }

    pub fn add_task(&mut self, name: &str) -> Result<usize> {
        if name.is_empty() {
            return Err(Error(ErrorKind::Input("name cannot be blank".to_string())));
        }

        self.tasks.push(Task::new(name));

        return Ok(self.tasks.len() - 1);
    }

    pub fn complete_task(&mut self, index: usize) -> Result<usize> {
        return match self.tasks.get_mut(index) {
            Some(_) => {
                // TODO Find a better way to handle completing tasks
                self.tasks.remove(index);
                return Ok(index);
            }
            None => Err(Error(ErrorKind::Input("id not found".to_string()))),
        };
    }

    pub fn save(&self) -> Result<()> {
        FileUtils::save(&self.db_path, &self.tasks)?;

        return Ok(());
    }
}
