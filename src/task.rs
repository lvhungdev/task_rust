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

    pub fn get_urgency(&self) -> f64 {
        let mut urgency: f64 = 0.0;
        let now: NaiveDateTime = Local::now().naive_local();

        if let Some(due_date) = self.due_date {
            urgency += 0.2;

            let duration = due_date.signed_duration_since(now);
            let num_seconds = 86400.0 * 7.0 - duration.num_seconds() as f64;

            if num_seconds >= 0.0 {
                let urg_per_day = 1.0;
                let urg_per_sec = urg_per_day / 86400.0;
                urgency += num_seconds * urg_per_sec;
            }
        }

        return urgency;
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
        self.sort_tasks();

        return Ok(());
    }

    pub fn get_tasks(&self) -> &Vec<Task> {
        return &self.tasks;
    }

    pub fn add_task(&mut self, name: &str, due_date: Option<NaiveDateTime>) -> Result<usize> {
        if name.is_empty() {
            return Err(Error(ErrorKind::Input("name cannot be blank".to_string())));
        }

        let task = Task::new(name, due_date);
        let created_date = task.created_date;

        self.tasks.push(task);
        self.repo.add(self.tasks.last().unwrap())?;

        self.sort_tasks();

        let index = self
            .tasks
            .iter()
            .position(|m| m.created_date == created_date)
            .unwrap();

        return Ok(index);
    }

    pub fn complete_task(&mut self, index: usize) -> Result<usize> {
        return match self.tasks.get_mut(index) {
            Some(_) => {
                let mut task = self.tasks.remove(index);

                task.is_completed = true;
                task.completed_date = Some(Local::now().naive_local());

                self.repo.update(&task)?;

                self.sort_tasks();

                return Ok(index);
            }
            None => Err(Error(ErrorKind::Input("id not found".to_string()))),
        };
    }

    fn sort_tasks(&mut self) {
        self.tasks.sort_by(|a, b| {
            let a_urg = a.get_urgency();
            let b_urg = b.get_urgency();

            return b_urg.partial_cmp(&a_urg).unwrap();
        });
    }
}
