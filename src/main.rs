use std::env;

use error::{Error, ErrorKind, Result};
use file::FileUtils;
use task::Task;

mod error;
mod file;
mod task;

const DB_PATH: &str = "./db.json";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match FileUtils::load::<Vec<Task>>(DB_PATH) {
        Ok(mut tasks) => match handle(&mut tasks, &args) {
            Ok(_) => (),
            Err(err) => println!("{}", err),
        },
        Err(err) => {
            println!("[ERR.IO] failed to load data from file");
            println!("{}", err);
        }
    };
}

fn handle(tasks: &mut Vec<Task>, args: &Vec<String>) -> Result<()> {
    match args.get(0) {
        Some(arg) => match arg.as_str() {
            "add" => add_task(tasks, args)?,
            "complete" => complete_task()?,
            _ => println!("[ERR] unknown command"),
        },
        None => list_task(tasks),
    };

    return Ok(());
}

fn list_task(tasks: &mut Vec<Task>) {
    for task in tasks {
        println!("{}", task);
    }
}

fn add_task(tasks: &mut Vec<Task>, args: &Vec<String>) -> Result<()> {
    let args: Vec<String> = args
        .iter()
        .skip(1)
        .filter_map(|m| {
            return if m.is_empty() {
                None
            } else {
                Some(m.to_string())
            };
        })
        .collect();

    if args.is_empty() {
        return Err(Error(ErrorKind::Input("name cannot be blank".to_string())));
    }

    let name: String = args.join(" ");

    let task: Task = Task::new(name);

    tasks.push(task);

    FileUtils::save(DB_PATH, tasks)?;

    return Ok(());
}

fn complete_task() -> Result<()> {
    Ok(())
}
