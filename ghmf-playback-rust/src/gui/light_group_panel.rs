use egui::{Context, Ui, Color32, Stroke, Rect, Pos2, Vec2, Sense, Frame};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use super::theme;

// Structure from DMX mapping file
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FixtureMapping {
    fixture_id: u8,
    fixture_name: String,
    start_channel: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DmxMapConfig {
    mappings: Vec<FixtureMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightGroup {
    pub name: String,
    pub fcw_code: String,
    pub fcw_fade_code: String,
    pub fixture_ids: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LightGroupConfig {
    pub groups: Vec<LightGroup>,
}

impl Default for LightGroupConfig {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct FixtureDefinition {
    id: u8,
    name: String,
}

pub struct LightGroupPanel {
    config: LightGroupConfig,
    config_path: String,
    
    // Form fields
    group_name: String,
    fcw_code: String,
    fcw_fade_code: String,
    
    // Available fixtures (from DMX map)
    available_fixtures: Vec<FixtureDefinition>,
    
    // Selected fixtures for current group being created
    selected_fixtures: Vec<u8>,
    
    // Editing state
    editing_group_index: Option<usize>,
    
    // UI state
    error_message: Option<String>,
    success_message: Option<String>,
}

impl Default for LightGroupPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl LightGroupPanel {
    pub fn new() -> Self {
        let config_path = "Config/light_groups.json".to_string();
        let config = Self::load_config(&config_path);
        
        // Load fixtures from DMX mapping
        let available_fixtures = Self::load_fixtures_from_dmx_map();
        
        Self {
            config,
            config_path,
            group_name: String::new(),
            fcw_code: String::new(),
            fcw_fade_code: String::new(),
            available_fixtures,
            selected_fixtures: Vec::new(),
            editing_group_index: None,
            error_message: None,
            success_message: None,
        }
    }
    
    fn load_fixtures_from_dmx_map() -> Vec<FixtureDefinition> {
        let dmx_map_path = "Config/dmx_mapping.json";
        
        if let Ok(content) = fs::read_to_string(dmx_map_path) {
            if let Ok(dmx_config) = serde_json::from_str::<DmxMapConfig>(&content) {
                return dmx_config.mappings.iter().map(|mapping| {
                    FixtureDefinition {
                        id: mapping.fixture_id,
                        name: mapping.fixture_name.clone(),
                    }
                }).collect();
            }
        }
        
        // Fallback to empty list if no DMX mappings
        Vec::new()
    }
    
    fn load_config(path: &str) -> LightGroupConfig {
        if Path::new(path).exists() {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(config) = serde_json::from_str(&content) {
                    return config;
                }
            }
        }
        LightGroupConfig::default()
    }
    
    fn save_config(&mut self) -> Result<(), String> {
        let json = serde_json::to_string_pretty(&self.config)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(&self.config_path, json)
            .map_err(|e| format!("Failed to write config file: {}", e))?;
        
        Ok(())
    }
    
    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        // Header with cyan styling matching playlist panel
        ui.label(
            egui::RichText::new("Light Group Mapping")
                .size(24.0)
                .strong()
                .color(theme::AppColors::CYAN)
        );
        ui.add_space(10.0);
        
        // Show messages
        if let Some(ref msg) = self.error_message {
            ui.colored_label(theme::AppColors::ERROR, msg);
            ui.add_space(10.0);
        }
        
        if let Some(ref msg) = self.success_message {
            ui.colored_label(theme::AppColors::SUCCESS, msg);
            ui.add_space(10.0);
        }
        
        // Main content area using panels like DMX mapper
        egui::SidePanel::left("fixture_list_panel")
            .resizable(false)
            .exact_width(295.0)
            .show_inside(ui, |ui| {
                self.show_form_and_fixtures(ui);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.show_group_cards(ui);
        });
    }
    
    fn show_form_and_fixtures(&mut self, ui: &mut Ui) {
        // Form fields section with styled frame
        Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Create Light Group")
                        .size(16.0)
                        .strong()
                        .color(theme::AppColors::CYAN)
                );
                ui.add_space(12.0);
                
                ui.label(egui::RichText::new("Group Name").color(theme::AppColors::TEXT_PRIMARY));
                ui.text_edit_singleline(&mut self.group_name);
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("FWC Code").color(theme::AppColors::TEXT_PRIMARY));
                        ui.add(egui::TextEdit::singleline(&mut self.fcw_code).desired_width(100.0));
                    });
                    ui.add_space(10.0);
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Fade Code").color(theme::AppColors::TEXT_PRIMARY));
                        ui.add(egui::TextEdit::singleline(&mut self.fcw_fade_code).desired_width(100.0));
                    });
                });
            });
        
        ui.add_space(16.0);
        
        // Available fixtures section with styled frame
        Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("Available Fixtures")
                        .size(16.0)
                        .strong()
                        .color(theme::AppColors::CYAN)
                );
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Click fixtures to add them to the group")
                        .color(theme::AppColors::TEXT_SECONDARY)
                        .size(12.0)
                );
                ui.add_space(12.0);
        
        let available_height = ui.available_height() - 100.0;
        
        egui::ScrollArea::vertical()
            .id_salt("fixture_selection_scroll")
            .max_height(available_height)
            .show(ui, |ui| {
                ui.set_max_width(330.0);
                
                let available_fixtures = self.available_fixtures.clone();
                
                for fixture in &available_fixtures {
                    let is_selected = self.selected_fixtures.contains(&fixture.id);
                    
                    let text = format!("#{:02} {}", fixture.id, fixture.name);
                    let button_width = 250.0;
                    let bg_color = if is_selected {
                        theme::AppColors::SUCCESS // Cyan/Green when selected
                    } else {
                        theme::AppColors::SURFACE_LIGHT // Gray when not selected
                    };
                    
                    ui.push_id(format!("fixture_{}", fixture.id), |ui| {
                        let frame = egui::Frame::none()
                            .fill(bg_color)
                            .inner_margin(egui::Margin::same(0.0));
                        
                        let button_response = frame.show(ui, |ui| {
                            let (rect, response) = ui.allocate_exact_size(
                                Vec2::new(button_width, 30.0),
                                Sense::click()
                            );
                            
                            // Draw text left-aligned with 20px padding (matching DMX mapper)
                            let text_pos = rect.left_top() + egui::vec2(20.0, rect.height() / 2.0);
                            ui.painter().text(
                                text_pos,
                                egui::Align2::LEFT_CENTER,
                                &text,
                                egui::FontId::default(),
                                theme::AppColors::TEXT_PRIMARY,
                            );
                            
                            response
                        }).inner;
                        
                        // Handle click to toggle selection
                        if button_response.clicked() {
                            if is_selected {
                                self.selected_fixtures.retain(|&id| id != fixture.id);
                            } else {
                                self.selected_fixtures.push(fixture.id);
                            }
                        }
                    });
                    
                    ui.add_space(2.0);
                }
            });
        
        ui.add_space(12.0);
        
        // Create Group button matching PLC test button style
        let button_text = if self.editing_group_index.is_some() {
            "💾 Update Group"
        } else {
            "➕ Create Group"
        };
        
        let button = egui::Button::new(
            egui::RichText::new(button_text)
                .size(14.0)
                .color(Color32::WHITE)
        )
        .fill(theme::AppColors::CYAN)
        .min_size(Vec2::new(265.0, 36.0))
        .rounding(8.0);
        
        if ui.add(button).clicked() {
            self.create_group();
        }
            });
    }
    
    fn show_group_cards(&mut self, ui: &mut Ui) {
        ui.label(
            egui::RichText::new("Created Groups")
                .size(18.0)
                .strong()
                .color(theme::AppColors::CYAN)
        );
        ui.add_space(12.0);
        
        if self.config.groups.is_empty() {
            Frame::none()
                .fill(theme::AppColors::SURFACE)
                .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                .rounding(12.0)
                .inner_margin(32.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("No groups created yet")
                            .color(theme::AppColors::TEXT_DISABLED)
                            .size(14.0)
                    );
                });
            return;
        }
        
        egui::ScrollArea::vertical()
            .id_salt("groups_scroll")
            .show(ui, |ui| {
                let mut groups_to_delete = Vec::new();
                let groups = self.config.groups.clone();
                let available_fixtures = self.available_fixtures.clone();
                let editing_index = self.editing_group_index;
                
                // Display cards stacked vertically
                for (group_idx, group) in groups.iter().enumerate() {
                    ui.push_id(format!("group_card_{}", group_idx), |ui| {
                        let is_editing = editing_index == Some(group_idx);
                        
                        let card_bg = if is_editing {
                            theme::AppColors::PRIMARY_DARK
                        } else {
                            theme::AppColors::SURFACE
                        };
                        
                        Frame::none()
                            .fill(card_bg)
                            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                            .rounding(12.0)
                            .inner_margin(16.0)
                            .show(ui, |ui| {
                                // First line: Title and buttons
                                ui.horizontal(|ui| {
                                    let title = if !group.fcw_fade_code.is_empty() {
                                        format!("{} / {} - {}", group.fcw_code, group.fcw_fade_code, group.name)
                                    } else {
                                        format!("{} - {}", group.fcw_code, group.name)
                                    };
                                    
                                    ui.label(
                                        egui::RichText::new(title)
                                            .color(theme::AppColors::TEXT_PRIMARY)
                                            .size(18.0)
                                            .strong()
                                    );
                                    
                                    // Buttons on the right - icon-only, no border
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        let delete_label = ui.add(
                                            egui::Label::new(egui::RichText::new("🗑").size(18.0).color(theme::AppColors::ERROR))
                                                .sense(egui::Sense::click())
                                        );
                                        if delete_label.clicked() {
                                            groups_to_delete.push(group_idx);
                                        }
                                        
                                        ui.add_space(12.0);
                                        
                                        let edit_label = ui.add(
                                            egui::Label::new(egui::RichText::new("✏").size(18.0).color(theme::AppColors::CYAN))
                                                .sense(egui::Sense::click())
                                        );
                                        if edit_label.clicked() {
                                            self.load_group_for_editing(group_idx);
                                        }
                                    });
                                });
                                
                                ui.add_space(5.0);
                                
                                // Proper grid layout with 4 cells per row
                                use egui::Grid;
                                
                                Grid::new(format!("fixture_grid_{}", group_idx))
                                    .striped(false)
                                    .spacing([10.0, 5.0]) // Horizontal and vertical spacing between cells
                                    .min_col_width(160.0)
                                    .show(ui, |ui| {
                                        for (idx, &fixture_id) in group.fixture_ids.iter().enumerate() {
                                            if let Some(fixture) = available_fixtures.iter().find(|f| f.id == fixture_id) {
                                                let text = format!("#{} {}", fixture_id, fixture.name);
                                                ui.label(
                                                    egui::RichText::new(text)
                                                        .color(theme::AppColors::TEXT_SECONDARY)
                                                        .size(13.0)
                                                );
                                            }
                                            
                                            // End row after 4 columns
                                            if (idx + 1) % 4 == 0 {
                                                ui.end_row();
                                            }
                                        }
                                        
                                        // End last row if incomplete
                                        if group.fixture_ids.len() % 4 != 0 {
                                            ui.end_row();
                                        }
                                    });
                            });
                    });
                    
                    ui.add_space(8.0);
                }
                
                // Handle deletions
                for idx in groups_to_delete.iter().rev() {
                    self.config.groups.remove(*idx);
                    if let Err(e) = self.save_config() {
                        self.error_message = Some(e);
                    } else {
                        self.success_message = Some("Group deleted successfully".to_string());
                    }
                }
            });
    }
    
    fn create_group(&mut self) {
        // Clear messages
        self.error_message = None;
        self.success_message = None;
        
        // Validate
        if self.group_name.trim().is_empty() {
            self.error_message = Some("Group name is required".to_string());
            return;
        }
        
        if self.fcw_code.trim().is_empty() {
            self.error_message = Some("FWC code is required".to_string());
            return;
        }
        
        if self.selected_fixtures.is_empty() {
            self.error_message = Some("Please select at least one fixture".to_string());
            return;
        }
        
        if let Some(idx) = self.editing_group_index {
            // Update existing group
            if idx < self.config.groups.len() {
                self.config.groups[idx] = LightGroup {
                    name: self.group_name.clone(),
                    fcw_code: self.fcw_code.clone(),
                    fcw_fade_code: self.fcw_fade_code.clone(),
                    fixture_ids: self.selected_fixtures.clone(),
                };
                self.success_message = Some("Group updated successfully".to_string());
            }
        } else {
            // Check for duplicate group name
            if self.config.groups.iter().any(|g| g.name == self.group_name) {
                self.error_message = Some("A group with this name already exists".to_string());
                return;
            }
            
            // Create new group
            let new_group = LightGroup {
                name: self.group_name.clone(),
                fcw_code: self.fcw_code.clone(),
                fcw_fade_code: self.fcw_fade_code.clone(),
                fixture_ids: self.selected_fixtures.clone(),
            };
            
            self.config.groups.push(new_group);
            self.success_message = Some("Group created successfully".to_string());
        }
        
        // Save to file
        if let Err(e) = self.save_config() {
            self.error_message = Some(e);
            return;
        }
        
        // Clear form
        self.clear_form();
    }
    
    fn load_group_for_editing(&mut self, index: usize) {
        if index < self.config.groups.len() {
            let group = &self.config.groups[index];
            self.group_name = group.name.clone();
            self.fcw_code = group.fcw_code.clone();
            self.fcw_fade_code = group.fcw_fade_code.clone();
            self.selected_fixtures = group.fixture_ids.clone();
            self.editing_group_index = Some(index);
        }
    }
    
    fn clear_form(&mut self) {
        self.group_name.clear();
        self.fcw_code.clear();
        self.fcw_fade_code.clear();
        self.selected_fixtures.clear();
        self.editing_group_index = None;
    }
}
