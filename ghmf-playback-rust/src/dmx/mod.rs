mod enttec;
mod universe;

pub use enttec::EnttecDmxPro;
pub use universe::DmxUniverse;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DmxError {
    #[error("DMX device not found")]
    DeviceNotFound,
    
    #[error("Failed to open serial port: {0}")]
    SerialError(#[from] serialport::Error),
    
    #[error("Invalid channel number: {0}")]
    InvalidChannel(usize),
    
    #[error("Communication error: {0}")]
    CommError(String),
}
