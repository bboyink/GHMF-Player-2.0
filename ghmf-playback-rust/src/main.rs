mod audio;
mod dmx;
mod plc;
mod lighting;
mod commands;
mod playlist;
mod config;
mod utils;
mod gui;

use anyhow::Result;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("ghmf_playback=debug,info")
        .init();

    info!("Fountain Director v1.0");
    info!("Starting application...");

    // Launch GUI
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1290.0, 975.0])
            .with_min_inner_size([800.0, 600.0])
            .with_title("Fountain Director v1.0"),
        ..Default::default()
    };

    let result = eframe::run_native(
        "Fountain Director",
        native_options,
        Box::new(|cc| Ok(Box::new(gui::PlaybackApp::new(cc)))),
    );

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to run application: {}", e);
            std::process::exit(1);
        }
    }
}



