use chrono::Local;

use crate::error::{Error, ErrorKind, Result};
use crate::{repo::Repo, task::Task};

pub struct TaskManager {
    repo: Repo,
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new(repo: Repo) -> Self {
        return Self {
            repo,
            tasks: Vec::new(),
        };
    }

    pub fn load(&mut self) -> Result<()> {
        self.tasks = self.repo.get(false)?;

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
        self.repo.add(self.tasks.last().unwrap())?;

        return Ok(self.tasks.len() - 1);
    }

    pub fn complete_task(&mut self, index: usize) -> Result<usize> {
        return match self.tasks.get_mut(index) {
            Some(_) => {
                let mut task = self.tasks.remove(index);

                task.is_completed = true;
                task.completed_date = Some(Local::now());

                self.repo.update(&task)?;

                return Ok(index);
            }
            None => Err(Error(ErrorKind::Input("id not found".to_string()))),
        };
    }
}
