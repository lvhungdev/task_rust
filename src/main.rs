use std::env;

use error::Result;
use manager::TaskManager;

mod error;
mod file;
mod manager;
mod task;

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

fn handle(manager: &mut TaskManager, args: &Vec<String>) -> Result<()> {
    match args.get(0) {
        Some(arg) => match arg.as_str() {
            "add" => {
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

                manager.add_task(&name)?;
                manager.save()?;
            }
            "complete" => match args.get(1) {
                Some(index) => {
                    match index.to_string().parse::<usize>() {
                        Ok(index) => {
                            manager.complete_task(index)?;
                            manager.save()?;
                        }
                        Err(_) => println!("[ERR.INPUT] id is invalid"),
                    };
                }
                None => println!("[ERR.INPUT] id not found"),
            },
            _ => println!("[ERR.INPUT] unknown command"),
        },
        None => manager.list_task(),
    };

    return Ok(());
}
