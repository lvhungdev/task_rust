use std::fs;

use directories::BaseDirs;

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
        if fs::metadata(path).is_err() {
            fs::File::create(path)?;
        }

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

    pub fn get_standard_local_data_path() -> Option<String> {
        return match BaseDirs::new() {
            Some(base_dir) => match base_dir.data_local_dir().to_str() {
                Some(path) => Some(path.to_string()),
                None => None,
            },
            None => None,
        };
    }
}
