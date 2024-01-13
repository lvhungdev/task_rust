use file::{FileUtils, JsonSerializer};
use task::Task;

mod file;
mod task;
mod error;

fn main() {
    let db_path: &str = "./db.json";

    let mut tasks: Vec<Task> = FileUtils::load(db_path).unwrap();

    for task in &tasks {
        println!("{}", task.to_json());
    }

    tasks.push(Task::new(format!("Task #{}", tasks.len())));

    FileUtils::save(db_path, &tasks);
}
