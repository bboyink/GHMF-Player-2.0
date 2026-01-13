mod app;
mod playback_panel;
mod lighting_panel;
mod status_panel;
mod settings_dialog;
mod command_panel;
mod theme;
mod sidebar;
mod dmx_map_panel;
mod light_group_panel;
mod legacy_color_panel;
mod playlist_panel;
mod start_time_panel;
mod procedures_panel;
mod operator_panel;

pub use app::PlaybackApp;
pub use sidebar::{AppView, Sidebar};
pub use operator_panel::OperatorPanel;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GuiError {
    #[error("Failed to initialize GUI: {0}")]
    InitError(String),
    
    #[error("Rendering error: {0}")]
    RenderError(String),
}
