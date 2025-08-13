use crate::models::{Contestant, Property};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub contestants: Vec<Contestant>,
    pub properties: Vec<Property>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            contestants: Vec::new(),
            properties: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> Self {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".scord");
        fs::create_dir_all(&path).ok();
        path.push("data.json");
        
        Self { file_path: path }
    }



    pub fn load(&self) -> Result<AppData> {
        if !self.file_path.exists() {
            return Ok(AppData::default());
        }

        let content = fs::read_to_string(&self.file_path)
            .with_context(|| format!("Failed to read file: {:?}", self.file_path))?;

        let data: AppData = serde_json::from_str(&content)
            .with_context(|| "Failed to parse JSON data")?;

        Ok(data)
    }

    pub fn save(&self, data: &AppData) -> Result<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {:?}", parent))?;
        }

        let content = serde_json::to_string_pretty(data)
            .with_context(|| "Failed to serialize data")?;

        fs::write(&self.file_path, content)
            .with_context(|| format!("Failed to write file: {:?}", self.file_path))?;

        Ok(())
    }


}