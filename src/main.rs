use error::Result;
use file::{FileUtils, JsonSerializer};
use task::Task;

mod error;
mod file;
mod task;

fn main() -> Result<()> {
    let db_path: &str = "./db.json";

    let mut tasks: Vec<Task> = FileUtils::load(db_path)?;

    for task in &tasks {
        println!("{}", task.to_json()?);
    }

    tasks.push(Task::new(format!("Task #{}", tasks.len())));

    match FileUtils::save(db_path, &tasks) {
        Ok(_) => todo!(),
        Err(err) => println!("{}", err),
    }

    return Ok(());
}
