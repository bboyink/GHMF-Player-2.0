use super::theme;
use crate::plc::PlcStatus;
use egui::{Ui, RichText, Color32};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusType {
    Info,
    Success,
    Warning,
    Error,
}

pub fn show(ui: &mut Ui, message: &str, status_type: StatusType, status_time: Instant, dmx_connected: bool, plc_status: &PlcStatus, use_rgbw: bool) {
    ui.horizontal(|ui| {
        // Status icon and message
        let (icon, color) = match status_type {
            StatusType::Info => ("ℹ", theme::AppColors::INFO),
            StatusType::Success => ("✓", theme::AppColors::SUCCESS),
            StatusType::Warning => ("⚠", theme::AppColors::WARNING),
            StatusType::Error => ("✗", theme::AppColors::ERROR),
        };
        
        ui.label(RichText::new(icon).color(color).size(14.0));
        ui.label(RichText::new(message).color(theme::AppColors::TEXT_SECONDARY).size(12.0));
        
        ui.add_space(20.0);
        
        // DMX Status
        let dmx_color = if dmx_connected {
            theme::AppColors::SUCCESS
        } else {
            theme::AppColors::TEXT_DISABLED
        };
        ui.label(RichText::new("DMX:").color(theme::AppColors::TEXT_SECONDARY).size(11.0));
        ui.label(RichText::new(if dmx_connected { "Connected" } else { "Disconnected" })
            .color(dmx_color)
            .size(11.0));
        ui.label(RichText::new(format!("Mode: {}", if use_rgbw { "RGBW" } else { "RGB" }))
            .color(theme::AppColors::TEXT_DISABLED)
            .size(11.0));
        
        ui.add_space(15.0);
        
        // PLC Status
        let (plc_text, plc_color) = match plc_status {
            PlcStatus::Disabled => ("Disabled", theme::AppColors::TEXT_DISABLED),
            PlcStatus::Connected => ("Connected", theme::AppColors::SUCCESS),
            PlcStatus::Disconnected => ("Disconnected", Color32::from_rgb(180, 180, 180)),
            PlcStatus::Reconnecting => ("Reconnecting", theme::AppColors::WARNING),
        };
        ui.label(RichText::new("PLC:").color(theme::AppColors::TEXT_SECONDARY).size(11.0));
        ui.label(RichText::new(plc_text).color(plc_color).size(11.0));
        
        // Time since status update
        let elapsed = status_time.elapsed().as_secs();
        if elapsed < 60 {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(RichText::new(format!("{}s ago", elapsed))
                    .color(theme::AppColors::TEXT_DISABLED)
                    .size(11.0));
            });
        }
    });
}
