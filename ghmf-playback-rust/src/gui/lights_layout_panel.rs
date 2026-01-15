use egui::{Context, Ui, Color32, Stroke, Rect, Pos2, Vec2, Sense, Frame};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tracing::{info, warn};

use super::theme;

#[derive(Debug, Clone)]
struct FixtureInfo {
    id: u32,
    name: String,
}

const GRID_COLS: usize = 27;
const GRID_ROWS: usize = 6;
const CELL_SIZE: f32 = 22.0;
const CELL_PADDING: f32 = 0.25;
const ASSIGNED_COLOR: Color32 = Color32::from_rgb(0, 150, 255); // #0096FF

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightsLayout {
    /// Map of "row_col" -> fixture_id
    /// e.g., "0_5" -> 12 means row 0, col 5 has fixture 12
    pub cells: HashMap<String, u32>,
}

impl Default for LightsLayout {
    fn default() -> Self {
        Self {
            cells: HashMap::new(),
        }
    }
}

impl LightsLayout {
    fn load() -> Self {
        let config_path = "Config/lights_layout.json";
        match fs::read_to_string(config_path) {
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(layout) => layout,
                    Err(e) => {
                        warn!("Failed to parse lights layout config: {}", e);
                        Self::default()
                    }
                }
            }
            Err(_) => {
                info!("No lights layout config found, creating new one");
                Self::default()
            }
        }
    }
    
    fn save(&self) {
        let config_path = "Config/lights_layout.json";
        match serde_json::to_string_pretty(self) {
            Ok(json) => {
                if let Err(e) = fs::write(config_path, json) {
                    warn!("Failed to save lights layout config: {}", e);
                } else {
                    info!("Saved lights layout config");
                }
            }
            Err(e) => {
                warn!("Failed to serialize lights layout config: {}", e);
            }
        }
    }
    
    fn get_fixture_at(&self, row: usize, col: usize) -> Option<u32> {
        let key = format!("{}_{}", row, col);
        self.cells.get(&key).copied()
    }
    
    fn set_fixture_at(&mut self, row: usize, col: usize, fixture_id: u32) {
        let key = format!("{}_{}", row, col);
        self.cells.insert(key, fixture_id);
    }
    
    fn remove_fixture_at(&mut self, row: usize, col: usize) {
        let key = format!("{}_{}", row, col);
        self.cells.remove(&key);
    }
    
    fn find_fixture_position(&self, fixture_id: u32) -> Option<(usize, usize)> {
        for (key, &fid) in &self.cells {
            if fid == fixture_id {
                let parts: Vec<&str> = key.split('_').collect();
                if parts.len() == 2 {
                    if let (Ok(row), Ok(col)) = (parts[0].parse(), parts[1].parse()) {
                        return Some((row, col));
                    }
                }
            }
        }
        None
    }
}

pub struct LightsLayoutPanel {
    layout: LightsLayout,
    selected_fixture: Option<u32>,
    available_fixtures: Vec<FixtureInfo>,
    error_message: Option<String>,
    error_show_until: Option<std::time::Instant>,
}

impl LightsLayoutPanel {
    pub fn new() -> Self {
        Self {
            layout: LightsLayout::load(),
            selected_fixture: None,
            available_fixtures: Vec::new(),
            error_message: None,
            error_show_until: None,
        }
    }
    
    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        // Load fixtures if not already loaded
        if self.available_fixtures.is_empty() {
            self.load_fixtures();
        }
        
        // Clear error if expired
        if let Some(until) = self.error_show_until {
            if std::time::Instant::now() > until {
                self.error_message = None;
                self.error_show_until = None;
            }
        }
        
        ui.heading("Lights Layout");
        ui.add_space(10.0);
        ui.label("Create a visual layout for DMX fixture display on the Operator screen");
        ui.add_space(20.0);
        
        // Show error message if any
        if let Some(ref msg) = self.error_message {
            Frame::none()
                .fill(Color32::from_rgb(180, 0, 0))
                .stroke(Stroke::new(1.0, Color32::from_rgb(220, 0, 0)))
                .rounding(4.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.colored_label(Color32::WHITE, msg);
                });
            ui.add_space(10.0);
        }
        
        ui.horizontal(|ui| {
            // Left side: Fixture list
            ui.vertical(|ui| {
                ui.set_width(250.0);
                ui.set_min_height(ui.available_height());
                
                Frame::none()
                    .fill(theme::AppColors::SURFACE)
                    .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                    .rounding(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.heading("Available Fixtures");
                        ui.add_space(10.0);
                        
                        if self.available_fixtures.is_empty() {
                            ui.label("No fixtures available");
                        } else {
                            egui::ScrollArea::vertical()
                                .id_salt("fixtures_scroll")
                                .min_scrolled_height(600.0)
                                .show(ui, |ui| {
                                    for fixture in &self.available_fixtures {
                                        let is_selected = self.selected_fixture == Some(fixture.id);
                                        let is_assigned = self.layout.find_fixture_position(fixture.id).is_some();
                                        
                                        let button_color = if is_selected {
                                            theme::AppColors::PRIMARY
                                        } else if is_assigned {
                                            Color32::from_rgb(60, 60, 60)
                                        } else {
                                            theme::AppColors::SURFACE_LIGHT
                                        };
                                        
                                        ui.horizontal(|ui| {
                                            let text = format!("#{:02} {}", fixture.id, fixture.name);
                                            
                                            // Use Frame for left-aligned text with padding
                                            let frame = egui::Frame::none()
                                                .fill(button_color)
                                                .inner_margin(egui::Margin::same(0.0));
                                            
                                            let button_response = frame.show(ui, |ui| {
                                                let (rect, response) = ui.allocate_exact_size(
                                                    Vec2::new(220.0, 24.0),
                                                    egui::Sense::click()
                                                );
                                                
                                                // Draw text left-aligned with 20px padding
                                                let text_pos = rect.left_top() + egui::vec2(20.0, rect.height() / 2.0);
                                                ui.painter().text(
                                                    text_pos,
                                                    egui::Align2::LEFT_CENTER,
                                                    &text,
                                                    egui::FontId::proportional(13.0),
                                                    Color32::WHITE
                                                );
                                                
                                                response
                                            });
                                            
                                            let response = button_response.inner;
                                            
                                            if response.clicked() {
                                                self.selected_fixture = Some(fixture.id);
                                            }
                                        });
                                        
                                        ui.add_space(2.0);
                                    }
                                });
                        }
                    });
            });
            
            ui.add_space(20.0);
            
            // Right side: Grid
            ui.vertical(|ui| {
                Frame::none()
                    .fill(theme::AppColors::SURFACE)
                    .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                    .rounding(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.heading("Layout Grid");
                            ui.label(
                                egui::RichText::new(format!("({} × {})", GRID_COLS, GRID_ROWS))
                                    .size(14.0)
                                    .color(theme::AppColors::TEXT_SECONDARY)
                            );
                        });
                        ui.add_space(10.0);
                        
                        // Draw grid - all 8 rows visible
                        for row in 0..GRID_ROWS {
                            ui.horizontal(|ui| {
                                for col in 0..GRID_COLS {
                                            let fixture = self.layout.get_fixture_at(row, col);
                                            
                                            let (bg_color, text_color, text) = if let Some(fid) = fixture {
                                                (ASSIGNED_COLOR, Color32::WHITE, format!("{}", fid))
                                            } else {
                                                (theme::AppColors::BACKGROUND, theme::AppColors::TEXT_SECONDARY, "".to_string())
                                            };
                                            
                                            let cell_rect = Rect::from_min_size(
                                                ui.cursor().min,
                                                Vec2::new(CELL_SIZE, CELL_SIZE),
                                            );
                                            
                                            let response = ui.allocate_rect(cell_rect, Sense::click());
                                            
                                            // Draw cell background
                                            ui.painter().rect_filled(
                                                cell_rect,
                                                2.0,
                                                bg_color,
                                            );
                                            
                                            // Draw cell border
                                            ui.painter().rect_stroke(
                                                cell_rect,
                                                2.0,
                                                Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT),
                                            );
                                            
                                            // Draw text if fixture assigned
                                            if !text.is_empty() {
                                                ui.painter().text(
                                                    cell_rect.center(),
                                                    egui::Align2::CENTER_CENTER,
                                                    text,
                                                    egui::FontId::monospace(12.0),
                                                    text_color,
                                                );
                                            }
                                            
                                            // Handle click
                                            if response.clicked() {
                                                self.handle_cell_click(row, col);
                                            }
                                            
                                            ui.add_space(CELL_PADDING);
                                        }
                                    });
                                }
                    });
            });
        });
    }
    
    fn handle_cell_click(&mut self, row: usize, col: usize) {
        if let Some(existing_fixture) = self.layout.get_fixture_at(row, col) {
            // Cell already has a fixture - unassign it
            self.layout.remove_fixture_at(row, col);
            self.layout.save();
            info!("Unassigned fixture {} from row {}, col {}", existing_fixture, row, col);
        } else if let Some(selected_fixture) = self.selected_fixture {
            // Check if fixture is already assigned elsewhere
            if let Some((existing_row, existing_col)) = self.layout.find_fixture_position(selected_fixture) {
                self.error_message = Some(format!(
                    "Fixture {} is already assigned to Row {}, Col {}",
                    selected_fixture, existing_row + 1, existing_col + 1
                ));
                self.error_show_until = Some(std::time::Instant::now() + std::time::Duration::from_secs(3));
            } else {
                // Assign fixture to cell
                self.layout.set_fixture_at(row, col, selected_fixture);
                self.layout.save();
                info!("Assigned fixture {} to row {}, col {}", selected_fixture, row, col);
            }
        } else {
            // No fixture selected
            self.error_message = Some("Please select a fixture from the list first".to_string());
            self.error_show_until = Some(std::time::Instant::now() + std::time::Duration::from_secs(2));
        }
    }
    
    fn load_fixtures(&mut self) {
        // Load fixtures from DMX map JSON
        let dmx_map_path = "Config/dmx_mapping.json";
        info!("Loading fixtures from: {}", dmx_map_path);
        match fs::read_to_string(dmx_map_path) {
            Ok(contents) => {
                match serde_json::from_str::<serde_json::Value>(&contents) {
                    Ok(json) => {
                        if let Some(mappings) = json.get("mappings").and_then(|v| v.as_array()) {
                            let mut fixtures: Vec<FixtureInfo> = mappings.iter()
                                .filter_map(|f| {
                                    let id = f.get("fixture_id").and_then(|id| id.as_u64()).map(|id| id as u32)?;
                                    let name = f.get("fixture_name").and_then(|n| n.as_str()).unwrap_or("Unknown").to_string();
                                    Some(FixtureInfo { id, name })
                                })
                                .collect();
                            fixtures.sort_by_key(|f| f.id);
                            self.available_fixtures = fixtures;
                            info!("Loaded {} fixtures from DMX mapping", self.available_fixtures.len());
                        } else {
                            warn!("No 'mappings' array found in DMX mapping JSON");
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse DMX mapping: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to load DMX mapping file: {}", e);
            }
        }
    }
}
