use std::env;

use error::{Error, ErrorKind, Result};
use manager::TaskManager;
use repo::Repo;

mod error;
mod manager;
mod repo;
mod task;
mod ui;

fn main() {
    let repo: Repo = Repo::new();
    match repo.init() {
        Ok(_) => (),
        Err(err) => {
            println!("[ERR.IO] failed to connect to database");
            println!("{}", err);
            return;
        }
    };

    let mut manager: TaskManager = TaskManager::new(repo);
    match manager.load() {
        Ok(_) => (),
        Err(err) => {
            println!("[ERR.IO] failed to load data from database");
            println!("{}", err);
            return;
        }
    };

    let args: Vec<String> = env::args().skip(1).collect();

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
    ui::table::Table::new(3)
        .with_header(vec![
            "Id".to_string(),
            "Description".to_string(),
            "Due".to_string(),
        ])
        .with_content(
            manager
                .get_tasks()
                .iter()
                .enumerate()
                .map(|(i, m)| {
                    vec![
                        (i + 1).to_string(),
                        m.description.to_string(),
                        match m.due_date {
                            Some(due) => due.to_string(),
                            None => "".to_string(),
                        },
                    ]
                })
                .collect(),
        )
        .display();

    return Ok(());
}
