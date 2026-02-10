use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;
use std::fs;

use crate::models::Fish;

/// Water quality parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterParams {
    pub purity: f32,       // 0.0 - 100.0 (General cleanliness)
    pub ph: f32,           // 0.0 - 14.0 (Acidity/Alkalinity, Ideal: 7.0)
    pub temperature: f32,  // Celsius (Ideal: 24-26)
}

impl Default for WaterParams {
    fn default() -> Self {
        Self {
            purity: 100.0,
            ph: 7.0,
            temperature: 25.0,
        }
    }
}

/// Main save data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: String,
    pub last_saved: DateTime<Utc>,
    pub fish: Vec<Fish>,
    pub player_name: String,
    #[serde(default)] // Use default if missing (backward compatibility)
    pub water: WaterParams,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            last_saved: Utc::now(),
            fish: Vec::new(),
            player_name: "Player".to_string(),
            water: WaterParams::default(),
        }
    }
}

impl SaveData {
    /// Get the save file path
    pub fn get_save_path() -> Result<PathBuf> {
        let config_dir = directories::ProjectDirs::from("", "", "fishtank")
            .map(|dirs| dirs.config_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from(".fishtank"));

        fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("save.json"))
    }

    /// Load save data from disk
    pub fn load() -> Result<Self> {
        let path = Self::get_save_path()?;
        
        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        
        // Try to load save file, if it fails (old format), backup and start fresh
        match serde_json::from_str::<SaveData>(&content) {
            Ok(save) => Ok(save),
            Err(_e) => {
                // Backup old save file
                let backup_path = path.with_extension("json.backup");
                let _ = fs::rename(&path, backup_path);
                
                // Return fresh save
                Ok(Self::default())
            }
        }
    }

    /// Save data to disk
    pub fn save(&mut self) -> Result<()> {
        self.last_saved = Utc::now();
        let path = Self::get_save_path()?;
        
        // Atomic write: write to temp file, then rename
        let temp_path = path.with_extension("tmp");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&temp_path, content)?;
        fs::rename(temp_path, path)?;
        
        Ok(())
    }

    /// Calculate elapsed time since last save
    pub fn time_since_last_save(&self) -> chrono::Duration {
        Utc::now().signed_duration_since(self.last_saved)
    }
}
