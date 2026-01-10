use egui::{Context, Ui, Color32, Vec2, Sense};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyColor {
    pub index: u8,
    pub name: String,
    pub hex: String,
}

impl LegacyColor {
    fn new(index: u8, name: &str, hex: &str) -> Self {
        Self {
            index,
            name: name.to_string(),
            hex: hex.to_string(),
        }
    }

    fn to_color32(&self) -> Color32 {
        if let Ok(rgb) = u32::from_str_radix(&self.hex, 16) {
            let r = ((rgb >> 16) & 0xFF) as u8;
            let g = ((rgb >> 8) & 0xFF) as u8;
            let b = (rgb & 0xFF) as u8;
            Color32::from_rgb(r, g, b)
        } else {
            Color32::BLACK
        }
    }
}

pub struct LegacyColorPanel {
    colors: Vec<LegacyColor>,
    editing_index: Option<usize>,
    temp_hex: String,
    temp_name: String,
}

impl Default for LegacyColorPanel {
    fn default() -> Self {
        let mut panel = Self {
            colors: Vec::new(),
            editing_index: None,
            temp_hex: String::new(),
            temp_name: String::new(),
        };
        panel.load_colors();
        panel
    }
}

impl LegacyColorPanel {
    fn load_colors(&mut self) {
        let config_path = Path::new("Config/legacy_colors.json");
        
        if config_path.exists() {
            if let Ok(data) = std::fs::read_to_string(config_path) {
                if let Ok(colors) = serde_json::from_str(&data) {
                    self.colors = colors;
                    return;
                }
            }
        }
        
        // Default colors if file doesn't exist
        self.colors = vec![
            LegacyColor::new(0, "Off", "000000"),
            LegacyColor::new(1, "Red", "FF0000"),
            LegacyColor::new(2, "Blue", "0000FF"),
            LegacyColor::new(3, "Violet", "BA00FF"),
            LegacyColor::new(4, "Yellow", "FFE501"),
            LegacyColor::new(5, "Orange", "FF9001"),
            LegacyColor::new(6, "Green", "00FF00"),
            LegacyColor::new(7, "Magenta", "FF0A52"),
            LegacyColor::new(8, "White", "FFFFFF"),
            LegacyColor::new(9, "Pink", "FF3080"),
            LegacyColor::new(10, "Cyan", "33FFFF"),
            LegacyColor::new(11, "Light Violet", "9999FF"),
            LegacyColor::new(12, "Spring Green", "00EE76"),
            LegacyColor::new(13, "Light Orange", "FFC200"),
            LegacyColor::new(14, "Light Green", "4BFF3D"),
            LegacyColor::new(15, "Incandescent", "D2FFDB"),
        ];
        
        self.save_colors();
    }

    fn save_colors(&self) {
        let config_path = Path::new("Config/legacy_colors.json");
        if let Ok(data) = serde_json::to_string_pretty(&self.colors) {
            let _ = std::fs::write(config_path, data);
        }
    }

    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        ui.heading(egui::RichText::new("Legacy Color Mapping").color(Color32::WHITE).size(24.0));
        ui.add_space(10.0);
        ui.label("Map FWC values to RGB hex colors for legacy color commands");
        ui.add_space(20.0);

        egui::ScrollArea::vertical()
            .id_salt("legacy_color_scroll")
            .show(ui, |ui| {
                // Header
                egui::Grid::new("legacy_color_grid")
                    .striped(true)
                    .spacing([20.0, 8.0])
                    .min_col_width(80.0)
                    .show(ui, |ui| {
                        // Header row
                        ui.label(egui::RichText::new("Preview").strong().size(16.0).color(Color32::WHITE));
                        ui.label(egui::RichText::new("Index").strong().size(16.0).color(Color32::WHITE));
                        ui.label(egui::RichText::new("Color Name").strong().size(16.0).color(Color32::WHITE));
                        ui.label(egui::RichText::new("RGB Hex").strong().size(16.0).color(Color32::WHITE));
                        ui.end_row();

                        // Data rows
                        for i in 0..self.colors.len() {
                            let is_editing = self.editing_index == Some(i);
                            
                            // Clone the color data we need for display
                            let color_rgb = self.colors[i].to_color32();
                            let color_index = self.colors[i].index;
                            let color_name = self.colors[i].name.clone();
                            let color_hex = self.colors[i].hex.clone();

                            // Preview square
                            let (rect, _) = ui.allocate_exact_size(
                                Vec2::new(18.0, 18.0),
                                Sense::hover()
                            );
                            ui.painter().rect_filled(
                                rect,
                                4.0,
                                color_rgb
                            );
                            ui.painter().rect_stroke(
                                rect,
                                4.0,
                                egui::Stroke::new(1.0, Color32::GRAY)
                            );

                            // Index
                            ui.label(egui::RichText::new(format!("{}", color_index)).size(15.0));

                            // Color Name - editable
                            if is_editing {
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut self.temp_name)
                                        .desired_width(150.0)
                                        .font(egui::TextStyle::Body)
                                );
                                if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    self.colors[i].name = self.temp_name.clone();
                                    self.editing_index = None;
                                    self.save_colors();
                                }
                            } else {
                                let response = ui.add(
                                    egui::Label::new(egui::RichText::new(&color_name).size(15.0))
                                        .sense(Sense::click())
                                );
                                if response.clicked() {
                                    self.editing_index = Some(i);
                                    self.temp_name = color_name.clone();
                                }
                            }

                            // RGB Hex - editable
                            if is_editing {
                                ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("#").size(15.0));
                                    let response = ui.add(
                                        egui::TextEdit::singleline(&mut self.temp_hex)
                                            .desired_width(80.0)
                                            .char_limit(6)
                                            .font(egui::TextStyle::Body)
                                    );
                                    if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        // Validate hex
                                        let cleaned = self.temp_hex.to_uppercase().chars()
                                            .filter(|c| c.is_ascii_hexdigit())
                                            .collect::<String>();
                                        if cleaned.len() == 6 {
                                            self.colors[i].hex = cleaned;
                                            self.save_colors();
                                        }
                                        self.editing_index = None;
                                    }
                                });
                            } else {
                                let response = ui.add(
                                    egui::Label::new(egui::RichText::new(format!("#{}", color_hex)).size(15.0))
                                        .sense(Sense::click())
                                );
                                if response.clicked() {
                                    self.editing_index = Some(i);
                                    self.temp_hex = color_hex.clone();
                                    self.temp_name = color_name.clone();
                                }
                            }

                            ui.end_row();
                        }
                    });
            });
    }
}
