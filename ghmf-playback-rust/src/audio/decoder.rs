use super::AudioError;
use std::path::Path;

pub struct AudioDecoder;

impl AudioDecoder {
    pub fn get_duration(path: &str) -> Result<std::time::Duration, AudioError> {
        if !Path::new(path).exists() {
            return Err(AudioError::FileNotFound(path.to_string()));
        }

        // TODO: Implement actual duration detection
        // For now, return a placeholder
        Ok(std::time::Duration::from_secs(0))
    }

    pub fn is_supported(path: &str) -> bool {
        let path = Path::new(path);
        if let Some(ext) = path.extension() {
            let ext = ext.to_string_lossy().to_lowercase();
            matches!(ext.as_str(), "wav" | "mp3" | "flac" | "ogg")
        } else {
            false
        }
    }
}
