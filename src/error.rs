use std::{convert, io, result};

pub struct Error(ErrorKind);

pub enum ErrorKind {
    Json,
    Io,
}

pub type Result<T> = result::Result<T, Error>;

impl<T> convert::From<io::Result<T>> for Result<T> {
    fn from(value: io::Result<T>) -> Self {
        return match value {
            Ok(v) => Ok(v),
            io::Error => Error(ErrorKind::Io),
        };
    }
}
