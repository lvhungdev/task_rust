use std::fs;

use crate::error::Result;

pub struct FileUtils;

pub trait JsonSerializer {
    fn to_json(&self) -> String;
    fn from_json(json: &str) -> Self;
}

impl FileUtils {
    pub fn load<T>(path: &str) -> Result<T>
    where
        T: JsonSerializer,
    {
        // TODO handle errors
        // TODO handle create file if not exist

        let json: String = fs::read_to_string(path)?;

        return Ok(T::from_json(&json));
    }

    pub fn save<T>(path: &str, data: &T)
    where
        T: JsonSerializer,
    {
        // TODO handle errors
        let _ = fs::write(path, data.to_json());
    }
}
