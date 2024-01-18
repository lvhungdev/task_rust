use std::env;

use error::{Error, ErrorKind, Result};
use manager::TaskManager;
use ui::UI;

mod error;
mod file;
mod manager;
mod task;
mod ui;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut manager: TaskManager = TaskManager::new("./db.json");

    match manager.load() {
        Ok(_) => (),
        Err(err) => {
            println!("[ERR.IO] failed to load data from file");
            println!("{}", err);
        }
    };

    match handle(&mut manager, &args) {
        Ok(_) => (),
        Err(err) => {
            println!("{}", err);
        }
    }
}

fn handle(manager: &mut TaskManager, args: &[String]) -> Result<()> {
    return match args.get(0) {
        Some(arg) => match arg.as_str() {
            "add" => handle_add(manager, args),
            "complete" => handle_complete(manager, args),
            _ => Err(Error(ErrorKind::Input("unknown command".to_string()))),
        },

        None => handle_list(manager),
    };
}

fn handle_add(manager: &mut TaskManager, args: &[String]) -> Result<()> {
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

    let name: String = args.join(" ");

    let index: usize = manager.add_task(&name)?;
    manager.save()?;

    println!("Created task {}", index + 1);

    return Ok(());
}

fn handle_complete(manager: &mut TaskManager, args: &[String]) -> Result<()> {
    return match args.get(1) {
        Some(index) => {
            return match index.to_string().parse::<usize>() {
                Ok(index) => {
                    if index == 0 {
                        return Err(Error(ErrorKind::Input("id not found".to_string())));
                    }

                    let index: usize = manager.complete_task(index - 1)?;
                    manager.save()?;

                    println!("Completed task {}", index + 1);

                    return Ok(());
                }
                Err(_) => Err(Error(ErrorKind::Input("id is invalid".to_string()))),
            };
        }
        None => Err(Error(ErrorKind::Input("id not found".to_string()))),
    };
}

fn handle_list(manager: &mut TaskManager) -> Result<()> {
    UI::display_tasks(manager.get_tasks());
    return Ok(());
}
