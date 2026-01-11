mod player;
mod decoder;

pub use player::AudioPlayer;
pub use decoder::AudioDecoder;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudioError {
    #[error("Failed to initialize audio device: {0}")]
    DeviceError(String),
    
    #[error("Failed to decode audio file: {0}")]
    DecodeError(String),
    
    #[error("Decoder error: {0}")]
    DecoderError(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
