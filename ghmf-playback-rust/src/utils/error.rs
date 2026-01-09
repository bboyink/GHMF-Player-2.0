use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Audio error: {0}")]
    Audio(#[from] crate::audio::AudioError),
    
    #[error("DMX error: {0}")]
    Dmx(#[from] crate::dmx::DmxError),
    
    #[error("Command error: {0}")]
    Command(#[from] crate::commands::CommandError),
    
    #[error("Lighting error: {0}")]
    Lighting(#[from] crate::lighting::LightingError),
    
    #[error("Generic error: {0}")]
    Generic(String),
}
