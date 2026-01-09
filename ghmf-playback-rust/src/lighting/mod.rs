pub mod color;
pub mod channel;
pub mod fixture_manager;

pub use fixture_manager::FixtureManager;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LightingError {
    #[error("Invalid channel: {0}")]
    InvalidChannel(usize),
    
    #[error("Invalid light index: {0}")]
    InvalidLight(usize),
}

// TODO: Implement full lighting system
// This will include:
// - Light fixtures with channel mappings
// - Color management and fading
// - Light grouping/modules
// - Effect generation
