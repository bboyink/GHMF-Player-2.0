use egui::{Button, ComboBox, Frame, Layout, RichText, ScrollArea, TextEdit, Vec2};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::gui::theme::AppColors;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureEntry {
    pub name: String,
    pub minutes_before: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProceduresConfig {
    pub entries: Vec<ProcedureEntry>,
}

impl Default for ProceduresConfig {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

pub struct ProceduresPanel {
    config: ProceduresConfig,
    config_path: PathBuf,
    
    // UI state
    procedure_name: String,
    selected_minutes: u32,
}

impl Default for ProceduresPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ProceduresPanel {
    pub fn new() -> Self {
        let config_path = PathBuf::from("Config/procedures.json");
        let config = Self::load_config(&config_path);
        
        Self {
            config,
            config_path,
            procedure_name: String::new(),
            selected_minutes: 5, // Default to 5 minutes
        }
    }
    
    /// Get all procedures sorted by minutes_before
    pub fn get_procedures(&self) -> Vec<ProcedureEntry> {
        let mut procedures = self.config.entries.clone();
        procedures.sort_by_key(|p| p.minutes_before);
        procedures
    }
    
    fn load_config(path: &PathBuf) -> ProceduresConfig {
        if path.exists() {
            if let Ok(contents) = fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&contents) {
                    return config;
                }
            }
        }
        ProceduresConfig::default()
    }
    
    fn save_config(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.config) {
            let _ = fs::write(&self.config_path, json);
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            
            ui.vertical(|ui| {
                ui.add_space(20.0);
                
                // Main heading
                ui.label(
                    RichText::new("Procedures Configuration")
                        .size(24.0)
                        .strong()
                        .color(AppColors::CYAN),
                );
                
                ui.add_space(20.0);
                
                ui.horizontal(|ui| {
                    // Left column: Input form
                    Frame::none()
                        .fill(AppColors::SURFACE)
                        .stroke(egui::Stroke::new(1.0, AppColors::SURFACE_LIGHT))
                        .rounding(12.0)
                        .inner_margin(16.0)
                        .show(ui, |ui| {
                            ui.set_max_width(350.0);
                            
                            ui.vertical(|ui| {
                                // Section header
                                ui.label(
                                    RichText::new("Add Procedure")
                                        .size(18.0)
                                        .strong()
                                        .color(AppColors::CYAN),
                                );
                                
                                ui.add_space(16.0);
                                
                                // Procedure Name
                                ui.label(
                                    RichText::new("Procedure Name")
                                        .size(16.0)
                                        .color(AppColors::TEXT_PRIMARY),
                                );
                                ui.add_space(4.0);
                                
                                let name_edit = TextEdit::singleline(&mut self.procedure_name)
                                    .desired_width(318.0)
                                    .font(egui::TextStyle::Heading)
                                    .hint_text("Enter procedure name...");
                                ui.add_sized(Vec2::new(318.0, 36.0), name_edit);
                                
                                ui.add_space(16.0);
                                
                                // Minutes Before Show
                                ui.label(
                                    RichText::new("Minutes Before Show Start")
                                        .size(16.0)
                                        .color(AppColors::TEXT_PRIMARY),
                                );
                                ui.add_space(4.0);
                                
                                ComboBox::from_id_source("minutes_selector")
                                    .selected_text(format!("{} minutes", self.selected_minutes))
                                    .width(318.0)
                                    .show_ui(ui, |ui| {
                                        for minutes in 1..=20 {
                                            ui.selectable_value(
                                                &mut self.selected_minutes,
                                                minutes,
                                                format!("{} minutes", minutes),
                                            );
                                        }
                                    });
                                
                                ui.add_space(20.0);
                                
                                // Add Button
                                let add_enabled = !self.procedure_name.trim().is_empty();
                                
                                ui.add_enabled_ui(add_enabled, |ui| {
                                    let button = Button::new(
                                        RichText::new("Add Procedure")
                                            .size(14.0)
                                            .color(egui::Color32::WHITE),
                                    )
                                    .fill(if add_enabled {
                                        AppColors::CYAN
                                    } else {
                                        AppColors::SURFACE_LIGHT
                                    })
                                    .rounding(8.0)
                                    .min_size(Vec2::new(318.0, 36.0));
                                    
                                    if ui.add(button).clicked() && add_enabled {
                                        self.add_procedure();
                                    }
                                });
                            });
                        });
                    
                    ui.add_space(20.0);
                    
                    // Right column: Configured procedures list
                    ui.vertical(|ui| {
                        ui.set_min_width(500.0);
                        ui.set_min_height(600.0);
                        
                        Frame::none()
                            .fill(AppColors::SURFACE)
                            .stroke(egui::Stroke::new(1.0, AppColors::SURFACE_LIGHT))
                            .rounding(12.0)
                            .inner_margin(16.0)
                            .show(ui, |ui| {
                                ui.vertical(|ui| {
                                    // Section header
                                    ui.label(
                                        RichText::new("Configured Procedures")
                                            .size(18.0)
                                            .strong()
                                            .color(AppColors::CYAN),
                                    );
                                    
                                    ui.add_space(12.0);
                                    
                                    // Procedures list
                                    ScrollArea::vertical()
                                        .min_scrolled_height(550.0)
                                        .show(ui, |ui| {
                                        if self.config.entries.is_empty() {
                                            ui.add_space(20.0);
                                            ui.label(
                                                RichText::new("No procedures configured yet")
                                                    .size(14.0)
                                                    .color(AppColors::TEXT_SECONDARY),
                                            );
                                        } else {
                                            let mut to_remove: Option<usize> = None;
                                            
                                            for (idx, entry) in self.config.entries.iter().enumerate() {
                                                ui.add_space(8.0);
                                                
                                                Frame::none()
                                                    .fill(AppColors::SURFACE_LIGHT)
                                                    .stroke(egui::Stroke::new(1.0, AppColors::SURFACE))
                                                    .rounding(8.0)
                                                    .inner_margin(12.0)
                                                    .show(ui, |ui| {
                                                        ui.horizontal(|ui| {
                                                            ui.vertical(|ui| {
                                                                // Procedure name
                                                                ui.label(
                                                                    RichText::new(&entry.name)
                                                                        .size(14.0)
                                                                        .strong()
                                                                        .color(AppColors::TEXT_PRIMARY),
                                                                );
                                                                
                                                                ui.add_space(2.0);
                                                                
                                                                // Time before show
                                                                ui.label(
                                                                    RichText::new(format!(
                                                                        "⏰ {} minutes before show",
                                                                        entry.minutes_before
                                                                    ))
                                                                    .size(16.0)
                                                                    .color(AppColors::TEXT_SECONDARY),
                                                                );
                                                            });
                                                            
                                                            ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                                                                let delete_button = Button::new(
                                                                    RichText::new("🗑")
                                                                        .size(18.0)
                                                                        .color(egui::Color32::WHITE),
                                                                )
                                                                .fill(AppColors::ERROR)
                                                                .rounding(4.0)
                                                                .min_size(Vec2::new(24.0, 24.0));
                                                                
                                                                if ui.add(delete_button).clicked() {
                                                                    to_remove = Some(idx);
                                                                }
                                                            });
                                                        });
                                                    });
                                            }
                                            
                                            // Remove entry if delete was clicked
                                            if let Some(idx) = to_remove {
                                                self.config.entries.remove(idx);
                                                self.save_config();
                                            }
                                        }
                                    });
                                });
                            });
                    });
                });
            });
        });
    }
    
    fn add_procedure(&mut self) {
        let name = self.procedure_name.trim().to_string();
        if name.is_empty() {
            return;
        }
        
        let entry = ProcedureEntry {
            name,
            minutes_before: self.selected_minutes,
        };
        
        self.config.entries.push(entry);
        self.save_config();
        
        // Clear the input fields
        self.procedure_name.clear();
        self.selected_minutes = 5; // Reset to default
    }
}
