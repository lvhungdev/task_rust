use std::env;

use chrono::NaiveDateTime;
use converter::TimeConverter;
use error::{Error, ErrorKind, Result};
use parser::{CliParser, Command};
use repo::Repo;
use task::TaskManager;

mod converter;
mod error;
mod parser;
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
    let parser: CliParser = CliParser::new(env::args().collect());

    return match parser.parse()? {
        Command::List => handle_list(manager),
        Command::Add(name, due_date, priority) => handle_add(manager, &name, due_date, priority),
        Command::Complete(index) => handle_complete(manager, index),
        Command::Unknown => Err(Error(ErrorKind::Input("unknown command".to_string()))),
    };
}

fn handle_add(
    manager: &mut TaskManager,
    name: &str,
    due_date: Option<NaiveDateTime>,
    priority: Option<task::Priority>,
) -> Result<()> {
    let index: usize = manager.add_task(&name, due_date, priority)?;

    println!("Created task {}", index + 1);

    return Ok(());
}

fn handle_complete(manager: &mut TaskManager, index: usize) -> Result<()> {
    let index: usize = manager.complete_task(index - 1)?;

    println!("Completed task {}", index + 1);

    return Ok(());
}

fn handle_list(manager: &mut TaskManager) -> Result<()> {
    if manager.get_tasks().is_empty() {
        println!("empty");
        return Ok(());
    }

    ui::Table::new(5)
        .with_header(vec![
            "Id".to_string(),
            "Desc".to_string(),
            "Pri".to_string(),
            "Due".to_string(),
            "Urg".to_string(),
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
                        match &m.priority {
                            Some(p) => p.to_string(),
                            None => "".to_string(),
                        },
                        match m.due_date {
                            Some(due) => TimeConverter::get_relative_time_since_now(due),
                            None => "".to_string(),
                        },
                        format!("{:.1}", (m.get_urgency() * 10.0).round() / 10.0),
                    ]
                })
                .collect(),
        )
        .display();

    return Ok(());
}
