use egui::{Context, Ui, Color32, Stroke, RichText, TextureHandle, ColorImage, Rect, Pos2, Vec2, Sense};
use egui_extras::{Size, StripBuilder, TableBuilder, Column};
use egui_plot::{Plot, Line, PlotPoints, Legend};
use crate::gui::theme;
use crate::gui::playback_panel::{self, PlaybackPanelState};
use crate::gui::procedures_panel::ProcedureEntry;
use crate::audio::AudioPlayer;
use crate::lighting::FixtureManager;
use std::time::Duration;
use chrono::{Local, Timelike, NaiveTime, Datelike};
use serde::{Deserialize, Serialize};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::fs;
use tracing::{info, warn};

/// Represents a song in the playlist
#[derive(Clone, Debug)]
pub struct PlaylistSong {
    pub title: String,
    pub path: PathBuf,
    pub duration: Duration,
    pub is_opening: bool,
    pub is_ending: bool,
}

/// Weather information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeatherInfo {
    pub conditions: String,
    pub temperature: i32,
    pub wind_speed: i32,
    pub hourly_forecast: Vec<HourlyForecast>,
}

/// Hourly forecast entry
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HourlyForecast {
    pub time: String,
    pub temperature: i32,
    pub short_forecast: String,
    pub precipitation_chance: Option<u32>,
}

/// Schedule/Procedure information
#[derive(Clone, Debug)]
pub struct ProcedureInfo {
    pub name: String,
    pub time_until: Duration,
}

/// Announcement file info
#[derive(Clone, Debug)]
pub struct AnnouncementFile {
    pub name: String,
    pub path: String,
}

/// PLC output log entry
#[derive(Clone, Debug)]
pub struct PlcLogEntry {
    pub timestamp: Duration,
    pub ctl_codes: Vec<String>,
}

/// DMX fixture state
#[derive(Clone, Debug)]
pub struct DmxFixtureState {
    pub fixture_number: u16,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Fixture information for grid display
#[derive(Clone, Debug)]
struct FixtureInfo {
    id: u32,
    name: String,
}

/// Lights layout configuration (matches lights_layout_panel)
#[derive(Debug, Clone, Serialize, Deserialize)]
struct LightsLayout {
    /// Map of "row_col" -> fixture_id
    pub cells: std::collections::HashMap<String, u32>,
}

impl Default for LightsLayout {
    fn default() -> Self {
        Self {
            cells: std::collections::HashMap::new(),
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
                Self::default()
            }
        }
    }
    
    fn get_fixture_at(&self, row: usize, col: usize) -> Option<u32> {
        let key = format!("{}_{}", row, col);
        self.cells.get(&key).copied()
    }
}

/// Playback state information
#[derive(Clone, Debug)]
pub struct PlaybackState {
    pub current_song: Option<String>,
    pub current_position: Duration,
    pub total_duration: Duration,
    pub is_playing: bool,
    pub waveform_data: Vec<f32>, // Audio waveform amplitudes for visualization
}

/// Main operator panel state
pub struct OperatorPanel {
    // Playback state
    pub playback: PlaybackState,
    
    // Volume controls
    pub volume_left: f32,   // Controllable, starts at 35
    
    // Schedule information
    pub show_start_time: String, // HH:MM PM format
    pub next_procedure: Option<ProcedureInfo>,
    
    // Weather
    pub weather: WeatherInfo,
    pub last_weather_update: std::time::Instant,
    pub weather_update_interval: std::time::Duration, // Update every minute
    pub weather_rx: Receiver<WeatherInfo>,
    pub weather_tx: Sender<WeatherInfo>,
    pub weather_fetch_pending: bool,
    
    // Playlist
    pub current_playlist: Vec<PlaylistSong>,
    pub current_playlist_date: Option<String>,
    pub current_playlist_theme: Option<String>,
    pub current_playlist_type: Option<String>, // "Pre-Show" or "Playlist" or "Testing"
    pub current_song_index: usize,
    pub total_runtime: Duration,
    pub remaining_time: Duration,
    
    // Announcements
    pub available_announcements: Vec<AnnouncementFile>,
    pub show_announcement_popup: bool,
    pub selected_announcement: Option<usize>,
    
    // PLC Output
    pub plc_log: Vec<PlcLogEntry>,
    pub max_plc_log_entries: usize,
    
    // DMX Output
    pub dmx_fixtures: Vec<DmxFixtureState>,
    pub available_fixtures: Vec<FixtureInfo>,
    pub lights_layout: LightsLayout,
    
    // Start Show With selector
    pub available_playlists: Vec<String>,
    pub selected_playlist_index: usize,
    
    // Weather icons
    rain_icon: Option<Arc<TextureHandle>>,
    right_arrow_icon: Option<Arc<TextureHandle>>,
    down_arrow_icon: Option<Arc<TextureHandle>>,
    temp_icon: Option<Arc<TextureHandle>>,
    wind_icon: Option<Arc<TextureHandle>>,
    list_icon: Option<Arc<TextureHandle>>,
    theme_icon: Option<Arc<TextureHandle>>,
}

impl Default for OperatorPanel {
    fn default() -> Self {
        let (weather_tx, weather_rx) = channel();
        
        Self {
            playback: PlaybackState {
                current_song: None,
                current_position: Duration::from_secs(0),
                total_duration: Duration::from_secs(0),
                is_playing: false,
                waveform_data: Vec::new(),
            },
            volume_left: 35.0,
            show_start_time: "7:00 PM".to_string(),
            next_procedure: None, // Will be updated from procedures panel
            weather: WeatherInfo {
                conditions: "Loading...".to_string(),
                temperature: 0,
                wind_speed: 0,
                hourly_forecast: Vec::new(),
            },
            last_weather_update: std::time::Instant::now(),
            weather_update_interval: std::time::Duration::from_secs(60), // Update every minute
            weather_rx,
            weather_tx,
            weather_fetch_pending: false,
            current_playlist: Vec::new(),
            current_playlist_date: None,
            current_playlist_theme: None,
            current_playlist_type: None,
            current_song_index: 0,
            total_runtime: Duration::from_secs(0),
            remaining_time: Duration::from_secs(0),
            available_announcements: Vec::new(),
            show_announcement_popup: false,
            selected_announcement: None,
            plc_log: Vec::new(),
            max_plc_log_entries: 50,
            dmx_fixtures: Vec::new(),
            available_fixtures: Vec::new(),
            lights_layout: LightsLayout::load(),
            available_playlists: vec![
                "Pre-Show".to_string(),
                "Playlist".to_string(),
                "Testing".to_string(),
            ],
            selected_playlist_index: 0,
            rain_icon: None,
            right_arrow_icon: None,
            down_arrow_icon: None,
            temp_icon: None,
            wind_icon: None,
            list_icon: None,
            theme_icon: None,
        }
    }
}

impl OperatorPanel {
    pub fn new() -> Self {
        let mut panel = Self::default();
        panel.load_pre_show_playlist();
        panel
    }
    
    /// Load Pre-Show playlist from the Music/Playlists folder
    pub fn load_pre_show_playlist(&mut self) {
        let today = Local::now().date_naive();
        let playlist_folder = shellexpand::tilde("~/Desktop/GHMF Playback 2.0/Music/Playlists").to_string();
        
        // Try to find Pre-Show playlist (date-independent, same every day)
        if let Ok(entries) = fs::read_dir(&playlist_folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("playlist") {
                    // Read and parse the playlist file
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(playlist) = serde_json::from_str::<crate::gui::playlist_panel::Playlist>(&content) {
                            // Check if this is Pre-Show playlist (no date check - same every day)
                            if playlist.theme == "Pre-Show" {
                                // Convert songs to PlaylistSong format
                                self.current_playlist = playlist.songs.iter().map(|song| {
                                    PlaylistSong {
                                        title: song.title.clone(),
                                        path: song.path.clone(),
                                        duration: Duration::from_secs(song.duration_secs as u64),
                                        is_opening: false,
                                        is_ending: false,
                                    }
                                }).collect();
                                
                                // Calculate total runtime
                                self.total_runtime = Duration::from_secs(
                                    playlist.songs.iter().map(|s| s.duration_secs as u64).sum()
                                );
                                self.remaining_time = self.total_runtime;
                                
                                // Store the date, theme, and type
                                self.current_playlist_date = Some(today.format("%m-%d-%Y").to_string());
                                self.current_playlist_theme = Some(playlist.theme.clone());
                                self.current_playlist_type = Some("Pre-Show".to_string());
                                
                                // Set to max value so first increment in get_next_song wraps to 0
                                self.current_song_index = usize::MAX;
                                
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Load Testing playlist from the Music/Playlists folder
    pub fn load_testing_playlist(&mut self) {
        let today = Local::now().date_naive();
        let playlist_folder = shellexpand::tilde("~/Desktop/GHMF Playback 2.0/Music/Playlists").to_string();
        
        // Try to find Testing playlist (date-independent, same every day)
        if let Ok(entries) = fs::read_dir(&playlist_folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("playlist") {
                    // Read and parse the playlist file
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(playlist) = serde_json::from_str::<crate::gui::playlist_panel::Playlist>(&content) {
                            // Check if this is Testing playlist (no date check - same every day)
                            if playlist.theme == "Testing" {
                                // Convert songs to PlaylistSong format
                                self.current_playlist = playlist.songs.iter().map(|song| {
                                    PlaylistSong {
                                        title: song.title.clone(),
                                        path: song.path.clone(),
                                        duration: Duration::from_secs(song.duration_secs as u64),
                                        is_opening: false,
                                        is_ending: false,
                                    }
                                }).collect();
                                
                                // Calculate total runtime
                                self.total_runtime = Duration::from_secs(
                                    playlist.songs.iter().map(|s| s.duration_secs as u64).sum()
                                );
                                self.remaining_time = self.total_runtime;
                                
                                // Store the date, theme, and type
                                self.current_playlist_date = Some(today.format("%m-%d-%Y").to_string());
                                self.current_playlist_theme = Some(playlist.theme.clone());
                                self.current_playlist_type = Some("Testing".to_string());
                                
                                // Set to max value so first increment in get_next_song wraps to 0
                                self.current_song_index = usize::MAX;
                                
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Load today's playlist from the Music/Playlists folder
    pub fn load_todays_playlist(&mut self, playlist_folder: &str) {
        let today = Local::now().date_naive();
        let playlist_folder = shellexpand::tilde(playlist_folder).to_string();
        
        // Clear current playlist before loading
        self.current_playlist.clear();
        self.total_runtime = Duration::from_secs(0);
        self.remaining_time = Duration::from_secs(0);
        self.current_playlist_date = None;
        self.current_playlist_theme = None;
        self.current_playlist_type = Some("Playlist".to_string());
        self.current_song_index = 0;
        
        // Try to find a playlist for today's date
        let mut found = false;
        if let Ok(entries) = fs::read_dir(&playlist_folder) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("playlist") {
                    // Read and parse the playlist file
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(playlist) = serde_json::from_str::<crate::gui::playlist_panel::Playlist>(&content) {
                            // Skip Testing and Pre-Show playlists - only load date-specific playlists
                            if playlist.theme == "Testing" || playlist.theme == "Pre-Show" {
                                continue;
                            }
                            
                            // Check if this playlist is for today
                            if playlist.date == today {
                                // Convert songs to PlaylistSong format
                                self.current_playlist = playlist.songs.iter().map(|song| {
                                    PlaylistSong {
                                        title: song.title.clone(),
                                        path: song.path.clone(),
                                        duration: Duration::from_secs(song.duration_secs as u64),
                                        is_opening: song.title == "Opening",
                                        is_ending: song.title == "Closing" || song.title == "Ending",
                                    }
                                }).collect();
                                
                                // Calculate total runtime
                                self.total_runtime = Duration::from_secs(
                                    playlist.songs.iter().map(|s| s.duration_secs as u64).sum()
                                );
                                self.remaining_time = self.total_runtime;
                                
                                // Store the date, theme, and type
                                self.current_playlist_date = Some(today.format("%m-%d-%Y").to_string());
                                self.current_playlist_theme = Some(playlist.theme.clone());
                                self.current_playlist_type = Some("Playlist".to_string());
                                
                                // Set to max value so first increment in get_next_song wraps to 0
                                self.current_song_index = usize::MAX;
                                
                                found = true;
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        // If no playlist found for today, add a placeholder message
        if !found {
            eprintln!("No playlist found for today's date: {}", today.format("%Y-%m-%d"));
        }
    }
    
    /// Get the first song path from the selected playlist type
    pub fn get_first_song_from_playlist(&self, playlist_type: &str) -> Option<PathBuf> {
        use std::fs;
        
        let base_path = PathBuf::from(shellexpand::tilde("Music").to_string());
        
        match playlist_type {
            "Pre-Show" => {
                // Load first song from Pre-Show folder
                let pre_show_path = base_path.join("Pre-Show");
                Self::get_first_audio_file_from_folder(&pre_show_path)
            }
            "Playlist" => {
                // Load first song from today's playlist in Playlists folder
                let playlist_folder = base_path.join("Playlists");
                let today = Local::now().date_naive();
                
                // Find today's playlist
                if let Ok(entries) = fs::read_dir(&playlist_folder) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("playlist") {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(playlist) = serde_json::from_str::<crate::gui::playlist_panel::Playlist>(&content) {
                                    if playlist.date == today && !playlist.songs.is_empty() {
                                        return Some(playlist.songs[0].path.clone());
                                    }
                                }
                            }
                        }
                    }
                }
                None
            }
            "Testing" => {
                // Load first song from Testing folder
                let testing_path = base_path.join("Testing");
                Self::get_first_audio_file_from_folder(&testing_path)
            }
            _ => None
        }
    }
    
    /// Helper to get first audio file from a folder
    fn get_first_audio_file_from_folder(folder: &PathBuf) -> Option<PathBuf> {
        use std::fs;
        
        if let Ok(entries) = fs::read_dir(folder) {
            let mut audio_files: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| {
                    p.extension()
                        .and_then(|s| s.to_str())
                        .map(|ext| matches!(ext, "wav" | "mp3" | "flac"))
                        .unwrap_or(false)
                })
                .collect();
            
            // Sort alphabetically
            audio_files.sort();
            
            // Return first file
            audio_files.into_iter().next()
        } else {
            None
        }
    }
    
    pub fn get_next_song_from_current_playlist(&mut self) -> Option<PathBuf> {
        // Check if we have a loaded playlist
        if self.current_playlist.is_empty() {
            return None;
        }
        
        // Move to next song in playlist (with wrapping for usize::MAX -> 0)
        self.current_song_index = self.current_song_index.wrapping_add(1);
        
        // Check if we've reached the end
        if self.current_song_index >= self.current_playlist.len() {
            // Reset to beginning but don't loop
            self.current_song_index = self.current_playlist.len() - 1;
            return None; // Don't loop, stop at end
        }
        
        // Get the next song path
        let song = &self.current_playlist[self.current_song_index];
        let mut path = song.path.clone();
        
        // If the path has .ctl extension, try to find corresponding .wav or .mp3
        if path.extension().and_then(|s| s.to_str()) == Some("ctl") {
            // Try .wav first
            path.set_extension("wav");
            if !path.exists() {
                // Try .mp3
                path.set_extension("mp3");
                if !path.exists() {
                    // If neither exists, try uppercase
                    path.set_extension("WAV");
                    if !path.exists() {
                        path.set_extension("MP3");
                    }
                }
            }
        }
        
        Some(path)
    }
    
    /// Jump to a specific song in the current playlist by index
    pub fn jump_to_song(&mut self, index: usize) -> Option<PathBuf> {
        // Check if we have a loaded playlist and the index is valid
        if self.current_playlist.is_empty() || index >= self.current_playlist.len() {
            return None;
        }
        
        // Set the current song index
        self.current_song_index = index;
        
        // Get the song path
        let song = &self.current_playlist[self.current_song_index];
        let mut path = song.path.clone();
        
        // If the path has .ctl extension, try to find corresponding .wav or .mp3
        if path.extension().and_then(|s| s.to_str()) == Some("ctl") {
            // Try .wav first
            path.set_extension("wav");
            if !path.exists() {
                // Try .mp3
                path.set_extension("mp3");
                if !path.exists() {
                    // If neither exists, try uppercase
                    path.set_extension("WAV");
                    if !path.exists() {
                        path.set_extension("MP3");
                    }
                }
            }
        }
        
        Some(path)
    }
    
    /// Update procedures based on show start time
    pub fn update_procedures(&mut self, procedures: &[ProcedureEntry], show_start_time: &str) {
        // Parse show start time (format: "7:00 PM")
        let show_time = Self::parse_show_time(show_start_time);
        let now = Local::now().time();
        
        // Calculate minutes until show starts
        let minutes_until_show = Self::minutes_between(now, show_time);
        
        // Determine which mode we're in based on time of day
        let current_hour = now.hour();
        let current_minute = now.minute();
        
        let mut next_proc = None;
        
        // From 12:00 AM until 8:00 PM show Idle Mode
        if current_hour < 20 {
            next_proc = Some(ProcedureInfo {
                name: "Idle Mode".to_string(),
                time_until: Duration::from_secs(0),
            });
        }
        // From 11:00 PM until 12:00 AM show Idle Mode
        else if current_hour >= 23 {
            next_proc = Some(ProcedureInfo {
                name: "Idle Mode".to_string(),
                time_until: Duration::from_secs(0),
            });
        }
        // From 8:00 PM until 20 minutes before show time: Perform Fountain Test
        else if minutes_until_show > 20 {
            next_proc = Some(ProcedureInfo {
                name: "Perform Fountain Test".to_string(),
                time_until: Duration::from_secs(0),
            });
        }
        // From 20 minutes before show time: follow procedures schedule
        else {
            // Find the next procedure based on minutes until show
            for proc in procedures {
                if minutes_until_show >= proc.minutes_before as i64 {
                    // This procedure should trigger
                    let minutes_diff = (minutes_until_show - proc.minutes_before as i64).abs();
                    next_proc = Some(ProcedureInfo {
                        name: proc.name.clone(),
                        time_until: Duration::from_secs(minutes_diff as u64 * 60),
                    });
                    break;
                }
            }
            
            // If no procedure found, default to Perform Fountain Test
            if next_proc.is_none() {
                next_proc = Some(ProcedureInfo {
                    name: "Perform Fountain Test".to_string(),
                    time_until: Duration::from_secs(0),
                });
            }
        }
        
        self.next_procedure = next_proc;
        self.show_start_time = show_start_time.to_string();
    }
    
    /// Fetch weather data from weather.gov API
    pub fn fetch_weather(&self) {
        let tx = self.weather_tx.clone();
        
        tokio::spawn(async move {
            match fetch_weather_data().await {
                Ok(new_weather) => {
                    let _ = tx.send(new_weather);
                }
                Err(e) => {
                    eprintln!("Failed to fetch weather: {}", e);
                }
            }
        });
    }
    
    /// Parse show start time string like "7:00 PM" into NaiveTime
    fn parse_show_time(time_str: &str) -> NaiveTime {
        // Try to parse various formats
        if let Ok(time) = NaiveTime::parse_from_str(time_str, "%I:%M %p") {
            return time;
        }
        if let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") {
            return time;
        }
        // Default to 7:00 PM if parsing fails
        NaiveTime::from_hms_opt(19, 0, 0).unwrap()
    }
    
    /// Calculate minutes between two times
    fn minutes_between(from: NaiveTime, to: NaiveTime) -> i64 {
        let from_secs = from.num_seconds_from_midnight() as i64;
        let to_secs = to.num_seconds_from_midnight() as i64;
        
        let mut diff = to_secs - from_secs;
        
        // Handle crossing midnight
        if diff < 0 {
            diff += 24 * 60 * 60;
        }
        
        diff / 60
    }
    
    /// Load rain icon texture
    fn load_rain_icon(&mut self, ctx: &Context) {
        if self.rain_icon.is_some() {
            return;
        }
        
        let icon_bytes = include_bytes!("../../assets/rain.png");
        if let Ok(image) = image::load_from_memory(icon_bytes) {
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "rain_icon",
                color_image,
                Default::default()
            );
            
            self.rain_icon = Some(Arc::new(texture));
        }
    }
    
    /// Load temp icon texture
    fn load_temp_icon(&mut self, ctx: &Context) {
        if self.temp_icon.is_some() {
            return;
        }
        
        let icon_bytes = include_bytes!("../../assets/temp.png");
        if let Ok(image) = image::load_from_memory(icon_bytes) {
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "temp_icon",
                color_image,
                Default::default()
            );
            
            self.temp_icon = Some(Arc::new(texture));
        }
    }
    
    /// Load wind icon texture
    fn load_wind_icon(&mut self, ctx: &Context) {
        if self.wind_icon.is_some() {
            return;
        }
        
        let icon_bytes = include_bytes!("../../assets/wind.png");
        if let Ok(image) = image::load_from_memory(icon_bytes) {
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "wind_icon",
                color_image,
                Default::default()
            );
            
            self.wind_icon = Some(Arc::new(texture));
        }
    }
    
    /// Load arrow icon textures
    fn load_arrow_icons(&mut self, ctx: &Context) {
        if self.right_arrow_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/right.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "right_arrow_icon",
                    color_image,
                    Default::default()
                );
                
                self.right_arrow_icon = Some(Arc::new(texture));
            }
        }
        
        if self.down_arrow_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/down.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "down_arrow_icon",
                    color_image,
                    Default::default()
                );
                
                self.down_arrow_icon = Some(Arc::new(texture));
            }
        }
    }
    
    /// Load list icon texture
    fn load_list_icon(&mut self, ctx: &Context) {
        if self.list_icon.is_some() {
            return;
        }
        
        let icon_bytes = include_bytes!("../../assets/list.png");
        if let Ok(image) = image::load_from_memory(icon_bytes) {
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "list_icon",
                color_image,
                Default::default()
            );
            
            self.list_icon = Some(Arc::new(texture));
        }
    }
    
    /// Load theme icon texture
    fn load_theme_icon(&mut self, ctx: &Context) {
        if self.theme_icon.is_some() {
            return;
        }
        
        let icon_bytes = include_bytes!("../../assets/theme.png");
        if let Ok(image) = image::load_from_memory(icon_bytes) {
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "theme_icon",
                color_image,
                Default::default()
            );
            
            self.theme_icon = Some(Arc::new(texture));
        }
    }
    
    /// Main UI render function
    /// Returns Some(playlist_type) if user selected a new playlist type to load
    pub fn show(&mut self, 
        ctx: &Context, 
        ui: &mut Ui,
        is_playing: &mut bool,
        is_paused: &mut bool,
        playback_position: Duration,
        playback_duration: Duration,
        current_song: &str,
        current_playlist: &str,
        audio_player: &Option<Arc<Mutex<AudioPlayer>>>,
        playback_panel_state: &mut PlaybackPanelState,
        current_song_path: &Option<PathBuf>,
        recent_commands: &[(u64, String)],
        fixture_manager: Option<&Arc<Mutex<FixtureManager>>>,
    ) -> (Option<String>, Option<usize>, bool) { // Added bool for step button
        // Load icons if not already loaded
        self.load_rain_icon(ctx);
        self.load_arrow_icons(ctx);
        self.load_temp_icon(ctx);
        self.load_wind_icon(ctx);
        self.load_list_icon(ctx);
        self.load_theme_icon(ctx);
        
        // Check for weather updates from async task
        if let Ok(new_weather) = self.weather_rx.try_recv() {
            self.weather = new_weather;
            self.weather_fetch_pending = false;
        }
        
        // Update weather if needed (every minute) and not already pending
        if !self.weather_fetch_pending && self.last_weather_update.elapsed() >= self.weather_update_interval {
            self.fetch_weather();
            self.weather_fetch_pending = true;
            self.last_weather_update = std::time::Instant::now();
        }
        
        // Request repaint to keep time updated
        ctx.request_repaint();
        
        let mut selected_playlist_type = None;
        let mut clicked_song_index = None;
        let mut step_clicked = false;
        
        // Main layout: horizontal split with sidebar on left, main content on right
        egui::SidePanel::left("operator_left_panel")
            .resizable(false)
            .exact_width(280.0)
            .show_inside(ui, |ui| {
                let (playlist, song_idx) = self.show_left_sidebar(ui, *is_playing, *is_paused);
                selected_playlist_type = playlist;
                clicked_song_index = song_idx;
            });
        
        egui::CentralPanel::default().show_inside(ui, |ui| {
            if self.show_main_content(ui, is_playing, is_paused, playback_position, playback_duration, 
                current_song, current_playlist, audio_player, playback_panel_state, current_song_path, recent_commands, fixture_manager) {
                step_clicked = true;
            }
        });
        
        // Announcement popup modal
        if self.show_announcement_popup {
            self.show_announcement_popup_window(ctx);
        }
        
        (selected_playlist_type, clicked_song_index, step_clicked)
    }
    
    /// Fetch weather data from weather.gov API for Grand Haven, MI 49417
    fn fetch_weather_async(&mut self) {
        // TODO: Implement async weather API call to weather.gov
        // For now, update with placeholder data
        // This will be implemented with tokio spawn and reqwest in the integration phase
    }
    
    /// Save weather log at show start time
    fn save_weather_log(&self) {
        // TODO: Write weather JSON file with date, time, weather info, playlist name, theme
    }
    
    /// Left sidebar with information cards
    /// Returns (Some(playlist_type), Some(song_index)) if user selected a new playlist or clicked a song
    fn show_left_sidebar(&mut self, ui: &mut Ui, is_playing: bool, is_paused: bool) -> (Option<String>, Option<usize>) {
        let mut selected_playlist_type = None;
        let mut clicked_song_index = None;
        
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
            ui.add_space(10.0);
            
            // Add right padding to prevent scrollbar overlap
            let margin = egui::Margin {
                left: 0.0,
                right: 10.0,
                top: 0.0,
                bottom: 0.0,
            };
            egui::Frame::none()
                .inner_margin(margin)
                .show(ui, |ui| {
                    // 1. Current Time / Next Procedure Card
                    self.show_time_procedure_card(ui);
                    ui.add_space(15.0);
                    
                    // 2. Weather Card
                    self.show_weather_card(ui);
                    ui.add_space(15.0);
                    
                    // 3. Tonight's Show Starts At Card
                    self.show_show_start_card(ui);
                    ui.add_space(15.0);
                    
                    // 4. Playlist Display
                    if let Some(song_idx) = self.show_playlist_display(ui) {
                        clicked_song_index = Some(song_idx);
                    }
                    ui.add_space(15.0);
                    
                    // 5. Start Show With Selector (only show when not playing, or when paused/stopped)
                    if !is_playing || is_paused {
                        if let Some(playlist_type) = self.show_start_show_selector(ui) {
                            // Return the playlist type to be handled in app.rs
                            selected_playlist_type = Some(playlist_type);
                        }
                        ui.add_space(10.0);
                    }
                });
        });
        
        (selected_playlist_type, clicked_song_index)
    }
    
    /// Main content area with playback controls and outputs
    /// Returns true if step button was clicked
    fn show_main_content(&mut self, 
        ui: &mut Ui,
        is_playing: &mut bool,
        is_paused: &mut bool,
        playback_position: Duration,
        playback_duration: Duration,
        current_song: &str,
        current_playlist: &str,
        audio_player: &Option<Arc<Mutex<AudioPlayer>>>,
        playback_panel_state: &mut PlaybackPanelState,
        current_song_path: &Option<PathBuf>,
        recent_commands: &[(u64, String)],
        fixture_manager: Option<&Arc<Mutex<FixtureManager>>>,
    ) -> bool {
        let mut step_clicked = false;
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Use the Phase 3 playback panel for the main content
            if playback_panel::show(
                ui,
                is_playing,
                is_paused,
                playback_position,
                playback_duration,
                current_song,
                current_playlist,
                audio_player,
                playback_panel_state,
                current_song_path,
                recent_commands,
            ) {
                step_clicked = true;
            }
            
            ui.add_space(8.0);
            
            // 4. DMX Output Section
            self.show_dmx_output(ui, fixture_manager, recent_commands);
            ui.add_space(20.0);
        });
        
        step_clicked
    }
    
    // Placeholder methods for each section - will implement in subsequent phases
    
    fn show_time_procedure_card(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    // Current Time
                    let now = Local::now();
                    let time_str = now.format("%I:%M:%S %p").to_string();
                    ui.label(RichText::new(time_str)
                        .size(24.0)
                        .strong()
                        .color(theme::AppColors::CYAN));
                    
                    ui.add_space(8.0);
                    
                    // Next Procedure
                    if let Some(ref procedure) = self.next_procedure {
                        // Show minutes countdown only if not in Idle Mode or Fountain Test
                        if procedure.name != "Perform Fountain Test" 
                            && procedure.name != "Idle Mode" 
                            && procedure.time_until.as_secs() > 0 {
                            let minutes_until = procedure.time_until.as_secs() / 60;
                            ui.label(RichText::new(format!("Next Procedure in {} Minutes", minutes_until))
                                .size(14.0)
                                .color(Color32::WHITE));
                            ui.add_space(4.0);
                        }
                        
                        ui.label(RichText::new(&procedure.name)
                            .size(15.0)
                            .strong()
                            .color(if procedure.name == "Perform Fountain Test" || procedure.name == "Idle Mode" {
                                theme::AppColors::CYAN
                            } else {
                                Color32::WHITE
                            }));
                    } else {
                        ui.label(RichText::new("No Procedure Scheduled")
                            .size(14.0)
                            .color(theme::AppColors::TEXT_SECONDARY));
                    }
                });
            });
    }
    
    fn show_weather_card(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Header with Weather text
                    ui.label(RichText::new("Weather")
                        .size(16.0)
                        .strong()
                        .color(theme::AppColors::CYAN));
                    
                    ui.add_space(8.0);
                    
                    // Current Conditions
                    ui.label(RichText::new(&self.weather.conditions)
                        .size(14.0)
                        .color(Color32::WHITE));
                    
                    ui.add_space(4.0);
                    
                    // Temperature and Wind
                    ui.horizontal(|ui| {
                        // Temperature with icon
                        if let Some(ref temp_icon) = self.temp_icon {
                            let icon_size = egui::Vec2::new(16.0, 16.0);
                            ui.add(egui::Image::new(temp_icon.as_ref()).fit_to_exact_size(icon_size).tint(Color32::WHITE));
                        }
                        ui.label(RichText::new(format!("Temp: {}°F", self.weather.temperature))
                            .size(13.0)
                            .color(Color32::WHITE));
                        ui.add_space(8.0);
                        
                        // Wind with icon
                        if let Some(ref wind_icon) = self.wind_icon {
                            let icon_size = egui::Vec2::new(16.0, 16.0);
                            ui.add(egui::Image::new(wind_icon.as_ref()).fit_to_exact_size(icon_size).tint(Color32::WHITE));
                        }
                        ui.label(RichText::new(format!("Wind: {} mph", self.weather.wind_speed))
                            .size(13.0)
                            .color(Color32::WHITE));
                    });
                });
            });
    }
    
    fn show_show_start_card(&mut self, ui: &mut Ui) {
        egui::Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(8.0)
            .inner_margin(16.0)
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(RichText::new("Tonight's Show Starts At")
                        .size(14.0)
                        .color(theme::AppColors::TEXT_SECONDARY));
                    ui.add_space(6.0);
                    ui.label(RichText::new(&self.show_start_time)
                        .size(28.0)
                        .strong()
                        .color(Color32::WHITE));
                });
            });
    }
    
    fn show_playlist_display(&mut self, ui: &mut Ui) -> Option<usize> {
        let mut clicked_song_index = None;
        
        egui::Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(8.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Playlist header with date and theme
                    ui.horizontal(|ui| {
                        // List icon
                        if let Some(ref list_icon) = self.list_icon {
                            let icon_size = egui::Vec2::new(16.0, 16.0);
                            ui.add(egui::Image::new(list_icon.as_ref()).fit_to_exact_size(icon_size).tint(Color32::WHITE));
                        }
                        
                        // Date
                        if let Some(ref date) = self.current_playlist_date {
                            ui.label(RichText::new(date)
                                .size(14.0)
                                .strong()
                                .color(Color32::WHITE));
                        } else {
                            let now = Local::now();
                            ui.label(RichText::new(now.format("%m-%d-%Y").to_string())
                                .size(14.0)
                                .strong()
                                .color(Color32::WHITE));
                        }
                        
                        // 2 spaces
                        ui.label(RichText::new("  ")
                            .size(14.0));
                        
                        // Theme icon
                        if let Some(ref theme_icon) = self.theme_icon {
                            let icon_size = egui::Vec2::new(16.0, 16.0);
                            ui.add(egui::Image::new(theme_icon.as_ref()).fit_to_exact_size(icon_size).tint(Color32::WHITE));
                        }
                        
                        // Theme
                        if let Some(ref theme) = self.current_playlist_theme {
                            ui.label(RichText::new(theme)
                                .size(14.0)
                                .strong()
                                .color(Color32::WHITE));
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    // Scrollable song list
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            if self.current_playlist.is_empty() {
                                ui.label(RichText::new("No playlist loaded")
                                    .size(13.0)
                                    .color(theme::AppColors::TEXT_SECONDARY));
                            } else {
                                for (index, song) in self.current_playlist.iter().enumerate() {
                                    // Make each song row clickable
                                    let is_current = index == self.current_song_index && self.playback.is_playing;
                                    let response = ui.horizontal(|ui| {
                                        // Play indicator for current song
                                        if is_current {
                                            ui.label(RichText::new("▶")
                                                .size(12.0)
                                                .color(Color32::from_rgb(0, 255, 0)));
                                        } else {
                                            ui.add_space(15.0);
                                        }
                                        
                                        // Song title with special markers
                                        let title = if song.is_opening {
                                            "Opening"
                                        } else if song.is_ending {
                                            "Ending"
                                        } else {
                                            &song.title
                                        };
                                        
                                        ui.label(RichText::new(title)
                                            .size(13.0)
                                            .color(if is_current {
                                                Color32::from_rgb(0, 255, 0)
                                            } else {
                                                theme::AppColors::TEXT_SECONDARY
                                            }));
                                        
                                        // Duration
                                        let duration_str = format!("{}:{:02}",
                                            song.duration.as_secs() / 60,
                                            song.duration.as_secs() % 60);
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            ui.label(RichText::new(duration_str)
                                                .size(13.0)
                                                .color(if is_current {
                                                    Color32::from_rgb(0, 255, 0)
                                                } else {
                                                    theme::AppColors::TEXT_SECONDARY
                                                }));
                                        });
                                    }).response;
                                    
                                    // Make the row clickable
                                    let row_response = ui.interact(response.rect, ui.id().with(index), egui::Sense::click());
                                    if row_response.clicked() {
                                        clicked_song_index = Some(index);
                                    }
                                    
                                    // Change cursor on hover
                                    if row_response.hovered() {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    }
                                    
                                    ui.add_space(4.0);
                                }
                            }
                        });
                    
                    ui.add_space(8.0);
                    ui.separator();
                    ui.add_space(4.0);
                    
                    // Total and Remaining time
                    ui.horizontal(|ui| {
                        let total_str = format!("Total: {}:{:02}",
                            self.total_runtime.as_secs() / 60,
                            self.total_runtime.as_secs() % 60);
                        ui.label(RichText::new(total_str)
                            .size(14.0)
                            .color(theme::AppColors::TEXT_SECONDARY));
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Calculate remaining time: total - elapsed time from all songs
                            let mut elapsed_total = Duration::from_secs(0);
                            for i in 0..self.current_song_index {
                                if i < self.current_playlist.len() {
                                    elapsed_total += self.current_playlist[i].duration;
                                }
                            }
                            // Add current song's elapsed time
                            elapsed_total += self.playback.current_position;
                            
                            let remaining = if self.total_runtime > elapsed_total {
                                self.total_runtime - elapsed_total
                            } else {
                                Duration::from_secs(0)
                            };
                            
                            let remaining_str = format!("Remaining: {}:{:02}",
                                remaining.as_secs() / 60,
                                remaining.as_secs() % 60);
                            ui.label(RichText::new(remaining_str)
                                .size(14.0)
                                .color(theme::AppColors::TEXT_SECONDARY));
                        });
                    });
                });
            });
        
        clicked_song_index
    }
    
    fn show_start_show_selector(&mut self, ui: &mut Ui) -> Option<String> {
        let mut playlist_to_load = None;
        
        egui::Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(8.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("Start Show With")
                        .size(13.0)
                        .color(theme::AppColors::TEXT_SECONDARY));
                    ui.add_space(6.0);
                    
                    // ComboBox for playlist selection
                    let available_width = ui.available_width();
                    let previous_selection = self.selected_playlist_index;
                    egui::ComboBox::from_id_source("start_show_playlist")
                        .selected_text(&self.available_playlists[self.selected_playlist_index])
                        .width(available_width)
                        .show_ui(ui, |ui| {
                            for (i, playlist_name) in self.available_playlists.iter().enumerate() {
                                ui.selectable_value(&mut self.selected_playlist_index, i, playlist_name);
                            }
                        });
                    
                    // If selection changed, trigger loading
                    if previous_selection != self.selected_playlist_index {
                        playlist_to_load = Some(self.available_playlists[self.selected_playlist_index].clone());
                    }
                });
            });
        
        playlist_to_load
    }
    
    fn show_dmx_output(&mut self, ui: &mut Ui, fixture_manager: Option<&Arc<Mutex<FixtureManager>>>, recent_commands: &[(u64, String)]) {
        // Load fixtures if not already loaded
        if self.available_fixtures.is_empty() {
            self.load_fixtures();
        }
        
        egui::Frame::none()
            .fill(theme::AppColors::SURFACE)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(8.0)
            .inner_margin(12.0)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.label(RichText::new("DMX Fixture Layout").size(14.0).color(theme::AppColors::TEXT_SECONDARY));
                    ui.add_space(8.0);
                    
                    // Render the 27x6 grid
                    self.render_fixture_grid(ui, fixture_manager);
                    
                    ui.add_space(12.0);
                    
                    // Show CTL command log
                    self.render_ctl_command_log(ui, recent_commands);
                });
            });
    }
    
    fn load_fixtures(&mut self) {
        // Load fixtures from DMX map JSON
        let dmx_map_path = "Config/dmx_mapping.json";
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
                        }
                    }
                    Err(e) => {
                        warn!("Failed to parse DMX mapping: {}", e);
                    }
                }
            }
            Err(e) => {
                warn!("Failed to load DMX mapping: {}", e);
            }
        }
    }
    
    fn render_fixture_grid(&self, ui: &mut Ui, fixture_manager: Option<&Arc<Mutex<FixtureManager>>>) {
        const GRID_COLS: usize = 27;
        const GRID_ROWS: usize = 6;
        const CELL_SIZE: f32 = 28.0; // Slightly larger with minimal padding
        const DEFAULT_BG: Color32 = Color32::from_rgb(40, 40, 40); // Dark gray
        
        // Render grid with transparent empty cells and minimal spacing
        ui.style_mut().spacing.item_spacing = Vec2::new(0.5, 0.5); // Minimal padding between cells
        
        for row in 0..GRID_ROWS {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = Vec2::new(0.5, 0.5); // Minimal spacing within row
                
                for col in 0..GRID_COLS {
                    let fixture_id = self.lights_layout.get_fixture_at(row, col);
                    
                    let (bg_color, text_color, text) = if let Some(fid) = fixture_id {
                        // Get current color from fixture manager (RGBW)
                        let (bg, text_col) = if let Some(fm) = fixture_manager {
                            if let Ok(fm_lock) = fm.lock() {
                                if let Some((r, g, b, _w)) = fm_lock.get_fixture_color(fid as u16) {
                                    let bg = Color32::from_rgb(r, g, b);
                                    let text = Self::calculate_text_color(r, g, b);
                                    (bg, text)
                                } else {
                                    (DEFAULT_BG, Color32::WHITE)
                                }
                            } else {
                                (DEFAULT_BG, Color32::WHITE)
                            }
                        } else {
                            (DEFAULT_BG, Color32::WHITE)
                        };
                        
                        (bg, text_col, format!("{}", fid))
                    } else {
                        // Empty cell - transparent
                        (Color32::TRANSPARENT, Color32::TRANSPARENT, "".to_string())
                    };
                    
                    let cell_rect = Rect::from_min_size(
                        ui.cursor().min,
                        Vec2::new(CELL_SIZE, CELL_SIZE),
                    );
                    
                    let response = ui.allocate_rect(cell_rect, Sense::hover());
                    
                    // Draw cell background (transparent for empty cells)
                    if bg_color != Color32::TRANSPARENT {
                        ui.painter().rect_filled(
                            cell_rect,
                            2.0,
                            bg_color,
                        );
                        
                        // Draw cell border only for non-empty cells
                        ui.painter().rect_stroke(
                            cell_rect,
                            2.0,
                            Stroke::new(0.5, theme::AppColors::SURFACE_LIGHT),
                        );
                        
                        // Draw text centered
                        if !text.is_empty() {
                            ui.painter().text(
                                cell_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                &text,
                                egui::FontId::proportional(9.0),
                                text_color,
                            );
                        }
                    }
                }
            });
        }
    }
    
    /// Calculate text color (white or black) based on background brightness
    fn calculate_text_color(r: u8, g: u8, b: u8) -> Color32 {
        // Calculate relative luminance using Rec. 709 coefficients
        let luminance = 0.2126 * (r as f32) + 0.7152 * (g as f32) + 0.0722 * (b as f32);
        
        // Use white text on dark backgrounds, black on light
        if luminance < 128.0 {
            Color32::WHITE
        } else {
            Color32::BLACK
        }
    }
    
    /// Render CTL command log showing recent lighting commands in MM:SS.T > format
    fn is_water_command_static(fcw_address: u16) -> bool {
        matches!(fcw_address,
            1..=15 |    // Water commands
            34 |        // Water control
            87..=90 |   // Water/effects
            250..=253   // Water effects
        )
    }
    
    fn render_ctl_command_log(&self, ui: &mut Ui, recent_commands: &[(u64, String)]) {
        // Create a fixed-height scrollable area for 2 lines
        const LINE_HEIGHT: f32 = 16.0;
        const VISIBLE_LINES: usize = 2;
        const SCROLL_HEIGHT: f32 = LINE_HEIGHT * VISIBLE_LINES as f32;
        
        egui::Frame::none()
            .fill(theme::AppColors::BACKGROUND)
            .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
            .rounding(4.0)
            .inner_margin(8.0)
            .show(ui, |ui| {
                egui::ScrollArea::vertical()
                    .max_height(SCROLL_HEIGHT)
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if recent_commands.is_empty() {
                            ui.label(RichText::new("No lighting commands").size(11.0).color(theme::AppColors::TEXT_SECONDARY));
                        } else {
                            // Filter only lighting commands (non-water commands)
                            use std::collections::HashMap;
                            let mut grouped: HashMap<u64, Vec<String>> = HashMap::new();
                            for (timestamp_ms, command) in recent_commands.iter() {
                                // Parse FCW address to filter out water commands
                                if let Some(dash_pos) = command.find('-') {
                                    if let Ok(fcw_address) = command[..dash_pos].parse::<u16>() {
                                        // Only show lighting commands (filter out water commands)
                                        if !Self::is_water_command_static(fcw_address) {
                                            grouped.entry(*timestamp_ms).or_insert_with(Vec::new).push(command.clone());
                                        }
                                    }
                                }
                            }
                            
                            // Get unique timestamps sorted in reverse order (most recent first)
                            let mut timestamps: Vec<u64> = grouped.keys().copied().collect();
                            timestamps.sort_unstable_by(|a, b| b.cmp(a));
                            
                            // Display each timestamp with all its commands on one line (space-separated)
                            for timestamp_ms in timestamps.iter().take(20) {
                                if let Some(commands) = grouped.get(timestamp_ms) {
                                    // Convert milliseconds to MM:SS.T format
                                    let total_seconds = (*timestamp_ms as f64) / 1000.0;
                                    let minutes = (total_seconds / 60.0).floor() as u64;
                                    let seconds = (total_seconds % 60.0).floor() as u64;
                                    let tenths = ((total_seconds % 1.0) * 10.0).floor() as u64;
                                    
                                    let time_str = format!("{:02}:{:02}.{}", minutes, seconds, tenths);
                                    let commands_str = commands.join(" ");
                                    let line = format!("{} > {}", time_str, commands_str);
                                    
                                    ui.label(RichText::new(line).size(11.0).color(theme::AppColors::TEXT_PRIMARY).monospace());
                                }
                            }
                        }
                    });
            });
    }
    
    fn show_announcement_popup_window(&mut self, ctx: &Context) {
        egui::Window::new("Select Announcement")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("Announcement Popup - TBD");
                
                if ui.button("Close").clicked() {
                    self.show_announcement_popup = false;
                }
            });
    }
}
// Weather.gov API structures
#[derive(Deserialize, Debug)]
struct PointsResponse {
    properties: PointsProperties,
}

#[derive(Deserialize, Debug)]
struct PointsProperties {
    #[serde(rename = "forecastHourly")]
    forecast_hourly: String,
}

#[derive(Deserialize, Debug)]
struct ForecastResponse {
    properties: ForecastProperties,
}

#[derive(Deserialize, Debug)]
struct ForecastProperties {
    periods: Vec<ForecastPeriod>,
}

#[derive(Deserialize, Debug)]
struct ForecastPeriod {
    temperature: i32,
    #[serde(rename = "shortForecast")]
    short_forecast: String,
    #[serde(rename = "startTime")]
    start_time: String,
    #[serde(rename = "probabilityOfPrecipitation")]
    probability_of_precipitation: Option<ProbabilityValue>,
    #[serde(rename = "windSpeed")]
    wind_speed: String,
}

#[derive(Deserialize, Debug)]
struct ProbabilityValue {
    value: Option<u32>,
}

/// Fetch weather data from weather.gov API for Grand Haven, MI
async fn fetch_weather_data() -> Result<WeatherInfo, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .user_agent("GHMF-Playback/2.0 (contact@example.com)")
        .build()?;
    
    // Grand Haven, MI coordinates
    let lat = 43.0631;
    let lon = -86.2284;
    
    // Step 1: Get the forecast URL for this location
    let points_url = format!("https://api.weather.gov/points/{},{}", lat, lon);
    let points_response: PointsResponse = client.get(&points_url)
        .send()
        .await?
        .json()
        .await?;
    
    // Step 2: Get hourly forecast
    let forecast_url = &points_response.properties.forecast_hourly;
    let forecast_response: ForecastResponse = client.get(forecast_url)
        .send()
        .await?
        .json()
        .await?;
    
    let periods = forecast_response.properties.periods;
    
    if periods.is_empty() {
        return Err("No forecast data available".into());
    }
    
    // Current conditions (first period)
    let current = &periods[0];
    
    // Extract wind speed number (e.g., "5 mph" -> 5)
    let wind_speed = current.wind_speed
        .split_whitespace()
        .next()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    
    // Get next hour forecast (take first 2 periods for next hour)
    let hourly_forecast: Vec<HourlyForecast> = periods.iter()
        .skip(1) // Skip current
        .take(2) // Next 2 hours
        .map(|period| {
            // Parse time from ISO 8601 format to just hour
            let time = chrono::DateTime::parse_from_rfc3339(&period.start_time)
                .ok()
                .map(|dt| dt.format("%I:%M %p").to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            
            HourlyForecast {
                time,
                temperature: period.temperature,
                short_forecast: period.short_forecast.clone(),
                precipitation_chance: period.probability_of_precipitation
                    .as_ref()
                    .and_then(|p| p.value),
            }
        })
        .collect();
    
    Ok(WeatherInfo {
        conditions: current.short_forecast.clone(),
        temperature: current.temperature,
        wind_speed,
        hourly_forecast,
    })
}