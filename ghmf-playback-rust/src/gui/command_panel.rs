use super::theme;
use egui::{Ui, RichText, ScrollArea, Color32};

pub fn show(
    ui: &mut Ui,
    recent_commands: &[(u64, String)],
    playback_position_ms: u64,
    ctl_loaded: bool,
) {
    ui.heading(RichText::new("Command Output").size(18.0).color(Color32::BLACK));
    ui.add_space(10.0);
    
    if !ctl_loaded {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new("No CTL file loaded").size(14.0).color(Color32::DARK_GRAY));
            ui.add_space(10.0);
            ui.label(RichText::new("Open a song to see command output").color(Color32::GRAY));
        });
        return;
    }
    
    if recent_commands.is_empty() {
        ui.vertical_centered(|ui| {
            ui.add_space(40.0);
            ui.label(RichText::new("Waiting for playback...").size(14.0).color(Color32::DARK_GRAY));
        });
        return;
    }
    
    // Separate commands into Water and Lights categories
    let (water_commands, light_commands): (Vec<_>, Vec<_>) = recent_commands.iter()
        .partition(|(_, cmd)| is_water_command(cmd));
    
    // Water section
    ui.group(|ui| {
        ui.heading(RichText::new("💧 Water").size(16.0).color(Color32::from_rgb(30, 144, 255)));
        ui.add_space(5.0);
        
        if water_commands.is_empty() {
            ui.label(RichText::new("No water commands yet").color(Color32::GRAY).italics());
        } else {
            ScrollArea::vertical()
                .id_source("water_scroll_area")
                .max_height(200.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Show newest first (no reverse needed since we want newest at top)
                    for (time_ms, cmd_desc) in water_commands.iter().rev() {
                        show_command_line(ui, *time_ms, cmd_desc, playback_position_ms);
                    }
                });
        }
    });
    
    ui.add_space(10.0);
    
    // Lights section
    ui.group(|ui| {
        ui.heading(RichText::new("💡 Lights").size(16.0).color(Color32::from_rgb(255, 215, 0)));
        ui.add_space(5.0);
        
        if light_commands.is_empty() {
            ui.label(RichText::new("No light commands yet").color(Color32::GRAY).italics());
        } else {
            ScrollArea::vertical()
                .id_source("lights_scroll_area")
                .max_height(200.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Show newest first
                    for (time_ms, cmd_desc) in light_commands.iter().rev() {
                        show_command_line(ui, *time_ms, cmd_desc, playback_position_ms);
                    }
                });
        }
    });
}

fn show_command_line(ui: &mut Ui, time_ms: u64, cmd_desc: &str, playback_position_ms: u64) {
    let age_ms = playback_position_ms.saturating_sub(time_ms);
    let is_recent = age_ms < 1000; // Highlight if within last second
    
    ui.horizontal(|ui| {
        // Timestamp
        let time_color = if is_recent { 
            Color32::from_rgb(0, 128, 0)  // Green for recent
        } else { 
            Color32::DARK_GRAY 
        };
        
        ui.label(RichText::new(format_time(time_ms))
            .size(12.0)
            .color(time_color));
        
        ui.add_space(10.0);
        
        // Command description - always black
        let cmd_text = RichText::new(cmd_desc)
            .size(13.0)
            .color(Color32::BLACK);
        
        if is_recent {
            ui.label(cmd_text.strong());
        } else {
            ui.label(cmd_text);
        }
    });
    
    ui.add_space(3.0);
}

fn is_water_command(cmd_desc: &str) -> bool {
    // Check if command is water-related based on GHMF documentation
    // Water FCW addresses:
    // 1-13: Module water rings (W1-W8) and WAVE water controls
    // 217-255: Module WAVE Sols, Left/Right/Odd/Even/ALL WAVE Sols
    // 700-896: WAVE Sols 0-196
    // Parse "XXX-YYY" format
    if let Some(dash_pos) = cmd_desc.find('-') {
        if let Ok(fcw_num) = cmd_desc[..dash_pos].trim().parse::<u16>() {
            return (fcw_num >= 1 && fcw_num <= 13) ||
                   (fcw_num >= 217 && fcw_num <= 255) ||
                   (fcw_num >= 700 && fcw_num <= 896);
        }
    }
    false
}

fn format_time(ms: u64) -> String {
    let total_secs = ms / 1000;
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    let millis = (ms % 1000) / 100;
    
    format!("{:02}:{:02}.{}", minutes, seconds, millis)
}
