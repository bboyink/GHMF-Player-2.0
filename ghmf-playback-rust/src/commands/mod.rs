mod command;
mod command_file;
mod executor;
mod ctl_file;

pub use command::{Command, CommandLine};
pub use command_file::CommandFile;
pub use executor::CommandExecutor;
pub use ctl_file::{CtlFile, CtlCommand, CtlLine};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommandError {
    #[error("Invalid command format: {0}")]
    ParseError(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
