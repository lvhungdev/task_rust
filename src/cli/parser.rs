use chrono::{DateTime, Local};

use crate::error::Result;
use crate::error::{Error, ErrorKind};

pub enum Command {
    List,
    Add(String, Option<DateTime<Local>>),
    Complete(usize),
    Unknown,
}

pub struct Parser {
    args: Vec<String>,
}

impl Parser {
    pub fn new(args: Vec<String>) -> Self {
        return Parser { args };
    }

    pub fn parse(&self) -> Result<Command> {
        return match self.args.get(1) {
            Some(arg) => match arg.as_str() {
                "add" => self.parse_add(),
                "complete" => self.parse_complete(),
                _ => Ok(Command::Unknown),
            },

            None => Ok(Command::List),
        };
    }

    // TODO Refactor this method to make it more concise and readable
    // This method for now handles only due_date option
    // Ideally, we should have a method that processes for each option
    // E.g due_date, project, etc
    fn parse_add(&self) -> Result<Command> {
        let mut name_vec: Vec<String> = Vec::with_capacity(self.args.len());
        let mut due_date: Option<DateTime<Local>> = None;

        for arg in self.args.iter().skip(2) {
            if arg.contains(":") {
                let mut iter = arg.split(":");
                let key: &str = iter.next().unwrap();
                let value: String = iter.collect::<Vec<&str>>().join(":");

                match key {
                    "due" => {
                        match value.parse::<DateTime<Local>>() {
                            Ok(date) => due_date = Some(date),
                            Err(_) => {
                                return Err(Error(ErrorKind::Input(format!(
                                    "invalid date format: {}",
                                    value
                                ))));
                            }
                        };
                    }
                    _ => {
                        return Err(Error(ErrorKind::Input(format!(
                            "unknown argument: {}",
                            key
                        ))));
                    }
                };
            } else if !arg.is_empty() {
                name_vec.push(arg.to_string());
            }
        }

        return Ok(Command::Add(name_vec.join(" "), due_date));
    }

    fn parse_complete(&self) -> Result<Command> {
        return match self.args.get(2) {
            Some(index) => match index.to_string().parse::<usize>() {
                Ok(index) => {
                    return if index == 0 {
                        Err(Error(ErrorKind::Input("id not found".to_string())))
                    } else {
                        Ok(Command::Complete(index))
                    };
                }

                Err(_) => Err(Error(ErrorKind::Input("id is invalid".to_string()))),
            },

            None => Err(Error(ErrorKind::Input("id not found".to_string()))),
        };
    }
}
