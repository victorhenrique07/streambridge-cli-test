use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::errors::Errors;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Watermark {
    pub issue_id: u64,
    pub issue_updated_at: DateTime<Utc>,
}

pub fn create_watermark(issue: Watermark) -> Result<(), Box<dyn Error>> {
    let watermark = std::path::PathBuf::from("watermark");

    let watermark_path = watermark.join("watermark.json");

    fs::create_dir_all(&watermark)?;

    let mut file = File::create(watermark_path.clone())?;

    let data = serde_json::to_vec(&issue)?;
    file.write_all(&data)?;

    Ok(())
}

pub fn check_watermark() -> Option<Watermark> {
    let watermark = std::path::PathBuf::from("watermark");

    let watermark_path = watermark.join("watermark.json");

    if !watermark_path.exists() {
        return None;
    }

    let mut file = match File::open(&watermark_path) {
        Ok(value) => value,
        Err(error) => panic!(
            "{}",
            Errors::OpenFileError(String::from("watermark.json"), error)
        ),
    };

    let mut content = String::new();
    file.read_to_string(&mut content).ok()?;

    let data: Watermark = match serde_json::from_str(&content) {
        Ok(value) => value,
        Err(_) => panic!("{}", &Errors::SerializingError()),
    };

    Some(data)
}
