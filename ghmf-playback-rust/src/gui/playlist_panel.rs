use super::theme;
use crate::audio::AudioDecoder;
use egui::{Ui, RichText, Color32, ScrollArea, Vec2, Stroke, Frame};
use std::path::PathBuf;
use chrono::{Datelike, NaiveDate, Local};
use serde::{Serialize, Deserialize};
use std::fs;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Song {
    pub title: String,
    pub path: PathBuf,
    pub duration_secs: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub date: NaiveDate,
    pub theme: String,
    pub songs: Vec<Song>,
}

impl Playlist {
    pub fn total_duration(&self) -> u32 {
        self.songs.iter().map(|s| s.duration_secs).sum()
    }
    
    pub fn format_duration(secs: u32) -> String {
        let minutes = secs / 60;
        let seconds = secs % 60;
        format!("{:02}:{:02}", minutes, seconds)
    }
    
    pub fn save_to_file(&self, playlist_folder: &str) -> Result<(), std::io::Error> {
        // Create playlist folder if it doesn't exist
        let folder = shellexpand::tilde(playlist_folder).to_string();
        fs::create_dir_all(&folder)?;
        
        // Generate filename: YYYY-MM-DD_Theme.playlist
        let safe_theme = self.theme.replace(" ", "_").replace("/", "-");
        let filename = format!("{}_{}_{}.playlist", 
            self.date.format("%Y-%m-%d"),
            safe_theme,
            self.name.replace(" ", "_")
        );
        let filepath = PathBuf::from(&folder).join(filename);
        
        // Serialize to JSON
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filepath, json)?;
        
        Ok(())
    }
    
    pub fn load_from_file(filepath: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(filepath)?;
        let playlist: Playlist = serde_json::from_str(&content)?;
        Ok(playlist)
    }
    
    pub fn load_all_from_folder(playlist_folder: &str) -> Vec<Playlist> {
        let folder = shellexpand::tilde(playlist_folder).to_string();
        let mut playlists = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("playlist") {
                    if let Ok(playlist) = Playlist::load_from_file(&path) {
                        playlists.push(playlist);
                    }
                }
            }
        }
        
        // Sort by date descending
        playlists.sort_by(|a, b| b.date.cmp(&a.date));
        playlists
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SongFolder {
    Production,
    Testing,
    Events,
    Drone,
    OpenClose,
}

impl SongFolder {
    pub fn as_str(&self) -> &str {
        match self {
            SongFolder::Production => "Production",
            SongFolder::Testing => "Testing",
            SongFolder::Events => "Events",
            SongFolder::Drone => "Drone",
            SongFolder::OpenClose => "Open-Close",
        }
    }
}

pub struct PlaylistPanel {
    // Calendar
    selected_date: NaiveDate,
    current_month: NaiveDate,
    
    // Song selection
    search_query: String,
    selected_folder: SongFolder,
    available_songs: Vec<Song>,
    
    // Playlist being created
    playlist_songs: Vec<Song>,
    playlist_name: String,
    playlist_theme: String,
    theme_filter_char: Option<char>,
    dragging_index: Option<usize>,
    
    // Existing playlists
    saved_playlists: Vec<Playlist>,
    playlist_folder: String,
    editing_index: Option<usize>,
    
    // Open-Close folder path
    open_close_folder: String,
    has_closing_song: bool,
    
    // View playlist popup
    viewing_playlist: Option<usize>,
}

impl Default for PlaylistPanel {
    fn default() -> Self {
        let today = Local::now().naive_local().date();
        Self {
            selected_date: today,
            current_month: today.with_day(1).unwrap(),
            search_query: String::new(),
            selected_folder: SongFolder::Production,
            available_songs: Vec::new(),
            playlist_songs: Vec::new(),
            playlist_name: String::new(),
            playlist_theme: String::from("Other"),
            theme_filter_char: None,
            dragging_index: None,
            saved_playlists: Vec::new(),
            playlist_folder: String::new(),
            editing_index: None,
            open_close_folder: String::new(),
            has_closing_song: false,
            viewing_playlist: None,
        }
    }
}

impl PlaylistPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn show(&mut self, ui: &mut Ui, production_folder: &str, testing_folder: &str, events_folder: &str, drone_folder: &str, open_close_folder: &str, playlist_folder: &str) {
        // Show popup if viewing a playlist
        if let Some(idx) = self.viewing_playlist {
            if idx < self.saved_playlists.len() {
                let playlist = &self.saved_playlists[idx];
                let mut close_popup = false;
                
                egui::Window::new(format!("Playlist: {} - {}", playlist.date.format("%m-%d-%Y"), playlist.theme))
                    .collapsible(false)
                    .resizable(false)
                    .default_width(400.0)
                    .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                    .show(ui.ctx(), |ui| {
                        ui.set_width(400.0);
                        
                        ui.label(
                            RichText::new(format!("Total Duration: {}", Playlist::format_duration(playlist.total_duration())))
                                .size(14.0)
                                .color(theme::AppColors::TEXT_SECONDARY)
                        );
                        
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        
                        ScrollArea::vertical()
                            .max_height(400.0)
                            .show(ui, |ui| {
                                for (i, song) in playlist.songs.iter().enumerate() {
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            RichText::new(format!("{}.", i + 1))
                                                .size(12.0)
                                                .color(theme::AppColors::TEXT_DISABLED)
                                        );
                                        
                                        ui.label(
                                            RichText::new(&song.title)
                                                .size(13.0)
                                                .color(Color32::WHITE)
                                        );
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(
                                                RichText::new(Playlist::format_duration(song.duration_secs))
                                                    .size(12.0)
                                                    .color(theme::AppColors::TEXT_SECONDARY)
                                            );
                                        });
                                    });
                                }
                            });
                        
                        ui.add_space(10.0);
                        
                        if ui.button("Close").clicked() {
                            close_popup = true;
                        }
                    });
                
                if close_popup {
                    self.viewing_playlist = None;
                }
            } else {
                self.viewing_playlist = None;
            }
        }
        
        ui.heading(RichText::new("Playlist Management").size(24.0).color(Color32::WHITE));
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Store folder paths
        self.open_close_folder = open_close_folder.to_string();
        
        // Load songs when folder changes
        self.load_songs_from_folder(production_folder, testing_folder, events_folder, drone_folder, open_close_folder);
        
        // Load playlists when folder changes
        if self.playlist_folder != playlist_folder {
            self.playlist_folder = playlist_folder.to_string();
            self.saved_playlists = Playlist::load_all_from_folder(playlist_folder);
        }
        
        ui.horizontal(|ui| {
            // Left column - Calendar + Saved playlists
            ui.vertical(|ui| {
                self.show_calendar(ui);
                ui.add_space(20.0);
                self.show_saved_playlists(ui);
            });
            
            ui.add_space(30.0);
            
            // Vertical separator
            ui.separator();
            
            ui.add_space(30.0);
            
            // Right column - Available songs and selected songs
            ui.vertical(|ui| {
                ui.set_min_width(380.0);
                ui.set_max_width(380.0);
                
                self.show_available_songs(ui, production_folder, testing_folder, events_folder, drone_folder, open_close_folder);
                ui.add_space(15.0);
                self.show_selected_songs(ui);
            });
        });
    }
    
    fn show_calendar(&mut self, ui: &mut Ui) {
        ui.set_max_width(230.0);
        
        Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(6.0)
            .inner_margin(egui::Margin::symmetric(10.0, 8.0))
            .show(ui, |ui| {
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
                
                ui.add_space(6.0);
                
                // Calendar data
                let first_day = self.current_month.with_day(1).unwrap();
                let last_day = self.last_day_of_month(self.current_month);
                let start_weekday = first_day.weekday().num_days_from_sunday();
                let prev_month = self.prev_month(self.current_month);
                let prev_month_last_day = self.last_day_of_month(prev_month);
                
                let cell_size = 28.0; // Consistent cell size for perfect grid
                
                // 8x7 Grid: 1 blank column + 7 date columns
                egui::Grid::new("calendar_grid")
                    .spacing([2.0, 2.0])
                .min_col_width(cell_size)
                .max_col_width(cell_size)
                .show(ui, |ui| {
                        // Row 1: Day headers (1 blank + 7 days)
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
                        
                        // Rows 2-7: Date cells (1 blank + 7 dates)
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
                                    let is_selected = date == self.selected_date;
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
                                        self.selected_date = date;
                                        self.playlist_name = date.format("%m-%d-%Y").to_string();
                                        if !is_current_month {
                                            self.current_month = date.with_day(1).unwrap();
                                        }
                                        // Auto-add opening and closing songs
                                        self.auto_add_opening_and_closing();
                                    }
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
    
    fn show_selected_songs(&mut self, ui: &mut Ui) {
        Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_height(420.0); // Fixed height
                
                ui.label(
                    RichText::new(format!("Show Date: {}", self.selected_date.format("%m-%d-%Y")))
                        .size(14.0)
                        .color(theme::AppColors::TEXT_SECONDARY)
                );
                
                ui.add_space(10.0);
                
                ScrollArea::vertical()
                    .id_source("selected_songs_scroll")
                    .max_height(340.0)
                    .show(ui, |ui| {
                        let mut songs_to_remove: Vec<String> = Vec::new();
                        
                        egui_dnd::dnd(ui, "playlist_dnd").show_vec(&mut self.playlist_songs, |ui, song, handle, _state| {
                            ui.horizontal(|ui| {
                                // Drag handle
                                handle.ui(ui, |ui| {
                                    ui.label(RichText::new("☰").color(theme::AppColors::TEXT_DISABLED));
                                });
                                
                                // Song info
                                ui.vertical(|ui| {
                                    ui.label(
                                        RichText::new(&song.title)
                                            .size(13.0)
                                            .color(Color32::WHITE)
                                    );
                                });
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(25.0); // Match spacing with available songs
                                    
                                    // Remove button
                                    if ui.add(egui::Button::new("🗑").frame(false)).clicked() {
                                        songs_to_remove.push(song.title.clone());
                                    }
                                    
                                    ui.label(
                                        RichText::new(Playlist::format_duration(song.duration_secs))
                                            .size(12.0)
                                            .color(theme::AppColors::TEXT_SECONDARY)
                                    );
                                });
                            });
                        });
                        
                        // Remove songs after iteration (remove first occurrence only)
                        for title in songs_to_remove {
                            if let Some(pos) = self.playlist_songs.iter().position(|s| s.title == title) {
                                self.playlist_songs.remove(pos);
                            }
                        }
                    });
                
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);
                
                // Total duration
                let total_secs = self.playlist_songs.iter().map(|s| s.duration_secs).sum::<u32>();
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new("Total Show Time:")
                            .size(13.0)
                            .strong()
                            .color(Color32::WHITE)
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(Playlist::format_duration(total_secs))
                                .size(13.0)
                                .strong()
                                .color(theme::AppColors::CYAN)
                        );
                    });
                });
                
                ui.add_space(10.0);
                
                // Theme dropdown
                ui.label(
                    RichText::new("Theme:")
                        .size(12.0)
                        .color(theme::AppColors::TEXT_SECONDARY)
                );
                ui.add_space(5.0);
                
                // Style the ComboBox to match the create button
                let available_width = ui.available_width();
                ui.visuals_mut().widgets.inactive.bg_fill = theme::AppColors::SURFACE_LIGHT;
                ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::same(8.0);
                ui.visuals_mut().widgets.hovered.bg_fill = theme::AppColors::SURFACE_LIGHT;
                ui.visuals_mut().widgets.hovered.rounding = egui::Rounding::same(8.0);
                ui.visuals_mut().widgets.active.bg_fill = theme::AppColors::SURFACE_LIGHT;
                ui.visuals_mut().widgets.active.rounding = egui::Rounding::same(8.0);
                
                egui::ComboBox::from_id_source("theme_combo")
                    .selected_text(&self.playlist_theme)
                    .width(available_width)
                    .show_ui(ui, |ui| {
                        // All themes
                        let mut themes = vec![
                            "1950s Night", "1960s Night", "1970s Night", "1980s Night", "1990s Night",
                            "2000s Night", "2020 - Music From This Deca...", "Abba", 
                            "Additions From The Committee", "All About Food", "All About The Weather",
                            "Animated Movie Night", "Around The World", "Beach Boys", "Boy Bands",
                            "British Invasion", "Broadway Night", "Christmas in July Collaborations",
                            "Country Night", "Dance Dance Dance", "Duo's and Trios", 
                            "From Sea to Shining Sea", "Hair Bands", "James Bond", "Jimmy Buffett",
                            "John Williams", "K-Pop", "Ladies Night", "Land Down Under",
                            "Live From Grand Haven", "Motown", "Movie Night", "Name That Tune",
                            "New Songs", "New Wave", "No Lyrics Necessary", "Oh Canada!",
                            "Pink Floyd", "Places", "Punk Night", "Rainbow Mix Tape",
                            "Rythm & Blues", "SciFi Night", "Special Event", 
                            "Super Hero / Marvel Night", "Taylor Swift", "The Muppets",
                            "Throwback - 00 Updates", "Throwback - 90s Update", 
                            "Tunes From Televisions", "Other"
                        ];
                        
                        // Handle keyboard input for quick navigation
                        ui.input(|i| {
                            for event in &i.events {
                                if let egui::Event::Text(text) = event {
                                    if let Some(search_char) = text.chars().next() {
                                        self.theme_filter_char = Some(search_char.to_lowercase().next().unwrap());
                                    }
                                }
                            }
                        });
                        
                        // Sort themes: matching filter char first, then rest
                        if let Some(filter_char) = self.theme_filter_char {
                            themes.sort_by_key(|theme| {
                                let first_char = theme.chars().next().unwrap_or(' ').to_lowercase().next().unwrap();
                                if first_char == filter_char {
                                    0 // Matching themes first
                                } else {
                                    1 // Everything else after
                                }
                            });
                        }
                        
                        // Display all themes in sorted order
                        for theme in themes {
                            ui.selectable_value(&mut self.playlist_theme, theme.to_string(), theme);
                        }
                    });
                
                ui.add_space(10.0);
                
                // Create/Update button
                let button_text = if self.editing_index.is_some() {
                    "💾 Save Changes"
                } else {
                    "+ Create Playlist"
                };
                
                let create_button = egui::Button::new(
                    RichText::new(button_text)
                        .size(14.0)
                        .color(Color32::WHITE)
                )
                .fill(theme::AppColors::SURFACE_LIGHT)
                .min_size(Vec2::new(ui.available_width(), 36.0))
                .rounding(8.0);
                
                if ui.add(create_button).clicked() && !self.playlist_songs.is_empty() {
                    // Ensure playlist name is set
                    if self.playlist_name.is_empty() {
                        self.playlist_name = self.selected_date.format("%m-%d-%Y").to_string();
                    }
                    self.create_or_update_playlist();
                }
            });
    }
    
    fn show_search_and_filter(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Search box
            ui.add_sized(
                [200.0, 24.0],
                egui::TextEdit::singleline(&mut self.search_query)
                    .hint_text("Search")
            );
            
            ui.add_space(8.0);
            
            // Sort dropdown
            egui::ComboBox::from_label("")
                .selected_text(self.selected_folder.as_str())
                .width(120.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected_folder, SongFolder::Production, "Production");
                    ui.selectable_value(&mut self.selected_folder, SongFolder::Testing, "Testing");
                    ui.selectable_value(&mut self.selected_folder, SongFolder::Events, "Events");
                    ui.selectable_value(&mut self.selected_folder, SongFolder::Drone, "Drone");
                    ui.selectable_value(&mut self.selected_folder, SongFolder::OpenClose, "Open-Close");
                });
        });
    }
    
    fn show_available_songs(&mut self, ui: &mut Ui, production_folder: &str, testing_folder: &str, events_folder: &str, drone_folder: &str, open_close_folder: &str) {
        Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_height(230.0); // Fixed height (reduced by 75px, then increased by 25px)
                
                ui.label(
                    RichText::new("Available Songs")
                        .size(16.0)
                        .strong()
                        .color(theme::AppColors::CYAN)
                );
                
                ui.add_space(10.0);
                
                // Search and filter - align at top
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {
                    ui.spacing_mut().interact_size.y = 23.0;
                    
                    // Search box
                    ui.add_sized(
                        [200.0, 33.0],
                        egui::TextEdit::singleline(&mut self.search_query)
                            .hint_text("Search")
                    );
                    
                    ui.add_space(8.0);
                    
                    // Filter dropdown
                    egui::ComboBox::from_label("")
                        .selected_text(self.selected_folder.as_str())
                        .width(120.0)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.selected_folder, SongFolder::Production, "Production");
                            ui.selectable_value(&mut self.selected_folder, SongFolder::Testing, "Testing");
                            ui.selectable_value(&mut self.selected_folder, SongFolder::Events, "Events");
                            ui.selectable_value(&mut self.selected_folder, SongFolder::Drone, "Drone");
                            ui.selectable_value(&mut self.selected_folder, SongFolder::OpenClose, "Open-Close");
                        });
                });
                
                ui.add_space(10.0);
                
                ScrollArea::vertical()
                    .id_source("available_songs_scroll")
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for song in &self.available_songs {
                            if !self.search_query.is_empty() && 
                               !song.title.to_lowercase().contains(&self.search_query.to_lowercase()) {
                                continue;
                            }
                            
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(&song.title)
                                        .size(14.0)
                                        .color(Color32::WHITE)
                                );
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.add_space(25.0);  // Add space to avoid scrollbar covering the icon
                                    if ui.add(egui::Button::new(RichText::new("➕").size(14.0)).frame(false)).clicked() {
                                        // Insert before closing song if it exists
                                        if self.has_closing_song && !self.playlist_songs.is_empty() {
                                            let insert_pos = self.playlist_songs.len() - 1;
                                            self.playlist_songs.insert(insert_pos, song.clone());
                                        } else {
                                            self.playlist_songs.push(song.clone());
                                        }
                                        self.search_query.clear();
                                    }
                                });
                            });
                        }
                    });
            });
    }
    
    fn show_saved_playlists(&mut self, ui: &mut Ui) {
        ui.set_width(230.0);
        
        Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(12.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.set_height(480.0); // Fixed height
                
                ui.label(
                    RichText::new("Saved Playlists")
                        .size(16.0)
                        .strong()
                        .color(theme::AppColors::CYAN)
                );
                
                ui.add_space(10.0);
                
                if self.saved_playlists.is_empty() {
                    ui.label(
                        RichText::new("No playlists available")
                            .size(12.0)
                            .color(theme::AppColors::TEXT_SECONDARY)
                    );
                } else {
                    ScrollArea::vertical()
                        .id_source("saved_playlists_scroll")
                        .max_height(400.0)
                        .show(ui, |ui| {
                            let mut to_delete = None;
                            let mut to_edit = None;
                            
                            let mut to_view = None;
                            
                            for (idx, playlist) in self.saved_playlists.iter().enumerate() {
                                ui.horizontal_top(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            RichText::new(playlist.date.format("%m-%d-%Y").to_string())
                                                .size(14.0)
                                                .color(Color32::WHITE)
                                        );
                                        ui.horizontal(|ui| {
                                            ui.label(
                                                RichText::new(format!("{} • {}", 
                                                    playlist.theme,
                                                    Playlist::format_duration(playlist.total_duration())))
                                                    .size(13.0)
                                                    .color(Color32::WHITE)
                                            );
                                            
                                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                                ui.add_space(15.0); // Add space for scrollbar
                                                if ui.add(egui::Button::new("🗑").frame(false)).clicked() {
                                                    to_delete = Some(idx);
                                                }
                                                ui.add_space(10.0);
                                                if ui.add(egui::Button::new("✏").frame(false)).clicked() {
                                                    to_edit = Some(idx);
                                                }
                                                ui.add_space(10.0);
                                                if ui.add(egui::Button::new("👁").frame(false)).clicked() {
                                                    to_view = Some(idx);
                                                }
                                            });
                                        });
                                    });
                                });
                                
                                ui.separator();
                            }
                            
                            if let Some(idx) = to_delete {
                                let playlist = &self.saved_playlists[idx];
                                
                                // Delete JSON file
                                let folder = shellexpand::tilde(&self.playlist_folder).to_string();
                                let safe_theme = playlist.theme.replace(" ", "_").replace("/", "-");
                                let filename = format!("{}_{}_{}. playlist", 
                                    playlist.date.format("%Y-%m-%d"),
                                    safe_theme,
                                    playlist.name.replace(" ", "_")
                                );
                                let filepath = PathBuf::from(&folder).join(filename);
                                
                                if filepath.exists() {
                                    if let Err(e) = fs::remove_file(&filepath) {
                                        eprintln!("Failed to delete playlist file: {}", e);
                                    }
                                }
                                
                                self.saved_playlists.remove(idx);
                            }
                            
                            if let Some(idx) = to_edit {
                                let playlist = self.saved_playlists[idx].clone();
                                self.selected_date = playlist.date;
                                self.playlist_name = playlist.name;
                                self.playlist_theme = playlist.theme;
                                self.playlist_songs = playlist.songs;
                                self.editing_index = Some(idx);
                            }
                            
                            if let Some(idx) = to_view {
                                self.viewing_playlist = Some(idx);
                            }
                        });
                }
            });
    }
    
    fn load_songs_from_folder(&mut self, production_folder: &str, testing_folder: &str, events_folder: &str, drone_folder: &str, open_close_folder: &str) {
        let folder = match self.selected_folder {
            SongFolder::Production => production_folder,
            SongFolder::Testing => testing_folder,
            SongFolder::Events => events_folder,
            SongFolder::Drone => drone_folder,
            SongFolder::OpenClose => open_close_folder,
        };
        
        // Clear and reload songs
        self.available_songs.clear();
        
        // Scan folder for .ctl files
        if let Ok(entries) = std::fs::read_dir(folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("ctl") {
                    if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                        // Try to find corresponding audio file and get its duration
                        let mut duration_secs = 180; // Default 3 minutes
                        
                        // Check for common audio extensions
                        for ext in &["wav", "mp3", "flac", "ogg"] {
                            let audio_path = path.with_extension(ext);
                            if audio_path.exists() {
                                if let Ok(duration) = AudioDecoder::get_duration(audio_path.to_str().unwrap_or("")) {
                                    duration_secs = duration.as_secs() as u32;
                                    break;
                                }
                            }
                        }
                        
                        self.available_songs.push(Song {
                            title: file_name.to_string(),
                            path: path.clone(),
                            duration_secs,
                        });
                    }
                }
            }
        }
        
        // Sort songs alphabetically
        self.available_songs.sort_by(|a, b| a.title.cmp(&b.title));
    }
    
    fn auto_add_opening_and_closing(&mut self) {
        // Clear current playlist
        self.playlist_songs.clear();
        self.has_closing_song = false;
        
        let folder = shellexpand::tilde(&self.open_close_folder).to_string();
        let mut opening: Option<Song> = None;
        let mut closing: Option<Song> = None;
        
        if let Ok(entries) = fs::read_dir(&folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if ext_lower == "mp3" || ext_lower == "wav" || ext_lower == "flac" || ext_lower == "ogg" {
                        if let Some(filename) = path.file_stem() {
                            let filename_lower = filename.to_string_lossy().to_lowercase();
                            
                            // Look for opening song
                            if filename_lower.contains("opening") {
                                if let Ok(duration) = AudioDecoder::get_duration(&path.to_string_lossy()) {
                                    opening = Some(Song {
                                        title: filename.to_string_lossy().to_string(),
                                        path: path.clone(),
                                        duration_secs: duration.as_secs() as u32,
                                    });
                                }
                            }
                            
                            // Look for closing song
                            if filename_lower.contains("closing") {
                                if let Ok(duration) = AudioDecoder::get_duration(&path.to_string_lossy()) {
                                    closing = Some(Song {
                                        title: filename.to_string_lossy().to_string(),
                                        path: path.clone(),
                                        duration_secs: duration.as_secs() as u32,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Add opening first
        if let Some(song) = opening {
            self.playlist_songs.push(song);
        }
        
        // Add closing last
        if let Some(song) = closing {
            self.playlist_songs.push(song);
            self.has_closing_song = true;
        }
    }
    
    fn create_or_update_playlist(&mut self) {
        let playlist = Playlist {
            name: self.playlist_name.clone(),
            date: self.selected_date,
            theme: self.playlist_theme.clone(),
            songs: self.playlist_songs.clone(),
        };
        
        if let Some(idx) = self.editing_index {
            // Update existing playlist
            if idx < self.saved_playlists.len() {
                // Delete old file first
                let old_playlist = &self.saved_playlists[idx];
                let folder = shellexpand::tilde(&self.playlist_folder).to_string();
                let safe_theme = old_playlist.theme.replace(" ", "_").replace("/", "-");
                let filename = format!("{}_{}_{}. playlist", 
                    old_playlist.date.format("%Y-%m-%d"),
                    safe_theme,
                    old_playlist.name.replace(" ", "_")
                );
                let filepath = PathBuf::from(&folder).join(filename);
                if filepath.exists() {
                    let _ = fs::remove_file(&filepath);
                }
                
                // Save new version
                if let Err(e) = playlist.save_to_file(&self.playlist_folder) {
                    eprintln!("Failed to save playlist: {}", e);
                }
                
                // Update in memory
                self.saved_playlists[idx] = playlist;
                self.saved_playlists.sort_by(|a, b| b.date.cmp(&a.date));
            }
            self.editing_index = None;
        } else {
            // Create new playlist
            if let Err(e) = playlist.save_to_file(&self.playlist_folder) {
                eprintln!("Failed to save playlist: {}", e);
            }
            
            self.saved_playlists.push(playlist);
            self.saved_playlists.sort_by(|a, b| b.date.cmp(&a.date));
        }
        
        // Clear the current playlist
        self.playlist_songs.clear();
        self.playlist_name.clear();
        self.playlist_theme = String::from("Other");
        self.has_closing_song = false;
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
