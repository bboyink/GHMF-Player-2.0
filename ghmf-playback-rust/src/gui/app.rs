use super::{playback_panel, lighting_panel, status_panel, settings_dialog, command_panel, theme, sidebar, dmx_map_panel};
use crate::audio::AudioPlayer;
use crate::dmx::{EnttecDmxPro, DmxUniverse};
use crate::plc::{PlcClient, PlcStatus};
use crate::config::{Settings, CsvConfig};
use crate::lighting::FixtureManager;
use crate::commands::CtlFile;
use egui::{CentralPanel, TopBottomPanel, SidePanel, Context, Color32, Stroke, Vec2};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::path::PathBuf;
use tracing::{info, warn};

pub use status_panel::StatusType;

pub struct PlaybackApp {
    // Core systems
    audio_player: Option<Arc<Mutex<AudioPlayer>>>,
    dmx_controller: Option<Arc<Mutex<EnttecDmxPro>>>,
    plc_client: Option<Arc<PlcClient>>,
    fixture_manager: Option<Arc<Mutex<FixtureManager>>>,
    csv_config: Option<Arc<CsvConfig>>,
    settings: Settings,
    
    // Playback data
    current_song_path: Option<PathBuf>,
    current_ctl_file: Option<CtlFile>,
    last_command_time: u64,
    recent_commands: Vec<(u64, String)>,  // (time_ms, command_description)
    
    // UI State
    sidebar: sidebar::Sidebar,
    show_about: bool,
    current_song: String,
    current_playlist: String,
    
    // Playback state
    is_playing: bool,
    is_paused: bool,
    playback_position: Duration,
    playback_duration: Duration,
    
    // DMX state
    dmx_connected: bool,
    dmx_last_update: Instant,
    
    // PLC state
    plc_status: PlcStatus,
    
    // Volume
    master_volume: f32,
    
    // Status messages
    status_message: String,
    status_type: StatusType,
    status_time: Instant,
    
    // Light control
    selected_fixture: Option<usize>,
    fixture_red: u8,
    fixture_green: u8,
    fixture_blue: u8,
    
    // DMX Mapper
    dmx_map_panel: dmx_map_panel::DmxMapPanel,
}


impl Default for PlaybackApp {
    fn default() -> Self {
        Self {
            audio_player: None,
            dmx_controller: None,
            plc_client: None,
            settings: Settings::load(),
            sidebar: sidebar::Sidebar::default(),
            show_about: false,
            current_song: "No song loaded".to_string(),
            current_playlist: "No playlist loaded".to_string(),
            is_playing: false,
            is_paused: false,
            playback_position: Duration::from_secs(0),
            playback_duration: Duration::from_secs(0),
            dmx_connected: false,
            dmx_last_update: Instant::now(),
            plc_status: PlcStatus::Disabled,
            master_volume: 0.8,
            status_message: "Ready".to_string(),
            status_type: StatusType::Info,
            status_time: Instant::now(),
            selected_fixture: None,
            fixture_red: 0,
            fixture_green: 0,
            fixture_blue: 0,
            csv_config: None,
            fixture_manager: None,
            current_song_path: None,
            current_ctl_file: None,
            last_command_time: 0,
            recent_commands: Vec::new(),
            dmx_map_panel: dmx_map_panel::DmxMapPanel::new(),
        }
    }
}

impl PlaybackApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure custom theme
        theme::configure_theme(&cc.egui_ctx);
        
        let mut app = Self::default();
        
        // Initialize systems
        app.initialize_audio();
        app.initialize_dmx();
        app.initialize_plc();
        app.initialize_csv_config();
        
        app
    }
    
    fn initialize_audio(&mut self) {
        match AudioPlayer::new() {
            Ok(player) => {
                self.audio_player = Some(Arc::new(Mutex::new(player)));
                self.set_status("Audio system initialized", StatusType::Success);
                info!("Audio system initialized");
            }
            Err(e) => {
                self.set_status(&format!("Audio init failed: {}", e), StatusType::Error);
                warn!("Failed to initialize audio: {}", e);
            }
        }
    }
    
    fn initialize_csv_config(&mut self) {
        // Try to load CSV config from Config/ directory
        let config_path = std::path::PathBuf::from("Config");
        
        match CsvConfig::load_from_dir(&config_path) {
            Ok(config) => {
                let config_arc = Arc::new(config);
                
                // Create fixture manager
                let fixture_manager = FixtureManager::new(
                    Arc::try_unwrap(config_arc.clone()).unwrap_or_else(|arc| (*arc).clone())
                );
                
                self.fixture_manager = Some(Arc::new(Mutex::new(fixture_manager)));
                self.csv_config = Some(config_arc);
                
                self.set_status("Configuration loaded from Config/", StatusType::Success);
                info!("Loaded CSV configuration from Config/");
            }
            Err(e) => {
                self.set_status(&format!("Config load failed: {}", e), StatusType::Warning);
                warn!("Failed to load CSV config: {}", e);
            }
        }
    }
    
    fn initialize_dmx(&mut self) {
        if !self.settings.dmx_enabled {
            self.set_status("DMX disabled in settings", StatusType::Info);
            return;
        }
        
        match EnttecDmxPro::new() {
            Ok(controller) => {
                self.dmx_controller = Some(Arc::new(Mutex::new(controller)));
                self.dmx_connected = true;
                self.set_status("DMX controller connected", StatusType::Success);
                info!("DMX controller initialized");
            }
            Err(e) => {
                self.set_status(&format!("DMX init failed: {}", e), StatusType::Warning);
                warn!("DMX initialization failed: {}", e);
            }
        }
    }
    
    fn initialize_plc(&mut self) {
        let plc = PlcClient::new(
            self.settings.plc_enabled,
            self.settings.plc_ip_address.clone(),
            self.settings.plc_port,
        );
        
        if self.settings.plc_enabled {
            let plc_arc = Arc::new(plc);
            let plc_clone = Arc::clone(&plc_arc);
            
            // Start persistent PLC connection and sender thread
            std::thread::spawn(move || {
                // Create runtime that lives for the entire thread
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                
                // Run forever in this runtime
                rt.block_on(async move {
                    // Connect once
                    if let Err(e) = plc_clone.connect(5000).await {
                        tracing::warn!("PLC connection failed: {}", e);
                        return;
                    }
                    
                    tracing::info!("PLC connected, starting sender loop");
                    
                    // Keep sending queued commands every 100ms
                    loop {
                        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        
                        if let Err(e) = plc_clone.send_queue().await {
                            tracing::debug!("PLC send error: {}", e);
                            // Try to reconnect
                            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                            if let Err(e) = plc_clone.connect(5000).await {
                                tracing::warn!("PLC reconnection failed: {}", e);
                            } else {
                                tracing::info!("PLC reconnected successfully");
                            }
                        }
                    }
                });
            });
            
            self.plc_client = Some(plc_arc);
            self.set_status("PLC client initialized", StatusType::Info);
        } else {
            self.plc_client = Some(Arc::new(plc));
            self.set_status("PLC disabled in settings", StatusType::Info);
        }
    }
    
    fn set_status(&mut self, message: &str, status_type: StatusType) {
        self.status_message = message.to_string();
        self.status_type = status_type;
        self.status_time = Instant::now();
    }
    
    fn update_playback_state(&mut self) {
        let should_execute_commands = if let Some(player) = &self.audio_player {
            if let Ok(player) = player.lock() {
                self.is_playing = player.is_playing();
                self.is_paused = player.is_paused();
                self.playback_position = player.get_position();
                self.master_volume = player.get_volume();
                
                // Check if we should execute commands
                self.is_playing && !self.is_paused
            } else {
                false
            }
        } else {
            false
        };
        
        // Execute commands if playing (after releasing player lock)
        if should_execute_commands {
            let current_time_ms = self.playback_position.as_millis() as u64;
            
            // Only execute commands for new 100ms intervals
            let current_interval = current_time_ms / 100;
            let last_interval = self.last_command_time / 100;
            
            if current_interval != last_interval {
                self.execute_commands_at_time(current_time_ms);
                self.last_command_time = current_time_ms;
            }
        }
    }
    
    fn update_dmx_state(&mut self) {
        if self.dmx_last_update.elapsed() > Duration::from_millis(50) {
            if let Some(dmx) = &self.dmx_controller {
                if let Ok(mut dmx) = dmx.lock() {
                    let _ = dmx.send_dmx();
                    self.dmx_last_update = Instant::now();
                }
            }
        }
    }
    
    fn open_song_dialog(&mut self) {
        // Spawn file dialog in background to avoid blocking UI
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("Audio Files", &["wav", "mp3", "flac"])
            .set_directory(std::env::current_dir().unwrap_or_default().join("Songs"))
            .pick_file()
        {
            self.load_song(path);
        }
    }
    
    fn load_song(&mut self, song_path: PathBuf) {
        // Check for corresponding .ctl file
        let ctl_path = song_path.with_extension("ctl");
        
        if !ctl_path.exists() {
            self.set_status(
                &format!("Warning: No .ctl file found for {}", song_path.display()),
                StatusType::Warning
            );
        }
        
        // Load CTL file
        match CtlFile::load(&ctl_path) {
            Ok(ctl_file) => {
                info!("Loaded CTL file: {} ({} command lines)", 
                    ctl_path.display(), ctl_file.lines.len());
                self.current_ctl_file = Some(ctl_file);
            }
            Err(e) => {
                warn!("Failed to load CTL file: {}", e);
                self.set_status(&format!("CTL load error: {}", e), StatusType::Warning);
            }
        }
        
        // Update UI state
        self.current_song = song_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();
        self.current_song_path = Some(song_path.clone());
        
        // Load audio file into player
        let mut audio_loaded = false;
        let mut audio_error = None;
        
        if let Some(player) = &self.audio_player {
            if let Ok(player) = player.lock() {
                let path_str = song_path.to_string_lossy();
                match player.play(&path_str) {
                    Ok(_) => {
                        info!("Audio loaded and playing: {}", path_str);
                        audio_loaded = true;
                    }
                    Err(e) => {
                        warn!("Failed to play audio: {}", e);
                        audio_error = Some(format!("Audio error: {}", e));
                    }
                }
            }
        }
        
        if audio_loaded {
            self.is_playing = true;
            self.is_paused = false;
            self.last_command_time = 0;
            self.recent_commands.clear();
            self.set_status(
                &format!("Loaded: {}", self.current_song),
                StatusType::Success
            );
        } else if let Some(err_msg) = audio_error {
            self.set_status(&err_msg, StatusType::Warning);
        } else {
            self.set_status("Audio player not initialized", StatusType::Warning);
        }
        
        info!("Song loaded: {}", song_path.display());
    }
    
    fn execute_commands_at_time(&mut self, time_ms: u64) {
        // Skip if no CTL file or fixture manager
        let ctl_file = match &self.current_ctl_file {
            Some(f) => f,
            None => return,
        };
        
        let fixture_manager = match &self.fixture_manager {
            Some(fm) => fm,
            None => return,
        };
        
        // Get commands for this time window
        let commands = ctl_file.get_commands_at_time(time_ms);
        
        if commands.is_empty() {
            return;
        }
        
        // Execute each command
        for cmd in &commands {
            let mut fm = fixture_manager.lock().unwrap();
            
            // Format as raw CTL format: "XXX-YYY" 
            let cmd_desc = if cmd.is_hex_color {
                format!("{:03}-{}", cmd.fcw_address, cmd.hex_color.as_ref().unwrap_or(&"???".to_string()))
            } else {
                format!("{:03}-{:03}", cmd.fcw_address, cmd.data)
            };
            
            // Check if this is a water command (for PLC routing)
            let is_water = Self::is_water_command(cmd.fcw_address);
            
            // Queue water commands to PLC (synchronous, non-blocking)
            if is_water {
                if let Some(plc) = &self.plc_client {
                    plc.queue_command_sync(cmd_desc.clone());
                }
            }
            
            // Add to recent commands (keep last 100 for better history)
            self.recent_commands.push((time_ms, cmd_desc));
            if self.recent_commands.len() > 100 {
                self.recent_commands.remove(0);
            }
            
            let result = if cmd.is_hex_color {
                if let Some(hex) = &cmd.hex_color {
                    fm.execute_hex_command(cmd.fcw_address, hex)
                } else {
                    continue;
                }
            } else {
                fm.execute_fcw_command(cmd.fcw_address, cmd.data)
            };
            
            if let Err(e) = result {
                warn!("Command execution error: {}", e);
            }
        }
        
        // Send to DMX if controller is available
        if let Some(dmx) = &self.dmx_controller {
            if let Some(fm) = &self.fixture_manager {
                let mut universe = DmxUniverse::new();
                let fm = fm.lock().unwrap();
                
                if let Err(e) = fm.apply_to_dmx(&mut universe) {
                    warn!("Failed to apply to DMX: {}", e);
                    return;
                }
                drop(fm);
                
                let mut dmx = dmx.lock().unwrap();
                // Copy universe data to DMX controller
                for (i, val) in universe.as_slice().iter().enumerate() {
                    if let Err(e) = dmx.set_channel(i + 1, *val) {
                        warn!("Failed to set DMX channel {}: {}", i + 1, e);
                        break;
                    }
                }
                
                if let Err(e) = dmx.send_dmx() {
                    warn!("Failed to send DMX: {}", e);
                }
            }
        }
        
        // PLC sending is handled by background thread - commands are already queued
    }
    
    /// Check if FCW address is a water command
    fn is_water_command(fcw_address: u16) -> bool {
        (fcw_address >= 1 && fcw_address <= 13) ||
        (fcw_address >= 217 && fcw_address <= 255) ||
        (fcw_address >= 700 && fcw_address <= 896)
    }
    
    // View rendering methods
    fn show_operator_view(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        // Operator mode - Main playback interface
        
        // Command output panel (left side)
        SidePanel::left("command_panel_operator")
            .default_width(350.0)
            .resizable(true)
            .show_inside(ui, |ui| {
                command_panel::show(
                    ui,
                    &self.recent_commands,
                    self.playback_position.as_millis() as u64,
                    self.current_ctl_file.is_some(),
                );
            });
        
        // Main playback controls (center)
        CentralPanel::default().show_inside(ui, |ui| {
            playback_panel::show(
                ui,
                &mut self.is_playing,
                &mut self.is_paused,
                &mut self.master_volume,
                self.playback_position,
                self.playback_duration,
                &self.current_song,
                &self.current_playlist,
                &self.audio_player,
            );
            
            // File operations section
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                if ui.button(
                    egui::RichText::new("📁 Open Song...")
                        .size(16.0)
                        .color(theme::AppColors::CYAN_LIGHT)
                ).clicked() {
                    self.open_song_dialog();
                }
                
                if ui.button(
                    egui::RichText::new("📋 Open Playlist...")
                        .size(16.0)
                        .color(theme::AppColors::TEXT_SECONDARY)
                ).clicked() {
                    self.set_status("Playlist feature coming soon", StatusType::Info);
                }
            });
        });
    }
    
    fn show_testing_view(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        // Testing mode - Manual light control and system testing
        
        // Lighting control panel (right side)
        SidePanel::right("lighting_panel_testing")
            .default_width(350.0)
            .resizable(true)
            .show_inside(ui, |ui| {
                lighting_panel::show(
                    ui,
                    &mut self.selected_fixture,
                    &mut self.fixture_red,
                    &mut self.fixture_green,
                    &mut self.fixture_blue,
                    &self.dmx_controller,
                );
            });
        
        // Test controls (center)
        CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("Testing Mode")
                        .size(24.0)
                        .color(theme::AppColors::CYAN)
                );
                ui.add_space(10.0);
                ui.label(
                    egui::RichText::new("Manual control and system testing")
                        .size(14.0)
                        .color(theme::AppColors::TEXT_SECONDARY)
                );
            });
            
            ui.add_space(30.0);
            ui.separator();
            ui.add_space(20.0);
            
            // System status cards
            egui::Grid::new("system_status_grid")
                .num_columns(2)
                .spacing([20.0, 20.0])
                .show(ui, |ui| {
                    // DMX Status Card
                    egui::Frame::none()
                        .fill(theme::AppColors::SURFACE)
                        .rounding(12.0)
                        .inner_margin(20.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("DMX Controller")
                                        .size(16.0)
                                        .color(theme::AppColors::TEXT_SECONDARY)
                                );
                                ui.add_space(10.0);
                                let status_text = if self.dmx_connected { "Connected" } else { "Disconnected" };
                                let status_color = if self.dmx_connected { 
                                    theme::AppColors::SUCCESS 
                                } else { 
                                    theme::AppColors::ERROR 
                                };
                                ui.label(
                                    egui::RichText::new(status_text)
                                        .size(20.0)
                                        .strong()
                                        .color(status_color)
                                );
                            });
                        });
                    
                    // PLC Status Card
                    egui::Frame::none()
                        .fill(theme::AppColors::SURFACE)
                        .rounding(12.0)
                        .inner_margin(20.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new("PLC Connection")
                                        .size(16.0)
                                        .color(theme::AppColors::TEXT_SECONDARY)
                                );
                                ui.add_space(10.0);
                                let (status_text, status_color) = match &self.plc_status {
                                    PlcStatus::Connected => ("Connected", theme::AppColors::SUCCESS),
                                    PlcStatus::Disconnected => ("Disconnected", theme::AppColors::ERROR),
                                    PlcStatus::Disabled => ("Disabled", theme::AppColors::TEXT_DISABLED),
                                    PlcStatus::Reconnecting => ("Reconnecting...", theme::AppColors::WARNING),
                                };
                                ui.label(
                                    egui::RichText::new(status_text)
                                        .size(20.0)
                                        .strong()
                                        .color(status_color)
                                );
                            });
                        });
                    
                    ui.end_row();
                });
            
            ui.add_space(30.0);
            
            // Test Actions
            ui.heading("Quick Actions");
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                if ui.button(
                    egui::RichText::new("🔄 Reconnect DMX")
                        .size(14.0)
                ).clicked() {
                    self.initialize_dmx();
                }
                
                if ui.button(
                    egui::RichText::new("🔄 Reconnect PLC")
                        .size(14.0)
                ).clicked() {
                    self.initialize_plc();
                }
                
                if ui.button(
                    egui::RichText::new("💡 All Lights Off")
                        .size(14.0)
                ).clicked() {
                    let mut cleared = false;
                    if let Some(dmx) = &self.dmx_controller {
                        if let Ok(mut dmx) = dmx.lock() {
                            dmx.clear();
                            let _ = dmx.send_dmx();
                            cleared = true;
                        }
                    }
                    if cleared {
                        self.set_status("All lights cleared", StatusType::Info);
                    }
                }
            });
        });
    }
    
    fn show_playlist_view(&mut self, _ctx: &Context, ui: &mut egui::Ui) {
        // Playlist manager - coming soon
        ui.vertical_centered(|ui| {
            ui.add_space(50.0);
            ui.heading(
                egui::RichText::new("Playlist Manager")
                    .size(28.0)
                    .color(theme::AppColors::CYAN)
            );
            ui.add_space(20.0);
            ui.label(
                egui::RichText::new("Create and manage playlists for automated shows")
                    .size(16.0)
                    .color(theme::AppColors::TEXT_SECONDARY)
            );
            ui.add_space(40.0);
            
            egui::Frame::none()
                .fill(theme::AppColors::SURFACE)
                .rounding(12.0)
                .inner_margin(40.0)
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("🚧")
                                .size(64.0)
                        );
                        ui.add_space(20.0);
                        ui.label(
                            egui::RichText::new("Coming Soon")
                                .size(20.0)
                                .color(theme::AppColors::WARNING)
                        );
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new("This feature is under development")
                                .size(14.0)
                                .color(theme::AppColors::TEXT_SECONDARY)
                        );
                    });
                });
        });
    }
    
    fn show_settings_view(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        // Settings view - inline settings panel
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(20.0);
            
            ui.vertical(|ui| {
                ui.heading(
                    egui::RichText::new("Application Settings")
                        .size(24.0)
                        .color(theme::AppColors::CYAN)
                );
                ui.add_space(5.0);
                ui.label(
                    egui::RichText::new("Configure system connections and preferences")
                        .size(14.0)
                        .color(theme::AppColors::TEXT_SECONDARY)
                );
            });
            
            ui.add_space(30.0);
            
            // DMX Settings Card
            egui::Frame::none()
                .fill(theme::AppColors::SURFACE)
                .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                .rounding(12.0)
                .inner_margin(24.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("DMX Controller")
                            .size(20.0)
                            .strong()
                            .color(theme::AppColors::CYAN)
                    );
                    ui.add_space(10.0);
                    ui.add(egui::Separator::default().spacing(0.0));
                    ui.add_space(15.0);
                    
                    ui.checkbox(&mut self.settings.dmx_enabled, 
                        egui::RichText::new("Enable DMX output")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Control DMX lighting via Enttec USB Pro")
                            .size(13.0)
                            .color(theme::AppColors::TEXT_SECONDARY)
                    );
                });
            
            ui.add_space(20.0);
            
            // PLC Settings Card
            egui::Frame::none()
                .fill(theme::AppColors::SURFACE)
                .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                .rounding(12.0)
                .inner_margin(24.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("PLC Connection")
                            .size(20.0)
                            .strong()
                            .color(theme::AppColors::CYAN)
                    );
                    ui.add_space(10.0);
                    ui.add(egui::Separator::default().spacing(0.0));
                    ui.add_space(15.0);
                    
                    ui.checkbox(&mut self.settings.plc_enabled, 
                        egui::RichText::new("Enable PLC communication")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(18.0);
                    
                    ui.label(
                        egui::RichText::new("PLC IP Address:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.text_edit_singleline(&mut self.settings.plc_ip_address);
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("PLC Port:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.add(egui::Slider::new(&mut self.settings.plc_port, 1..=65535));
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Send water commands to PLC over TCP/IP")
                            .size(13.0)
                            .color(theme::AppColors::TEXT_SECONDARY)
                    );
                });
            
            ui.add_space(30.0);
            
            // Save button with modern styling
            ui.horizontal(|ui| {
                let save_button = egui::Button::new(
                    egui::RichText::new("💾 Save Settings")
                        .size(16.0)
                        .color(Color32::WHITE)
                )
                .fill(theme::AppColors::CYAN)
                .min_size(Vec2::new(160.0, 44.0))
                .rounding(10.0);
                
                if ui.add(save_button).clicked() {
                    if let Err(e) = self.settings.save() {
                        self.set_status(&format!("Failed to save settings: {}", e), StatusType::Warning);
                    } else {
                        self.set_status("Settings saved successfully", StatusType::Success);
                        // Reinitialize systems with new settings
                        self.initialize_dmx();
                        self.initialize_plc();
                    }
                }
                
                ui.add_space(10.0);
                
                let about_button = egui::Button::new(
                    egui::RichText::new("ℹ️ About")
                        .size(16.0)
                        .color(Color32::WHITE)
                )
                .fill(theme::AppColors::SURFACE)
                .min_size(Vec2::new(120.0, 44.0))
                .rounding(10.0);
                
                if ui.add(about_button).clicked() {
                    self.show_about = true;
                }
            });
            
            ui.add_space(20.0);
        });
    }
}

impl eframe::App for PlaybackApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Apply theme
        theme::configure_theme(ctx);
        
        // Update state
        self.update_playback_state();
        self.update_dmx_state();
        
        // Bottom status bar with dark background
        TopBottomPanel::bottom("status_bar")
            .frame(egui::Frame::none()
                .fill(theme::AppColors::SURFACE)
                .inner_margin(8.0))
            .show(ctx, |ui| {
            status_panel::show(
                ui,
                &self.status_message,
                self.status_type,
                self.status_time,
                self.dmx_connected,
                &self.plc_status
            );
        });
        
        // Left sidebar navigation with dark background
        SidePanel::left("sidebar")
            .resizable(false)
            .frame(egui::Frame::none()
                .fill(theme::AppColors::BACKGROUND_LIGHT)
                .inner_margin(0.0))
            .show(ctx, |ui| {
                if let Some(new_view) = self.sidebar.show(ctx, ui) {
                    // View changed
                    info!("Switched to view: {:?}", new_view);
                }
            });
        
        // Main content area with dark background
        CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(theme::AppColors::PANEL)
                .inner_margin(20.0))
            .show(ctx, |ui| {
            match self.sidebar.selected_view {
                sidebar::AppView::Operator => {
                    self.show_operator_view(ctx, ui);
                }
                sidebar::AppView::Testing => {
                    self.show_testing_view(ctx, ui);
                }
                sidebar::AppView::Playlist => {
                    self.show_playlist_view(ctx, ui);
                }
                sidebar::AppView::DmxMap => {
                    self.dmx_map_panel.show(ctx, ui);
                }
                sidebar::AppView::Settings => {
                    self.show_settings_view(ctx, ui);
                }
            }
        });
        
        // About dialog
        if self.show_about {
            let should_close = std::cell::Cell::new(false);
            
            egui::Window::new("About GHMF Playback")
                .open(&mut self.show_about)
                .resizable(false)
                .collapsible(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("GHMF Playback 2.0");
                        ui.add_space(10.0);
                        ui.label("Cross-platform fountain playback system");
                        ui.label("with synchronized audio and DMX lighting");
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                        ui.label("Built with Rust 🦀");
                        ui.label("© City of Grand Haven");
                        ui.add_space(10.0);
                        if ui.button("Close").clicked() {
                            should_close.set(true);
                        }
                    });
                });
            
            if should_close.get() {
                self.show_about = false;
            }
        }
        
        // Request repaint for smooth animations
        ctx.request_repaint_after(Duration::from_millis(16)); // ~60fps
    }
}
