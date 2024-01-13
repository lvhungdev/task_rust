use std::{convert, fmt, io, result};

#[derive(Debug)]
pub struct Error(pub ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    Json(String),
    Io(String),
    Input(String),
}

pub type Result<T> = result::Result<T, Error>;

impl convert::From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        return Error(ErrorKind::Io(value.kind().to_string()));
    }
}

impl convert::From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        return Error(ErrorKind::Json(value.to_string()));
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let kind: &ErrorKind = &self.0;

        let msg: String = match kind {
            ErrorKind::Json(err_msg) => format!("[ERR.JSON] {}", err_msg),
            ErrorKind::Io(err_msg) => format!("[ERR.IO] {}", err_msg),
            ErrorKind::Input(err_msg) => format!("[ERR.INPUT] {}", err_msg),
        };

        return write!(f, "{}", msg);
    }
}
