use std::env;

use cli::parser::{Command, Parser};
use error::{Error, ErrorKind, Result};
use manager::TaskManager;
use repo::Repo;

mod cli;
mod error;
mod manager;
mod repo;
mod task;
mod ui;

fn main() {
    let repo: Repo = Repo::new();
    if let Err(err) = repo.init() {
        println!("[ERR.IO] failed to connect to database");
        println!("{}", err);
        return;
    }

    let mut manager: TaskManager = TaskManager::new(repo);
    if let Err(err) = manager.load() {
        println!("[ERR.IO] failed to load data from database");
        println!("{}", err);
        return;
    };

    if let Err(err) = handle(&mut manager) {
        println!("{}", err);
    }
}

fn handle(manager: &mut TaskManager) -> Result<()> {
    let parser: Parser = Parser::new(env::args().collect());

    return match parser.parse() {
        Ok(command) => match command {
            Command::List => handle_list(manager),
            Command::Add(name) => handle_add(manager, &name),
            Command::Complete(index) => handle_complete(manager, index),
            Command::Unknown => Err(Error(ErrorKind::Input("unknown command".to_string()))),
        },
        Err(err) => Err(err),
    };
}

fn handle_add(manager: &mut TaskManager, name: &str) -> Result<()> {
    let index: usize = manager.add_task(&name)?;

    println!("Created task {}", index + 1);

    return Ok(());
}

fn handle_complete(manager: &mut TaskManager, index: usize) -> Result<()> {
    let index: usize = manager.complete_task(index - 1)?;

    println!("Completed task {}", index + 1);

    return Ok(());
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
