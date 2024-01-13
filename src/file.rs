use std::fs;

use crate::error::Result;

pub struct FileUtils;

pub trait JsonSerializer {
    fn to_json(&self) -> Result<String>;
    fn from_json(json: &str) -> Result<Self>
    where
        Self: Sized;
}

impl FileUtils {
    pub fn load<T>(path: &str) -> Result<T>
    where
        T: JsonSerializer,
    {
        // TODO handle create file if not exist

        let json: String = fs::read_to_string(path)?;
        let data: T = T::from_json(&json)?;

        return Ok(data);
    }

    pub fn save<T>(path: &str, data: &T) -> Result<()>
    where
        T: JsonSerializer,
    {
        let json: String = data.to_json()?;
        fs::write(path, json)?;

        return Ok(());
    }
}
