use chrono::{Datelike, Duration, Local, NaiveDateTime};

use crate::error::Result;
use crate::error::{Error, ErrorKind};

pub enum Command {
    List,
    Add(String, Option<NaiveDateTime>),
    Complete(usize),
    Unknown,
}

pub struct CliParser {
    args: Vec<String>,
}

impl CliParser {
    pub fn new(args: Vec<String>) -> Self {
        return CliParser { args };
    }

    pub fn parse(&self) -> Result<Command> {
        return match self.args.get(1) {
            Some(arg) => match arg.as_str() {
                "add" => self.parse_add(),
                "complete" | "cmp" => self.parse_complete(),
                _ => Ok(Command::Unknown),
            },

            None => Ok(Command::List),
        };
    }

    fn parse_add(&self) -> Result<Command> {
        let mut name_vec: Vec<String> = Vec::with_capacity(self.args.len());
        let mut due_date: Option<NaiveDateTime> = None;

        for arg in self.args.iter().skip(2) {
            if arg.contains(":") {
                let mut iter = arg.split(":");
                let key: &str = iter.next().unwrap();
                let value: String = iter.collect::<Vec<&str>>().join(":");

                match key {
                    "due" => due_date = Some(TimeParser::parse(&value)?),
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

struct TimeParser;

impl TimeParser {
    fn parse(date_str: &str) -> Result<NaiveDateTime> {
        if let Some(date) = TimeParser::parse_absolute_date(date_str) {
            return Ok(date);
        }

        if let Some(date) = TimeParser::parse_relative_date(date_str) {
            return Ok(date);
        }

        if let Some(date) = TimeParser::parse_end_of_date(date_str) {
            return Ok(date);
        }

        return Err(Error(ErrorKind::Input(format!(
            "invalid date format: {}",
            date_str
        ))));
    }

    fn parse_absolute_date(date_str: &str) -> Option<NaiveDateTime> {
        return match date_str.parse::<NaiveDateTime>() {
            Ok(date) => Some(date),
            Err(_) => None,
        };
    }

    fn parse_relative_date(date_str: &str) -> Option<NaiveDateTime> {
        if date_str.len() < 2 {
            return None;
        }

        let (amount, unit): (&str, &str) = date_str.split_at(date_str.len() - 1);
        let amount: i64 = amount.parse().ok()?;

        return Some(
            Local::now().naive_local()
                + match unit {
                    "s" => Duration::seconds(amount),
                    "m" => Duration::minutes(amount),
                    "h" => Duration::hours(amount),
                    "d" => Duration::days(amount),
                    "w" => Duration::weeks(amount),
                    "M" => Duration::days(amount * 30),
                    "y" => Duration::days(amount * 365),
                    _ => return None,
                },
        );
    }

    fn parse_end_of_date(date_str: &str) -> Option<NaiveDateTime> {
        return match date_str {
            "eod" => Local::now().naive_local().date().and_hms_opt(23, 59, 59),
            "eow" => {
                let mut now = Local::now().naive_local().date();
                let days_to_end_of_week = 7 - now.weekday().number_from_monday();
                now += Duration::days(days_to_end_of_week as i64);
                now.and_hms_opt(23, 59, 59)
            }
            "eom" => {
                let mut now = Local::now().naive_local().date();
                let days_in_month: usize = match now.month() {
                    1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                    4 | 6 | 9 | 11 => 30,
                    2 => {
                        if now.year() % 4 == 0 {
                            29
                        } else {
                            28
                        }
                    }
                    _ => 0,
                };
                let days_to_end_of_month = days_in_month - now.day() as usize;
                now += Duration::days(days_to_end_of_month as i64);
                now.and_hms_opt(23, 59, 59)
            }
            "eoy" => {
                let mut now = Local::now().naive_local().date();
                let days_to_end_of_year =
                    if now.year() % 4 == 0 { 366 } else { 365 } - now.ordinal();
                now += Duration::days(days_to_end_of_year as i64);
                now.and_hms_opt(23, 59, 59)
            }
            _ => None,
        };
    }
}
