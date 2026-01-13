use egui::{Context, Ui, Frame, Stroke, Color32, Vec2, RichText};
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, Datelike, Local};
use super::theme;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTimeEntry {
    pub date: String,  // Format: MM-DD-YYYY
    pub time: String,  // Format: HH:MM AM/PM
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTimeConfig {
    pub entries: Vec<StartTimeEntry>,
}

impl Default for StartTimeConfig {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

pub struct StartTimePanel {
    config: StartTimeConfig,
    config_path: String,
    
    // Calendar state
    current_month: NaiveDate,
    selected_date: Option<NaiveDate>,
    
    // Available times
    available_times: Vec<String>,
}

impl Default for StartTimePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl StartTimePanel {
    pub fn new() -> Self {
        let config_path = "Config/start_time.json".to_string();
        let config = Self::load_config(&config_path);
        
        let today = Local::now().naive_local().date();
        
        Self {
            config,
            config_path,
            current_month: today.with_day(1).unwrap(),
            selected_date: None,
            available_times: vec![
                "10:10 PM".to_string(),
                "10:00 PM".to_string(),
                "09:50 PM".to_string(),
                "09:40 PM".to_string(),
                "09:30 PM".to_string(),
                "09:20 PM".to_string(),
                "09:10 PM".to_string(),
                "09:00 PM".to_string(),
                "08:50 PM".to_string(),
                "08:40 PM".to_string(),
                "08:30 PM".to_string(),
            ],
        }
    }
    
    fn load_config(path: &str) -> StartTimeConfig {
        if let Ok(data) = std::fs::read_to_string(path) {
            if let Ok(config) = serde_json::from_str(&data) {
                return config;
            }
        }
        StartTimeConfig::default()
    }
    
    fn save_config(&self) {
        if let Ok(data) = serde_json::to_string_pretty(&self.config) {
            let _ = std::fs::write(&self.config_path, data);
        }
    }
    
    /// Get show start time for today's date, or default to "7:00 PM"
    pub fn get_today_start_time(&self) -> String {
        let today = Local::now().naive_local().date();
        let today_str = today.format("%m-%d-%Y").to_string();
        
        // Look for today's entry in config
        for entry in &self.config.entries {
            if entry.date == today_str {
                return entry.time.clone();
            }
        }
        
        // Default to 7:00 PM if not found
        "7:00 PM".to_string()
    }
    
    pub fn show(&mut self, _ctx: &Context, ui: &mut Ui) {
        // Header
        ui.label(
            RichText::new("Show Start Time")
                .size(24.0)
                .strong()
                .color(theme::AppColors::CYAN)
        );
        ui.add_space(10.0);
        
        ui.horizontal(|ui| {
            // Left column - Calendar
            ui.vertical(|ui| {
                ui.set_max_width(270.0);
                
                Frame::none()
                    .fill(theme::AppColors::SURFACE)
                    .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                    .rounding(12.0)
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.set_max_width(238.0);
                        ui.label(
                            RichText::new("Select Date")
                                .size(16.0)
                                .strong()
                                .color(theme::AppColors::CYAN)
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new("Choose the show date")
                                .color(theme::AppColors::TEXT_SECONDARY)
                                .size(12.0)
                        );
                        ui.add_space(12.0);
                        
                        self.show_calendar(ui);
                    });
            });
            
            ui.add_space(20.0);
            
            // Middle column - Time selection
            ui.vertical(|ui| {
                ui.set_max_width(270.0);
                
                // Time selection (always visible)
                Frame::none()
                    .fill(theme::AppColors::SURFACE)
                    .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                    .rounding(12.0)
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Select Time")
                                .size(16.0)
                                .strong()
                                .color(theme::AppColors::CYAN)
                        );
                        ui.add_space(8.0);
                        ui.label(
                            RichText::new("Choose when the show should start")
                                .color(theme::AppColors::TEXT_SECONDARY)
                                .size(12.0)
                        );
                        ui.add_space(12.0);
                        
                        self.show_time_selector(ui);
                    });
            });
            
            ui.add_space(20.0);
            
            // Right column - Configured start times
            ui.vertical(|ui| {
                ui.set_min_width(300.0);
                
                Frame::none()
                    .fill(theme::AppColors::SURFACE)
                    .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                    .rounding(12.0)
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new("Configured Start Times")
                                .size(16.0)
                                .strong()
                                .color(theme::AppColors::CYAN)
                        );
                        ui.add_space(12.0);
                        
                        if self.config.entries.is_empty() {
                            ui.label(
                                RichText::new("No start times configured")
                                    .color(theme::AppColors::TEXT_SECONDARY)
                            );
                        } else {
                            egui::ScrollArea::vertical()
                                .id_source("start_times_scroll")
                                .max_height(500.0)
                                .show(ui, |ui| {
                                    let mut to_remove: Option<usize> = None;
                                    
                                    for (idx, entry) in self.config.entries.iter().enumerate() {
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(format!("📅 {}     🕐 {}", entry.date, entry.time))
                                                    .size(15.0)
                                                    .color(theme::AppColors::TEXT_PRIMARY)
                                            );
                                            
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                let delete_button = egui::Button::new(
                                                    RichText::new("🗑")
                                                        .size(7.0)
                                                        .color(Color32::WHITE)
                                                )
                                                .fill(theme::AppColors::ERROR)
                                                .min_size(Vec2::new(12.0, 12.0));
                                                
                                                if ui.add(delete_button).clicked() {
                                                    to_remove = Some(idx);
                                                }
                                            });
                                        });
                                        ui.add_space(6.0);
                                    }
                                    
                                    if let Some(idx) = to_remove {
                                        self.config.entries.remove(idx);
                                        self.save_config();
                                    }
                                });
                        }
                    });
            });
        });
    }
    
    fn show_calendar(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
            // Month navigation
            ui.horizontal(|ui| {
                if ui.small_button("◀").clicked() {
                    self.current_month = self.prev_month(self.current_month);
                }
                
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                    ui.label(
                        RichText::new(self.current_month.format("%B %Y").to_string())
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE)
                    );
                });
                
                if ui.small_button("▶").clicked() {
                    self.current_month = self.next_month(self.current_month);
                }
            });
            
            ui.add_space(8.0);
                    
                    // Calendar data
                    let first_day = self.current_month.with_day(1).unwrap();
                    let last_day = self.last_day_of_month(self.current_month);
                    let start_weekday = first_day.weekday().num_days_from_sunday();
                    let prev_month = self.prev_month(self.current_month);
                    let prev_month_last_day = self.last_day_of_month(prev_month);
                    
                    let cell_size = 28.0;
                    
                    // 8x7 Grid: 1 blank column + 7 date columns
                    egui::Grid::new("calendar_grid")
                        .spacing([2.0, 2.0])
                        .min_col_width(cell_size)
                        .max_col_width(cell_size)
                        .show(ui, |ui| {
                            // Row 1: Day headers
                            ui.label("");  // Blank column
                            
                            let days = ["S", "M", "T", "W", "T", "F", "S"];
                            for day in &days {
                                ui.allocate_ui_with_layout(
                                    Vec2::new(cell_size, 20.0),
                                    egui::Layout::centered_and_justified(egui::Direction::TopDown),
                                    |ui| {
                                        ui.label(
                                            RichText::new(*day)
                                                .size(11.0)
                                                .color(theme::AppColors::TEXT_DISABLED)
                                        );
                                    }
                                );
                            }
                            ui.end_row();
                            
                            // Rows 2-7: Date cells
                            for week in 0..6 {
                                ui.label("");  // Blank column
                                
                                for weekday in 0..7 {
                                    let day_offset = (week * 7 + weekday) as i32 - start_weekday as i32;
                                    
                                    let (date, is_current_month) = if day_offset < 0 {
                                        // Previous month
                                        let day = prev_month_last_day as i32 + day_offset + 1;
                                        (Some(prev_month.with_day(day as u32).unwrap()), false)
                                    } else if day_offset < last_day as i32 {
                                        // Current month
                                        (Some(self.current_month.with_day((day_offset + 1) as u32).unwrap()), true)
                                    } else {
                                        // Next month
                                        let next_month = self.next_month(self.current_month);
                                        let day = day_offset - last_day as i32 + 1;
                                        (Some(next_month.with_day(day as u32).unwrap()), false)
                                    };
                                    
                                    if let Some(date) = date {
                                        let is_selected = self.selected_date == Some(date);
                                        let is_today = date == Local::now().naive_local().date();
                                        
                                        let button_color = if is_selected {
                                            theme::AppColors::CYAN
                                        } else if is_today {
                                            Color32::from_rgb(220, 53, 69)
                                        } else {
                                            Color32::TRANSPARENT
                                        };
                                        
                                        let text_color = if is_selected || is_today {
                                            Color32::WHITE
                                        } else if is_current_month {
                                            theme::AppColors::TEXT_PRIMARY
                                        } else {
                                            theme::AppColors::TEXT_DISABLED
                                        };
                                        
                                        if ui.add(
                                            egui::Button::new(
                                                RichText::new(format!("{}", date.day()))
                                                    .color(text_color)
                                                    .size(13.0)
                                            )
                                            .fill(button_color)
                                            .min_size(Vec2::new(cell_size, cell_size))
                                            .frame(false)
                                        ).clicked() {
                                            self.selected_date = Some(date);
                                            if !is_current_month {
                                                self.current_month = date.with_day(1).unwrap();
                                            }
                                        }
                                    }
                                }
                                ui.end_row();
                            }
                        });
                });

    }
    
    fn show_time_selector(&mut self, ui: &mut Ui) {
        egui::ScrollArea::vertical()
            .id_source("time_selector_scroll")
            .max_height(400.0)
            .show(ui, |ui| {
                let has_date = self.selected_date.is_some();
                
                for time in &self.available_times.clone() {
                    let button_color = if has_date {
                        theme::AppColors::SURFACE_LIGHT
                    } else {
                        Color32::from_rgb(30, 41, 59) // Darker/disabled color
                    };
                    
                    let text_color = if has_date {
                        Color32::WHITE
                    } else {
                        theme::AppColors::TEXT_DISABLED
                    };
                    
                    let button = egui::Button::new(
                        RichText::new(time)
                            .size(15.0)
                            .color(text_color)
                    )
                    .fill(button_color)
                    .min_size(Vec2::new(200.0, 36.0));
                    
                    if ui.add_enabled(has_date, button).clicked() {
                        if let Some(date) = self.selected_date {
                            // Add new entry
                            self.config.entries.push(StartTimeEntry {
                                date: date.format("%m-%d-%Y").to_string(),
                                time: time.clone(),
                            });
                            self.save_config();
                            
                            // Clear selection
                            self.selected_date = None;
                        }
                    }
                    
                    ui.add_space(2.0);
                }
            });
    }
    
    fn prev_month(&self, date: NaiveDate) -> NaiveDate {
        if date.month() == 1 {
            NaiveDate::from_ymd_opt(date.year() - 1, 12, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(date.year(), date.month() - 1, 1).unwrap()
        }
    }
    
    fn next_month(&self, date: NaiveDate) -> NaiveDate {
        if date.month() == 12 {
            NaiveDate::from_ymd_opt(date.year() + 1, 1, 1).unwrap()
        } else {
            NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1).unwrap()
        }
    }
    
    fn last_day_of_month(&self, date: NaiveDate) -> u32 {
        let next_month = self.next_month(date);
        next_month.pred_opt().unwrap().day()
    }
}
