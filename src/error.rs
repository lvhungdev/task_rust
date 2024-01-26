use std::{convert, fmt, result};

#[derive(Debug)]
pub struct Error(pub ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    Io(String),
    Input(String),
}

pub type Result<T> = result::Result<T, Error>;

impl convert::From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        // TODO Implement better error handling
        return Error(ErrorKind::Io(format!("{}", err)));
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind: &ErrorKind = &self.0;

        let msg: String = match kind {
            ErrorKind::Io(err_msg) => format!("[ERR.IO] {}", err_msg),
            ErrorKind::Input(err_msg) => format!("[ERR.INPUT] {}", err_msg),
        };

        return write!(f, "{}", msg);
    }
}
