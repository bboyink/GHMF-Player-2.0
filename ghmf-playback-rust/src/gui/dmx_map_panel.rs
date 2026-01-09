use egui::{Context, Ui, Color32, Stroke, Rect, Pos2, Vec2, Sense, Response, Key};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Constants
const GRID_COLS: usize = 25;
const GRID_ROWS: usize = 21; // 512 channels / 25 columns ~= 21 rows (525 cells, but we only use 512)
const CELL_SIZE: f32 = 30.0;
const CELL_PADDING: f32 = 2.0;
const ASSIGNED_BG: Color32 = Color32::from_rgb(30, 41, 59); // #1E293B

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FixtureType {
    Light,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LightType {
    RGBW,
    RGB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureDefinition {
    pub id: u8,
    pub name: String,
    #[serde(default = "default_fixture_type")]
    pub fixture_type: FixtureType,
    #[serde(default = "default_light_type")]
    pub light_type: Option<LightType>,
    #[serde(default = "default_channel_count")]
    pub channel_count: u8,
}

fn default_fixture_type() -> FixtureType {
    FixtureType::Light
}

fn default_light_type() -> Option<LightType> {
    Some(LightType::RGBW)
}

fn default_channel_count() -> u8 {
    4
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureMapping {
    pub fixture_id: u8,
    pub fixture_name: String,
    pub start_channel: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmxMapConfig {
    pub mappings: Vec<FixtureMapping>,
}

impl DmxMapConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: DmxMapConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            mappings: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixtureConfig {
    pub custom_fixtures: Vec<FixtureDefinition>,
}

impl FixtureConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: FixtureConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn new() -> Self {
        Self {
            custom_fixtures: Vec::new(),
        }
    }
}

pub struct DmxMapPanel {
    fixtures: Vec<FixtureDefinition>,
    config: DmxMapConfig,
    config_path: String,
    fixture_config: FixtureConfig,
    fixture_config_path: String,
    selected_fixture_to_place: Option<FixtureDefinition>,
    selected_fixture_start: Option<u16>,
    hovered_cell: Option<u16>,
    hovered_fixture_id: Option<u8>,
    
    // Add fixture dialog state
    show_add_fixture_dialog: bool,
    new_fixture_name: String,
    new_fixture_type: FixtureType,
    new_light_type: LightType,
    new_channel_count: String,
    
    // Delete confirmation dialog state
    show_delete_confirmation: bool,
    fixture_to_delete_id: Option<u8>,
}

impl Default for DmxMapPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl DmxMapPanel {
    pub fn new() -> Self {
        let mut fixtures = vec![
            FixtureDefinition { id: 1, name: "Mod. 1 Front Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 2, name: "Mod. 1 Front Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 3, name: "Mod. 1 Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 4, name: "Mod. 1 Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 5, name: "Mod. 1 Back Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 6, name: "Mod. 1 Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 7, name: "Mod. 2 Front Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 8, name: "Mod. 2 Front Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 9, name: "Mod. 2 Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 10, name: "Mod. 2 Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 11, name: "Mod. 2 Back Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 12, name: "Mod. 2 Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 13, name: "Mod. 3 Front Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 14, name: "Mod. 3 Front Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 15, name: "Mod. 3 Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 16, name: "Mod. 3 Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 17, name: "Mod. 3 Back Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 18, name: "Mod. 3 Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 19, name: "Mod. 4 Front Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 20, name: "Mod. 4 Front Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 21, name: "Mod. 4 Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 22, name: "Mod. 4 Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 23, name: "Mod. 4 Back Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 24, name: "Mod. 4 Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 25, name: "Mod. 5 Front Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 26, name: "Mod. 5 Front Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 27, name: "Mod. 5 Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 28, name: "Mod. 5 Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 29, name: "Mod. 5 Back Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 30, name: "Mod. 5 Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 31, name: "Mod. 6 Front Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 32, name: "Mod. 6 Front Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 33, name: "Mod. 6 Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 34, name: "Mod. 6 Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 35, name: "Mod. 6 Back Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 36, name: "Mod. 6 Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 37, name: "Mod. 7 Front Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 38, name: "Mod. 7 Front Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 39, name: "Mod. 7 Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 40, name: "Mod. 7 Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 41, name: "Mod. 7 Back Center".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 42, name: "Mod. 7 Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 43, name: "Peacock Light 1".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 44, name: "Peacock Light 2".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 45, name: "Peacock Light 3".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 46, name: "Peacock Light 4".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 47, name: "Peacock Light 5".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 48, name: "Peacock Light 6".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 49, name: "Peacock Back Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 50, name: "Peacock Back Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 51, name: "Helix Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 52, name: "Helix Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            
            FixtureDefinition { id: 53, name: "Dove Left".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
            FixtureDefinition { id: 54, name: "Dove Right".to_string(), fixture_type: FixtureType::Light, light_type: Some(LightType::RGBW), channel_count: 4 },
        ];

        let config_path = "Config/dmx_mapping.json".to_string();
        let config = DmxMapConfig::load(&config_path).unwrap_or_else(|_| DmxMapConfig::new());
        
        let fixture_config_path = "Config/custom_fixtures.json".to_string();
        let fixture_config = FixtureConfig::load(&fixture_config_path).unwrap_or_else(|_| FixtureConfig::new());
        
        // Add custom fixtures to the list
        fixtures.extend(fixture_config.custom_fixtures.clone());

        Self {
            fixtures,
            config,
            config_path,
            fixture_config,
            fixture_config_path,
            selected_fixture_to_place: None,
            selected_fixture_start: None,
            hovered_cell: None,
            hovered_fixture_id: None,
            show_add_fixture_dialog: false,
            new_fixture_name: String::new(),
            new_fixture_type: FixtureType::Light,
            new_light_type: LightType::RGBW,
            new_channel_count: "4".to_string(),
            show_delete_confirmation: false,
            fixture_to_delete_id: None,
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading(egui::RichText::new("DMX Channel Mapper").color(Color32::WHITE));
        ui.add_space(10.0);

        egui::SidePanel::left("fixture_list_panel")
            .resizable(false)
            .exact_width(250.0)
            .show_inside(ui, |ui| {
                self.show_fixture_list(ui);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.show_dmx_grid(ctx, ui);
        });

        // Show add fixture dialog
        if self.show_add_fixture_dialog {
            self.show_add_fixture_dialog(ctx);
        }

        // Show delete confirmation dialog
        if self.show_delete_confirmation {
            self.show_delete_confirmation_dialog(ctx);
        }

        // Handle keyboard input for deletion
        if ctx.input(|i| i.key_pressed(Key::Delete)) {
            if let Some(start_channel) = self.selected_fixture_start {
                self.delete_fixture(start_channel);
                self.selected_fixture_start = None;
                self.save_config();
            }
        }
    }

    fn show_fixture_list(&mut self, ui: &mut Ui) {
        ui.heading(egui::RichText::new("Fixtures").color(Color32::WHITE));
        ui.add_space(5.0);
        ui.label(egui::RichText::new("Click a fixture, then click a channel in the grid to assign it.").color(Color32::WHITE));
        ui.add_space(10.0);

        let available_height = ui.available_height() - 80.0; // Reserve space for button and spacing

        egui::ScrollArea::vertical()
            .id_salt("fixture_list_scroll")
            .max_height(available_height)
            .show(ui, |ui| {
                ui.add_space(0.0);
                ui.set_max_width(230.0);
                
                let fixtures_clone = self.fixtures.clone();
                
                for fixture in &fixtures_clone {
                    let is_assigned = self.config.mappings.iter()
                        .any(|m| m.fixture_id == fixture.id);
                    
                    let is_selected = self.selected_fixture_to_place.as_ref()
                        .map(|f| f.id == fixture.id)
                        .unwrap_or(false);

                    ui.horizontal(|ui| {
                        let text = format!("#{:02} {}", fixture.id, fixture.name);
                        
                        let bg_color = if is_selected {
                            Color32::from_rgb(0, 120, 200)
                        } else if is_assigned {
                            Color32::from_rgb(0, 144, 81)
                        } else {
                            Color32::from_rgb(50, 50, 60)
                        };

                        let text_color = Color32::WHITE;

                        // Main fixture button
                        let button_width = if fixture.id > 54 { 180.0 } else { 220.0 };
                        
                        // Use Frame for left-aligned text with padding
                        let frame = egui::Frame::none()
                            .fill(bg_color)
                            .inner_margin(egui::Margin::same(0.0));
                        
                        let button_response = frame.show(ui, |ui| {
                            let (rect, response) = ui.allocate_exact_size(
                                Vec2::new(button_width, 30.0),
                                egui::Sense::click()
                            );
                            
                            // Draw text left-aligned with 20px padding
                            let text_pos = rect.left_top() + egui::vec2(20.0, rect.height() / 2.0);
                            ui.painter().text(
                                text_pos,
                                egui::Align2::LEFT_CENTER,
                                &text,
                                egui::FontId::default(),
                                text_color
                            );
                            
                            response
                        });
                        
                        let response = button_response.inner;

                        // Handle click to select fixture for placement
                        if response.clicked() {
                            self.selected_fixture_to_place = Some(fixture.clone());
                        }
                        
                        // Track hover for highlighting in grid
                        if response.hovered() && is_assigned {
                            self.hovered_fixture_id = Some(fixture.id);
                        }

                        // Show tooltip with assignment info
                        if let Some(mapping) = self.config.mappings.iter().find(|m| m.fixture_id == fixture.id) {
                            response.on_hover_text(format!("Assigned to channel {}\nClick to select for reassignment", mapping.start_channel));
                        } else {
                            response.on_hover_text("Click to select, then click a channel in the grid");
                        }

                        // Add delete button for custom fixtures
                        if fixture.id > 54 {
                            let delete_button = egui::Button::new(
                                egui::RichText::new("🗑").size(16.0).color(Color32::WHITE)
                            )
                            .fill(Color32::from_rgb(180, 50, 50))
                            .min_size(Vec2::new(30.0, 30.0));

                            let delete_response = ui.add(delete_button);
                            
                            if delete_response.clicked() {
                                self.fixture_to_delete_id = Some(fixture.id);
                                self.show_delete_confirmation = true;
                            }
                            
                            delete_response.on_hover_text("Delete this custom fixture");
                        }
                    });
                }
            });
            
        // Reset hover if not hovering any fixture in the scroll area
        // (This check needs to happen here while we still have the ui context)
        
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(5.0);
        
        // Add Fixture button
        if ui.add(egui::Button::new("➕ Add Fixture").min_size(Vec2::new(230.0, 35.0))).clicked() {
            self.show_add_fixture_dialog = true;
            self.new_fixture_name = String::new();
            self.new_fixture_type = FixtureType::Light;
            self.new_light_type = LightType::RGBW;
            self.new_channel_count = "4".to_string();
        }
    }

    fn show_dmx_grid(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.heading(egui::RichText::new("DMX Universe (512 Channels)").color(Color32::WHITE));
        ui.add_space(5.0);
        
        if let Some(fixture) = &self.selected_fixture_to_place {
            ui.label(egui::RichText::new(format!("Selected: #{:02} {} - Click a channel to place", fixture.id, fixture.name))
                .color(Color32::WHITE));
        } else if self.selected_fixture_start.is_some() {
            ui.label(egui::RichText::new("Press DELETE to remove selected fixture")
                .color(Color32::WHITE));
        } else {
            ui.label(egui::RichText::new("Click a fixture in the list, then click a channel to place it")
                .color(Color32::WHITE));
        }
        
        ui.add_space(10.0);

        // Build channel assignment map
        let mut channel_map: HashMap<u16, (u8, String)> = HashMap::new();
        for mapping in &self.config.mappings {
            let channel_count = self.fixtures.iter()
                .find(|f| f.id == mapping.fixture_id)
                .map(|f| f.channel_count as u16)
                .unwrap_or(4);
            for offset in 0..channel_count {
                let channel = mapping.start_channel + offset;
                channel_map.insert(channel, (mapping.fixture_id, mapping.fixture_name.clone()));
            }
        }

        let scroll = egui::ScrollArea::both()
            .id_salt("dmx_grid_scroll")
            .show(ui, |ui| {
                let total_width = GRID_COLS as f32 * (CELL_SIZE + CELL_PADDING) + 20.0; // Add space for scroll bar
                let total_height = GRID_ROWS as f32 * (CELL_SIZE + CELL_PADDING) + 20.0; // Add space for scroll bar
                
                let (response, painter) = ui.allocate_painter(
                    Vec2::new(total_width, total_height),
                    Sense::click()
                );

                // Track hovered cell
                if let Some(pos) = ctx.input(|i| i.pointer.hover_pos()) {
                    if response.rect.contains(pos) {
                        let relative_pos = pos - response.rect.min;
                        let col = (relative_pos.x / (CELL_SIZE + CELL_PADDING)) as usize;
                        let row = (relative_pos.y / (CELL_SIZE + CELL_PADDING)) as usize;
                        let channel = (row * GRID_COLS + col + 1) as u16;
                        if channel <= 512 {
                            self.hovered_cell = Some(channel);
                        } else {
                            self.hovered_cell = None;
                        }
                    } else {
                        self.hovered_cell = None;
                    }
                } else {
                    self.hovered_cell = None;
                }

                // Draw grid cells
                for row in 0..GRID_ROWS {
                    for col in 0..GRID_COLS {
                        let channel = (row * GRID_COLS + col + 1) as u16;
                        if channel > 512 {
                            continue;
                        }

                        let x = response.rect.min.x + col as f32 * (CELL_SIZE + CELL_PADDING);
                        let y = response.rect.min.y + row as f32 * (CELL_SIZE + CELL_PADDING);
                        
                        let cell_rect = Rect::from_min_size(
                            Pos2::new(x, y),
                            Vec2::new(CELL_SIZE, CELL_SIZE)
                        );

                        // Determine cell state
                        let is_assigned = channel_map.contains_key(&channel);
                        let is_selected = if let Some(sel_start) = self.selected_fixture_start {
                            if let Some((fixture_id, _)) = channel_map.get(&sel_start) {
                                let channel_count = self.fixtures.iter()
                                    .find(|f| f.id == *fixture_id)
                                    .map(|f| f.channel_count as u16)
                                    .unwrap_or(4);
                                channel >= sel_start && channel < sel_start + channel_count
                            } else {
                                false
                            }
                        } else {
                            false
                        };
                        
                        // Check if this cell belongs to the hovered fixture
                        let is_hovered_fixture = if let Some(hovered_id) = self.hovered_fixture_id {
                            channel_map.get(&channel).map(|(id, _)| *id == hovered_id).unwrap_or(false)
                        } else {
                            false
                        };
                        
                        let is_hovered = self.hovered_cell == Some(channel);

                        // Cell background
                        let bg_color = if is_selected {
                            Color32::from_rgb(70, 80, 120)
                        } else if is_hovered_fixture {
                            Color32::from_rgb(60, 100, 140)
                        } else if is_assigned {
                            ASSIGNED_BG
                        } else if is_hovered && self.selected_fixture_to_place.is_some() {
                            if self.can_place_fixture(channel) {
                                Color32::from_rgb(40, 80, 40)
                            } else {
                                Color32::from_rgb(80, 40, 40)
                            }
                        } else {
                            Color32::from_rgb(40, 40, 50)
                        };

                        painter.rect_filled(cell_rect, 2.0, bg_color);

                        // Cell border
                        let border_color = if is_selected {
                            Color32::from_rgb(100, 200, 255)
                        } else if is_hovered_fixture {
                            Color32::from_rgb(100, 150, 200)
                        } else {
                            Color32::from_rgb(60, 60, 70)
                        };
                        let border_width = if is_hovered_fixture { 2.0 } else { 1.0 };
                        painter.rect_stroke(cell_rect, 2.0, Stroke::new(border_width, border_color));

                        // Bottom border for channel type (RGBW or Other)
                        if is_assigned {
                            if let Some((fixture_id, _)) = channel_map.get(&channel) {
                                if let Some(fixture) = self.fixtures.iter().find(|f| f.id == *fixture_id) {
                                    if let Some(mapping) = self.config.mappings.iter().find(|m| m.fixture_id == *fixture_id) {
                                        let offset = channel - mapping.start_channel;
                                        
                                        let color = if fixture.fixture_type == FixtureType::Other {
                                            // Yellow for "Other" type fixtures
                                            Color32::from_rgb(255, 215, 0)
                                        } else if let Some(light_type) = &fixture.light_type {
                                            // RGBW or RGB coloring
                                            match light_type {
                                                LightType::RGBW => match offset {
                                                    0 => Color32::from_rgb(255, 0, 0),     // Red
                                                    1 => Color32::from_rgb(0, 255, 0),     // Green
                                                    2 => Color32::from_rgb(0, 100, 255),   // Blue
                                                    3 => Color32::from_rgb(255, 255, 255), // White
                                                    _ => Color32::GRAY,
                                                },
                                                LightType::RGB => match offset {
                                                    0 => Color32::from_rgb(255, 0, 0),     // Red
                                                    1 => Color32::from_rgb(0, 255, 0),     // Green
                                                    2 => Color32::from_rgb(0, 100, 255),   // Blue
                                                    _ => Color32::GRAY,
                                                }
                                            }
                                        } else {
                                            Color32::GRAY
                                        };
                                        
                                        let border_rect = Rect::from_min_max(
                                            Pos2::new(cell_rect.min.x, cell_rect.max.y - 2.0),
                                            cell_rect.max
                                        );
                                        painter.rect_filled(border_rect, 0.0, color);
                                    }
                                }
                            }
                        }

                        // Channel number
                        let text_color = if is_assigned {
                            Color32::from_rgb(180, 180, 180)
                        } else {
                            Color32::from_rgb(120, 120, 130)
                        };

                        painter.text(
                            cell_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            channel.to_string(),
                            egui::FontId::proportional(10.0),
                            text_color
                        );

                        // Handle cell click
                        let cell_response = ui.interact(cell_rect, ui.id().with(channel), Sense::click());
                        
                        if cell_response.clicked() {
                            if let Some(fixture) = &self.selected_fixture_to_place {
                                // Place the selected fixture here if possible
                                if self.can_place_fixture(channel) {
                                    self.place_fixture(fixture.clone(), channel);
                                    self.save_config();
                                    self.selected_fixture_to_place = None;
                                    self.selected_fixture_start = None;
                                }
                            } else if is_assigned {
                                // Select this fixture for deletion
                                if let Some((_, _)) = channel_map.get(&channel) {
                                    let start = self.find_fixture_start(channel);
                                    self.selected_fixture_start = start;
                                }
                            }
                        }

                        // Tooltip
                        if is_assigned {
                            if let Some((fixture_id, fixture_name)) = channel_map.get(&channel) {
                                if let Some(mapping) = self.config.mappings.iter().find(|m| m.fixture_id == *fixture_id) {
                                    let offset = channel - mapping.start_channel;
                                    let channel_type = match offset {
                                        0 => "Red",
                                        1 => "Green",
                                        2 => "Blue",
                                        3 => "White",
                                        _ => "Unknown",
                                    };
                                    cell_response.on_hover_text(
                                        format!("#{:02} {}\nChannel: {} ({})", fixture_id, fixture_name, channel, channel_type)
                                    );
                                }
                            }
                        } else {
                            cell_response.on_hover_text(format!("Channel {}\nUnassigned", channel));
                        }
                    }
                }
            });
    }

    fn can_place_fixture(&self, start_channel: u16) -> bool {
        // Get the selected fixture's channel count
        let channel_count = if let Some(fixture) = &self.selected_fixture_to_place {
            fixture.channel_count as u16
        } else {
            return false;
        };
        
        if start_channel > 512 - channel_count + 1 {
            return false; // Not enough room
        }

        // Check if any of the channels are already assigned
        for offset in 0..channel_count {
            let channel = start_channel + offset;
            for mapping in &self.config.mappings {
                // Get the channel count for this mapped fixture
                let mapped_fixture_channels = self.fixtures.iter()
                    .find(|f| f.id == mapping.fixture_id)
                    .map(|f| f.channel_count as u16)
                    .unwrap_or(4);
                let mapping_end = mapping.start_channel + mapped_fixture_channels - 1;
                if channel >= mapping.start_channel && channel <= mapping_end {
                    return false;
                }
            }
        }

        true
    }

    fn place_fixture(&mut self, fixture: FixtureDefinition, start_channel: u16) {
        // Remove existing mapping for this fixture if any
        self.config.mappings.retain(|m| m.fixture_id != fixture.id);

        // Add new mapping
        self.config.mappings.push(FixtureMapping {
            fixture_id: fixture.id,
            fixture_name: fixture.name,
            start_channel,
        });

        // Sort by start channel
        self.config.mappings.sort_by_key(|m| m.start_channel);
    }

    fn delete_fixture(&mut self, start_channel: u16) {
        self.config.mappings.retain(|m| m.start_channel != start_channel);
    }

    fn find_fixture_start(&self, channel: u16) -> Option<u16> {
        for mapping in &self.config.mappings {
            let channel_count = self.fixtures.iter()
                .find(|f| f.id == mapping.fixture_id)
                .map(|f| f.channel_count as u16)
                .unwrap_or(4);
            let end_channel = mapping.start_channel + channel_count - 1;
            if channel >= mapping.start_channel && channel <= end_channel {
                return Some(mapping.start_channel);
            }
        }
        None
    }

    fn save_config(&self) {
        if let Err(e) = self.config.save(&self.config_path) {
            eprintln!("Failed to save DMX mapping config: {}", e);
        }
    }
    
    fn save_fixture_config(&self) {
        if let Err(e) = self.fixture_config.save(&self.fixture_config_path) {
            eprintln!("Failed to save fixture config: {}", e);
        }
    }
    
    fn show_add_fixture_dialog(&mut self, ctx: &Context) {
        let mut should_close = false;
        let mut should_save = false;
        
        egui::Window::new("Add Custom Fixture")
            .open(&mut self.show_add_fixture_dialog)
            .resizable(false)
            .collapsible(false)
            .default_width(400.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(5.0);
                
                // Fixture Name
                ui.label(egui::RichText::new("Fixture Name:").color(Color32::WHITE));
                ui.text_edit_singleline(&mut self.new_fixture_name);
                ui.add_space(10.0);
                
                // Fixture Type
                ui.label(egui::RichText::new("Type of Fixture:").color(Color32::WHITE));
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.new_fixture_type, FixtureType::Light, "Light");
                    ui.radio_value(&mut self.new_fixture_type, FixtureType::Other, "Other");
                });
                ui.add_space(10.0);
                
                // Light-specific options
                if self.new_fixture_type == FixtureType::Light {
                    ui.label(egui::RichText::new("Light Type:").color(Color32::WHITE));
                    ui.horizontal(|ui| {
                        if ui.radio_value(&mut self.new_light_type, LightType::RGBW, "RGBW").clicked() {
                            self.new_channel_count = "4".to_string();
                        }
                        if ui.radio_value(&mut self.new_light_type, LightType::RGB, "RGB").clicked() {
                            self.new_channel_count = "3".to_string();
                        }
                    });
                } else {
                    // Other type - manual channel count
                    ui.label(egui::RichText::new("Number of Channels:").color(Color32::WHITE));
                    ui.add(egui::TextEdit::singleline(&mut self.new_channel_count).desired_width(100.0));
                }
                
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Buttons
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        should_save = true;
                        should_close = true;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                });
            });
        
        if should_save {
            self.add_new_fixture();
        }
        
        if should_close {
            self.show_add_fixture_dialog = false;
        }
    }
    
    fn show_delete_confirmation_dialog(&mut self, ctx: &Context) {
        let mut should_delete = false;
        let mut should_close = false;
        
        let fixture_name = self.fixture_to_delete_id
            .and_then(|id| self.fixtures.iter().find(|f| f.id == id))
            .map(|f| f.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        
        egui::Window::new("Delete Fixture")
            .open(&mut self.show_delete_confirmation)
            .resizable(false)
            .collapsible(false)
            .default_width(350.0)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.add_space(5.0);
                
                ui.label(egui::RichText::new(format!("Are you sure you want to delete '{}'?", fixture_name)).color(Color32::WHITE));
                ui.add_space(10.0);
                ui.label(egui::RichText::new("This will also remove any channel assignments for this fixture.").color(Color32::WHITE));
                
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Buttons
                ui.horizontal(|ui| {
                    if ui.button("Yes, Delete").clicked() {
                        should_delete = true;
                        should_close = true;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                });
            });
        
        if should_delete {
            if let Some(id) = self.fixture_to_delete_id {
                self.delete_custom_fixture(id);
            }
        }
        
        if should_close {
            self.show_delete_confirmation = false;
            self.fixture_to_delete_id = None;
        }
    }
    
    fn add_new_fixture(&mut self) {
        if self.new_fixture_name.trim().is_empty() {
            return;
        }
        
        // Get next available ID
        let next_id = self.fixtures.iter().map(|f| f.id).max().unwrap_or(54) + 1;
        
        // Parse channel count
        let channel_count = if self.new_fixture_type == FixtureType::Light {
            if self.new_light_type == LightType::RGBW { 4 } else { 3 }
        } else {
            self.new_channel_count.parse::<u8>().unwrap_or(1).max(1).min(32)
        };
        
        let new_fixture = FixtureDefinition {
            id: next_id,
            name: self.new_fixture_name.clone(),
            fixture_type: self.new_fixture_type.clone(),
            light_type: if self.new_fixture_type == FixtureType::Light {
                Some(self.new_light_type.clone())
            } else {
                None
            },
            channel_count,
        };
        
        // Add to fixtures list
        self.fixtures.push(new_fixture.clone());
        
        // Add to custom fixtures config
        self.fixture_config.custom_fixtures.push(new_fixture);
        
        // Save
        self.save_fixture_config();
    }
    
    fn delete_custom_fixture(&mut self, fixture_id: u8) {
        // Only allow deleting custom fixtures (ID > 54)
        if fixture_id <= 54 {
            return;
        }
        
        // Remove from fixtures list
        self.fixtures.retain(|f| f.id != fixture_id);
        
        // Remove from custom config
        self.fixture_config.custom_fixtures.retain(|f| f.id != fixture_id);
        
        // Remove any mappings for this fixture
        self.config.mappings.retain(|m| m.fixture_id != fixture_id);
        
        // Save both configs
        self.save_fixture_config();
        self.save_config();
    }
}

