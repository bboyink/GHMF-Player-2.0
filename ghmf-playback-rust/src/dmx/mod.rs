mod enttec;
mod universe;
mod sacn_output;

pub use enttec::EnttecDmxPro;
pub use universe::DmxUniverse;
pub use sacn_output::{SacnOutput, SacnFilterMode, get_network_interfaces};

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
