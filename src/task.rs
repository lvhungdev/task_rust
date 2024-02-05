use chrono::{Local, NaiveDateTime};

use crate::error::{Error, ErrorKind, Result};
use crate::repo::Repo;

pub struct Task {
    pub id: usize,
    pub description: String,
    pub created_date: NaiveDateTime,
    pub completed_date: Option<NaiveDateTime>,
    pub is_completed: bool,
    pub due_date: Option<NaiveDateTime>,
}

impl Task {
    pub fn new(name: &str, due_date: Option<NaiveDateTime>) -> Self {
        return Self {
            id: 0,
            description: name.to_string(),
            created_date: Local::now().naive_local(),
            completed_date: None,
            is_completed: false,
            due_date,
        };
    }
}

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

    pub fn add_task(&mut self, name: &str, due_date: Option<NaiveDateTime>) -> Result<usize> {
        if name.is_empty() {
            return Err(Error(ErrorKind::Input("name cannot be blank".to_string())));
        }

        self.tasks.push(Task::new(name, due_date));
        self.repo.add(self.tasks.last().unwrap())?;

        return Ok(self.tasks.len() - 1);
    }

    pub fn complete_task(&mut self, index: usize) -> Result<usize> {
        return match self.tasks.get_mut(index) {
            Some(_) => {
                let mut task = self.tasks.remove(index);

                task.is_completed = true;
                task.completed_date = Some(Local::now().naive_local());

                self.repo.update(&task)?;

                return Ok(index);
            }
            None => Err(Error(ErrorKind::Input("id not found".to_string()))),
        };
    }
}
