mod csv_config;

pub use csv_config::{
    CsvConfig, ColorDefinition, FixtureDefinition, FixtureFormat,
    FcwMapping, FcwDirective,
};

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub audio_device_id: Option<String>,
    pub audio_latency_ms: u32,
    pub dmx_enabled: bool,
    #[serde(default = "default_use_rgbw")]
    pub use_rgbw: bool,
    pub plc_enabled: bool,
    pub plc_ip_address: String,
    pub plc_port: u16,
    pub plc_port_name: Option<String>,
    pub last_playlist: Option<String>,
    #[serde(default = "default_production_folder")]
    pub production_folder: String,
    #[serde(default = "default_testing_folder")]
    pub testing_folder: String,
    #[serde(default = "default_events_folder")]
    pub events_folder: String,
    #[serde(default = "default_drone_folder")]
    pub drone_folder: String,
    #[serde(default = "default_open_close_folder")]
    pub open_close_folder: String,
    #[serde(default = "default_playlist_folder")]
    pub playlist_folder: String,
}

fn default_use_rgbw() -> bool {
    true
}

fn default_production_folder() -> String {
    "~/Production".to_string()
}

fn default_testing_folder() -> String {
    "~/Testing".to_string()
}

fn default_events_folder() -> String {
    "~/Events".to_string()
}

fn default_drone_folder() -> String {
    "~/Drone".to_string()
}

fn default_open_close_folder() -> String {
    "~/Open-Close".to_string()
}

fn default_playlist_folder() -> String {
    "~/Playlists".to_string()
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            audio_device_id: None,
            audio_latency_ms: 100,
            dmx_enabled: true,
            use_rgbw: true,
            plc_enabled: false,
            plc_ip_address: "192.168.1.10".to_string(),
            plc_port: 444,
            plc_port_name: None,
            last_playlist: None,
            production_folder: default_production_folder(),
            testing_folder: default_testing_folder(),
            events_folder: default_events_folder(),
            drone_folder: default_drone_folder(),
            open_close_folder: default_open_close_folder(),
            playlist_folder: default_playlist_folder(),
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = Self::config_path();
        
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match toml::from_str(&content) {
                        Ok(settings) => return settings,
                        Err(e) => eprintln!("Failed to parse settings: {}", e),
                    }
                }
                Err(e) => eprintln!("Failed to read settings: {}", e),
            }
        }

        Self::default()
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        
        Ok(())
    }

    fn config_path() -> PathBuf {
        // Use a platform-appropriate config directory
        if cfg!(target_os = "windows") {
            PathBuf::from("C:\\ghmf\\config\\playback.toml")
        } else if cfg!(target_os = "macos") {
            std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("Library/Application Support/GHMF/playback.toml")
        } else {
            std::env::var("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(".config/ghmf/playback.toml")
        }
    }
}

// Optional: Add dirs crate for cross-platform directory paths
// Add to Cargo.toml: dirs = "5.0"
