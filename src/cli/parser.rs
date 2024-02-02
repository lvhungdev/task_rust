use crate::error::Result;
use crate::error::{Error, ErrorKind};

pub enum Command {
    List,
    Add(String),
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
                "complete" => self.parse_done(),
                _ => Ok(Command::Unknown),
            },

            None => Ok(Command::List),
        };
    }

    fn parse_add(&self) -> Result<Command> {
        let args: Vec<String> = self
            .args
            .iter()
            .skip(2)
            .filter_map(|m| {
                return if m.is_empty() {
                    None
                } else {
                    Some(m.to_string())
                };
            })
            .collect();

        let name: String = args.join(" ");

        return Ok(Command::Add(name));
    }

    fn parse_done(&self) -> Result<Command> {
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
