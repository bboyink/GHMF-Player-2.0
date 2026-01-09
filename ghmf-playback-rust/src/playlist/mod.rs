use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlaylistError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

const SONG_COMMENT: &str = "(COMMENT)";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub songs: Vec<String>,
    pub start_index: usize,
}

impl Playlist {
    pub fn load(path: &str) -> Result<Self, PlaylistError> {
        if !Path::new(path).exists() {
            return Err(PlaylistError::FileNotFound(path.to_string()));
        }

        let name = Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unnamed")
            .to_string();

        let content = fs::read_to_string(path)?;
        let mut songs = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let decoded = Self::decode_entry(line)?;
            songs.push(decoded);
        }

        Ok(Self {
            name,
            songs,
            start_index: 0,
        })
    }

    fn decode_entry(entry: &str) -> Result<String, PlaylistError> {
        // If it's a comment or already a valid path, return as-is
        if entry.contains(SONG_COMMENT) || Path::new(entry).exists() {
            return Ok(entry.to_string());
        }

        // TODO: Implement XOR decoding for encrypted playlists
        // For now, just return the entry as-is
        Ok(entry.to_string())
    }

    pub fn is_comment(song: &str) -> bool {
        song.contains(SONG_COMMENT)
    }

    pub fn next_song(&mut self) -> Option<&str> {
        if self.start_index >= self.songs.len() {
            return None;
        }

        let song = &self.songs[self.start_index];
        self.start_index += 1;
        Some(song)
    }

    pub fn reset(&mut self) {
        self.start_index = 0;
    }
}
