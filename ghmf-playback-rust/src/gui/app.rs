use super::{playback_panel, lighting_panel, status_panel, settings_dialog, command_panel, theme, sidebar, dmx_map_panel, light_group_panel, legacy_color_panel, playlist_panel, start_time_panel, procedures_panel, operator_panel};
use crate::audio::AudioPlayer;
use crate::dmx::{EnttecDmxPro, DmxUniverse};
use crate::plc::{PlcClient, PlcStatus};
use crate::config::{Settings, CsvConfig};
use crate::lighting::FixtureManager;
use crate::commands::CtlFile;
use egui::{CentralPanel, TopBottomPanel, SidePanel, Context, Color32, Stroke, Vec2};
use egui_notify::Toasts;
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
    plc_last_status_check: Instant,
    
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
    
    // Light Group Mapper
    light_group_panel: light_group_panel::LightGroupPanel,
    
    // Legacy Color Mapper
    legacy_color_panel: legacy_color_panel::LegacyColorPanel,
    
    // Start Time Panel
    start_time_panel: start_time_panel::StartTimePanel,
    
    // Procedures Panel
    procedures_panel: procedures_panel::ProceduresPanel,
    
    // Playlist Panel
    playlist_panel: playlist_panel::PlaylistPanel,
    
    // Operator Panel
    operator_panel: operator_panel::OperatorPanel,
    
    // Playback Panel State
    playback_panel_state: playback_panel::PlaybackPanelState,
    
    // Toast notifications
    toasts: Toasts,
    
    // File dialog result channel
    folder_dialog_rx: Option<std::sync::mpsc::Receiver<(String, String)>>, // (folder_type, path)
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
            plc_last_status_check: Instant::now(),
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
            light_group_panel: light_group_panel::LightGroupPanel::new(),
            legacy_color_panel: legacy_color_panel::LegacyColorPanel::default(),
            start_time_panel: start_time_panel::StartTimePanel::new(),
            procedures_panel: procedures_panel::ProceduresPanel::new(),
            playlist_panel: playlist_panel::PlaylistPanel::new(),
            operator_panel: operator_panel::OperatorPanel::new(),
            playback_panel_state: playback_panel::PlaybackPanelState::default(),
            toasts: Toasts::default(),
            folder_dialog_rx: None,
        }
    }
}

impl PlaybackApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure custom theme
        theme::configure_theme(&cc.egui_ctx);
        
        let mut app = Self::default();
        
        // Set announcements folder from settings
        app.playback_panel_state.announcements_folder = app.settings.announcements_folder.clone();
        
        // Initialize systems
        app.initialize_audio();
        app.initialize_dmx();
        app.initialize_plc();
        app.initialize_csv_config();
        
        // Load first song from Pre-Show playlist if available
        app.load_first_pre_show_song();
        
        app
    }
    
    fn initialize_audio(&mut self) {
        match AudioPlayer::new() {
            Ok(player) => {
                self.audio_player = Some(Arc::new(Mutex::new(player)));
                info!("Audio system initialized");
            }
            Err(e) => {
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
                let mut fixture_manager = FixtureManager::new(
                    Arc::try_unwrap(config_arc.clone()).unwrap_or_else(|arc| (*arc).clone())
                );
                
                // Set RGBW mode from settings
                fixture_manager.set_rgbw_mode(self.settings.use_rgbw);
                
                self.fixture_manager = Some(Arc::new(Mutex::new(fixture_manager)));
                self.csv_config = Some(config_arc);
                
                info!("Loaded CSV configuration from Config/");
            }
            Err(e) => {
                warn!("Failed to load CSV config: {}", e);
            }
        }
    }
    
    fn initialize_dmx(&mut self) {
        if !self.settings.dmx_enabled {
            return;
        }
        
        match EnttecDmxPro::new() {
            Ok(controller) => {
                self.dmx_controller = Some(Arc::new(Mutex::new(controller)));
                self.dmx_connected = true;
                info!("DMX controller initialized");
            }
            Err(e) => {
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
            self.plc_status = PlcStatus::Disconnected; // Will update to Connected once connection succeeds
        } else {
            self.plc_client = Some(Arc::new(plc));
            self.plc_status = PlcStatus::Disabled;
        }
    }
    
    fn load_first_pre_show_song(&mut self) {
        // Try to get the first song from Pre-Show playlist
        if let Some(song_path) = self.operator_panel.get_next_song_from_current_playlist() {
            self.load_song(song_path.clone());
            self.current_playlist = "Pre-Show".to_string();
            
            // Load waveform for the song
            if let Some(ref song_path) = self.current_song_path {
                self.playback_panel_state.load_waveform(song_path);
            }
            
            // Don't auto-play on startup - user needs to press play
        }
    }
    
    fn set_status(&mut self, message: &str, status_type: StatusType) {
        self.status_message = message.to_string();
        self.status_type = status_type;
        self.status_time = Instant::now();
        
        // Also show toast notification
        match status_type {
            StatusType::Success => {
                self.toasts.success(message);
            }
            StatusType::Error => {
                self.toasts.error(message);
            }
            StatusType::Warning => {
                self.toasts.warning(message);
            }
            StatusType::Info => {
                self.toasts.info(message);
            }
        }
    }
    
    fn update_playback_state(&mut self) {
        // Check if announcement finished and needs to resume playlist
        if self.playback_panel_state.playing_announcement {
            if let Some(player) = &self.audio_player {
                if let Ok(player) = player.lock() {
                    // Check if announcement finished (player is idle/stopped)
                    if player.is_finished() {
                        // Announcement finished - reload the playlist song and resume
                        self.playback_panel_state.playing_announcement = false;
                        self.playback_panel_state.announcement_path = None;
                        
                        // Reload the saved song and seek to saved position
                        if let Some(ref song_path) = self.playback_panel_state.saved_song_path {
                            let path_str = song_path.to_string_lossy();
                            let song_path_clone = song_path.clone(); // Clone for later use
                            if let Ok(_) = player.play(&path_str) {
                                // Reload waveform for the show song (not for announcements)
                                drop(player); // Release lock before loading waveform
                                self.playback_panel_state.load_waveform(&song_path_clone);
                                
                                // Re-acquire lock for seek/resume
                                if let Some(player) = &self.audio_player {
                                    if let Ok(player) = player.lock() {
                                        // Seek to saved position
                                        let saved_pos = self.playback_panel_state.saved_position;
                                        if saved_pos > Duration::from_secs(0) {
                                            let _ = player.seek(saved_pos);
                                        }
                                        
                                        // Resume if we were paused for announcement
                                        if self.playback_panel_state.paused_for_announcement {
                                            player.resume();
                                            self.playback_panel_state.paused_for_announcement = false;
                                            self.is_paused = false;
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Clear saved state
                        self.playback_panel_state.saved_position = Duration::from_secs(0);
                        self.playback_panel_state.saved_song_path = None;
                    }
                }
            }
        }
        
        let (should_execute_commands, song_finished) = if let Some(player) = &self.audio_player {
            if let Ok(player) = player.lock() {
                let was_playing = self.is_playing;
                let was_paused = self.is_paused;
                self.is_playing = player.is_playing();
                self.is_paused = player.is_paused();
                self.playback_position = player.get_position();
                self.master_volume = player.get_volume();
                self.playback_panel_state.left_volume = player.get_volume();
                
                // Check if song finished by comparing position to waveform duration
                let song_duration = if let Some(ref wf) = self.playback_panel_state.waveform_data {
                    Duration::from_secs_f32(wf.duration_secs)
                } else {
                    Duration::from_secs(999999) // No waveform = treat as very long
                };
                
                // Song finished if: was playing AND NOT currently paused AND reached the end
                // Use a small buffer (0.5s) to detect near the end reliably
                let near_end = song_duration.saturating_sub(Duration::from_millis(500));
                let finished = was_playing && !self.is_paused && 
                    self.playback_position >= near_end && 
                    self.playback_position < song_duration + Duration::from_secs(2); // Prevent infinite detection
                
                // Check if we should execute commands
                (self.is_playing && !self.is_paused, finished)
            } else {
                (false, false)
            }
        } else {
            (false, false)
        };
        
        // Handle song finished outside the player lock to avoid borrow issues
        if song_finished {
            if let Some(song_path) = self.operator_panel.get_next_song_from_current_playlist() {
                self.load_song(song_path.clone());
                // Only load waveform if the song loaded successfully
                if self.current_song_path.is_some() && self.audio_player.is_some() {
                    if let Some(ref song_path) = self.current_song_path {
                        self.playback_panel_state.load_waveform(song_path);
                    }
                    // Auto-play the next song (unless it's Opening)
                    let is_opening = self.current_song.to_lowercase().contains("opening");
                    if !is_opening {
                        if let Some(player) = &self.audio_player {
                            if let Ok(player) = player.lock() {
                                player.resume();
                            }
                        }
                    }
                }
            } else {
                // No more songs - check if this was Pre-Show
                if self.operator_panel.current_playlist_type.as_deref() == Some("Pre-Show") {
                    // Pre-Show is complete - load today's playlist and auto-play
                    self.operator_panel.load_todays_playlist();
                    
                    // Get first song from today's playlist
                    if let Some(song_path) = self.operator_panel.get_next_song_from_current_playlist() {
                        self.load_song(song_path.clone());
                        self.current_playlist = "Playlist".to_string();
                        
                        // Load waveform for the new song
                        if let Some(ref song_path) = self.current_song_path {
                            self.playback_panel_state.load_waveform(song_path);
                        }
                        
                        // Auto-play when transitioning from Pre-Show to main playlist
                        if let Some(player) = &self.audio_player {
                            if let Ok(player) = player.lock() {
                                player.resume();
                            }
                        }
                    }
                } else {
                    // Regular playlist is complete - show "Show Completed"
                    if let Some(player) = &self.audio_player {
                        if let Ok(player) = player.lock() {
                            player.stop();
                        }
                    }
                    self.current_song = "Show Completed".to_string();
                    self.is_playing = false;
                    self.is_paused = false;
                    self.playback_position = Duration::from_secs(0);
                    self.playback_duration = Duration::from_secs(0);
                    self.playback_panel_state.clear_waveform();
                }
            }
        }
        
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
    
    fn update_plc_status(&mut self) {
        // Only check status every 500ms to avoid overhead
        if self.plc_last_status_check.elapsed() < Duration::from_millis(500) {
            return;
        }
        
        self.plc_last_status_check = Instant::now();
        
        if let Some(plc) = &self.plc_client {
            let plc_clone = Arc::clone(plc);
            
            // Spawn a quick thread to get status without blocking UI
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();
                
                let status = rt.block_on(async {
                    plc_clone.status().await
                });
                
                let _ = tx.send(status);
            });
            
            // Give it a tiny bit of time to respond (1ms)
            std::thread::sleep(Duration::from_millis(1));
            
            // Try to receive the status
            if let Ok(status) = rx.try_recv() {
                self.plc_status = status;
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
        
        // Load audio file into player (but pause immediately - don't auto-play)
        let mut audio_loaded = false;
        let mut audio_error = None;
        
        if let Some(player) = &self.audio_player {
            if let Ok(player) = player.lock() {
                let path_str = song_path.to_string_lossy();
                match player.play(&path_str) {
                    Ok(_) => {
                        // Audio player now starts paused by default
                        info!("Audio loaded (paused, ready to play): {}", path_str);
                        audio_loaded = true;
                    }
                    Err(e) => {
                        warn!("Failed to load audio: {}", e);
                        audio_error = Some(format!("Audio error: {}", e));
                    }
                }
            }
        }
        
        if audio_loaded {
            self.is_playing = false;
            self.is_paused = true; // Start paused
            self.playback_position = Duration::from_secs(0); // Reset position
            self.last_command_time = 0;
            self.recent_commands.clear();
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
        // Operator mode - New comprehensive operator interface
        
        // Update operator panel with procedures data and show start time
        let procedures = self.procedures_panel.get_procedures();
        let show_start_time = self.start_time_panel.get_today_start_time();
        self.operator_panel.update_procedures(&procedures, &show_start_time);
        
        // Sync playback state for playlist highlighting
        self.operator_panel.playback.is_playing = self.is_playing && !self.is_paused;
        self.operator_panel.playback.current_position = self.playback_position;
        
        let (selected_playlist_type, clicked_song_index) = self.operator_panel.show(
            ctx, 
            ui, 
            &mut self.is_playing, 
            &mut self.is_paused,
            self.playback_position,
            self.playback_duration,
            &self.current_song,
            &self.current_playlist,
            &self.audio_player,
            &mut self.playback_panel_state,
            &self.current_song_path,
        );
        
        // If user clicked a song in the playlist, jump to that song
        if let Some(song_index) = clicked_song_index {
            if let Some(song_path) = self.operator_panel.jump_to_song(song_index) {
                self.load_song(song_path.clone());
                self.current_playlist = "Production".to_string(); // Or track the actual playlist type
                
                // Load waveform for the new song
                if let Some(ref song_path) = self.current_song_path {
                    self.playback_panel_state.load_waveform(song_path);
                }
                
                // Auto-play the jumped-to song (unless it's Opening)
                let is_opening = self.current_song.to_lowercase().contains("opening");
                if !is_opening {
                    if let Some(player) = &self.audio_player {
                        if let Ok(player) = player.lock() {
                            player.resume();
                        }
                    }
                }
            }
        }
        
        // If user selected a new playlist type, load the playlist and first song
        if let Some(playlist_type) = selected_playlist_type {
            // Load the appropriate playlist based on type
            // Note: load_pre_show_playlist and load_todays_playlist do file I/O
            // but are necessary to populate the playlist display
            match playlist_type.as_str() {
                "Pre-Show" => {
                    self.operator_panel.load_pre_show_playlist();
                    // Load first song from Pre-Show
                    if let Some(song_path) = self.operator_panel.get_next_song_from_current_playlist() {
                        self.load_song(song_path);
                        self.current_playlist = playlist_type;
                        
                        // Load waveform for the new song
                        if let Some(ref song_path) = self.current_song_path {
                            self.playback_panel_state.load_waveform(song_path);
                        }
                    }
                }
                "Playlist" => {
                    self.operator_panel.load_todays_playlist();
                    // Load first song from today's playlist
                    if let Some(song_path) = self.operator_panel.get_next_song_from_current_playlist() {
                        self.load_song(song_path);
                        self.current_playlist = playlist_type;
                        
                        // Load waveform for the new song
                        if let Some(ref song_path) = self.current_song_path {
                            self.playback_panel_state.load_waveform(song_path);
                        }
                    }
                }
                "Testing" => {
                    self.operator_panel.load_testing_playlist();
                    // Load first song from Testing playlist
                    if let Some(song_path) = self.operator_panel.get_next_song_from_current_playlist() {
                        self.load_song(song_path);
                        self.current_playlist = playlist_type;
                        
                        // Load waveform for the new song
                        if let Some(ref song_path) = self.current_song_path {
                            self.playback_panel_state.load_waveform(song_path);
                        }
                    }
                }
                _ => {} // Other types handled separately
            }
        }
    }
    
    fn show_operator_view_old(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        // OLD Operator mode - Main playback interface
        
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
                self.playback_position,
                self.playback_duration,
                &self.current_song,
                &self.current_playlist,
                &self.audio_player,
                &mut self.playback_panel_state,
                &self.current_song_path,
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
                            // Get channels that should be ignored during reset
                            let ignore_channels = self.dmx_map_panel.get_ignore_reset_channels();
                            if ignore_channels.is_empty() {
                                dmx.clear();
                            } else {
                                dmx.clear_except(&ignore_channels);
                            }
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
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(20.0);
            
            self.playlist_panel.show(
                ui,
                &self.settings.production_folder,
                &self.settings.testing_folder,
                &self.settings.events_folder,
                &self.settings.pre_show_folder,
                &self.settings.playlist_folder,
                &self.settings.open_close_folder
            );
            
            ui.add_space(20.0);
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
                    
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("Fixture Mode:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.radio_value(&mut self.settings.use_rgbw, true, 
                            egui::RichText::new("RGBW (4 channels)")
                                .size(13.0)
                                .color(Color32::WHITE)
                        );
                        ui.add_space(10.0);
                        ui.radio_value(&mut self.settings.use_rgbw, false, 
                            egui::RichText::new("RGB (3 channels)")
                                .size(13.0)
                                .color(Color32::WHITE)
                        );
                    });
                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(if self.settings.use_rgbw {
                            "Converts RGB to RGBW for better color mixing"
                        } else {
                            "RGB mode only, White channel set to 0"
                        })
                            .size(12.0)
                            .color(theme::AppColors::TEXT_DISABLED)
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
                    let mut port_str = self.settings.plc_port.to_string();
                    if ui.text_edit_singleline(&mut port_str).changed() {
                        if let Ok(port_num) = port_str.parse::<u16>() {
                            self.settings.plc_port = port_num;
                        }
                    }
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Send water commands to PLC over TCP/IP")
                            .size(13.0)
                            .color(theme::AppColors::TEXT_SECONDARY)
                    );
                    
                    ui.add_space(15.0);
                    
                    // Test button
                    let test_button = egui::Button::new(
                        egui::RichText::new("🧪 Test PLC (099-000)")
                            .size(14.0)
                            .color(Color32::WHITE)
                    )
                    .fill(theme::AppColors::CYAN)
                    .min_size(Vec2::new(180.0, 36.0))
                    .rounding(8.0);
                    
                    if ui.add(test_button).clicked() {
                        if let Some(plc) = &self.plc_client {
                            plc.queue_command_sync("099-000".to_string());
                            self.set_status("Test command 099-000 sent to PLC", StatusType::Info);
                        } else {
                            self.set_status("PLC not initialized", StatusType::Warning);
                        }
                    }
                });
            
            ui.add_space(20.0);
            
            // Folder Paths Settings Card
            egui::Frame::none()
                .fill(theme::AppColors::SURFACE)
                .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                .rounding(12.0)
                .inner_margin(24.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Folder Paths")
                            .size(20.0)
                            .strong()
                            .color(theme::AppColors::CYAN)
                    );
                    ui.add_space(10.0);
                    ui.add(egui::Separator::default().spacing(0.0));
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Production Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.production_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.production_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("production".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Testing Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.testing_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.testing_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("testing".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Events Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.events_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.events_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("events".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Drone Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.drone_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.drone_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("drone".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Open-Close Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.open_close_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.open_close_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("open_close".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Playlist Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.playlist_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.playlist_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("playlist".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Pre-Show Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.pre_show_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.pre_show_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("pre_show".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Announcements Folder:")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.add_sized([300.0, 24.0], 
                            egui::TextEdit::singleline(&mut self.settings.announcements_folder));
                        let folder_btn = egui::Button::new(egui::RichText::new("📁").size(16.0))
                            .min_size(egui::Vec2::new(24.0, 24.0))
                            .frame(false)
                            .fill(Color32::TRANSPARENT);
                        if ui.add(folder_btn).clicked() {
                            let current_dir = self.settings.announcements_folder.clone();
                            let (tx, rx) = std::sync::mpsc::channel();
                            self.folder_dialog_rx = Some(rx);
                            let ctx = ui.ctx().clone();
                            std::thread::spawn(move || {
                                if let Some(path) = rfd::FileDialog::new()
                                    .set_directory(&current_dir)
                                    .pick_folder() {
                                    let _ = tx.send(("announcements".to_string(), path.display().to_string()));
                                    ctx.request_repaint();
                                }
                            });
                        }
                    });
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Paths for show files and configurations")
                            .size(13.0)
                            .color(theme::AppColors::TEXT_SECONDARY)
                    );
                });
            
            ui.add_space(20.0);
            
            // Drone Settings Card
            egui::Frame::none()
                .fill(theme::AppColors::SURFACE)
                .stroke(Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                .rounding(12.0)
                .inner_margin(24.0)
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Drone Settings")
                            .size(20.0)
                            .strong()
                            .color(theme::AppColors::CYAN)
                    );
                    ui.add_space(10.0);
                    ui.add(egui::Separator::default().spacing(0.0));
                    ui.add_space(15.0);
                    
                    ui.label(
                        egui::RichText::new("Battery Warning (Songs Before Message):")
                            .size(14.0)
                            .color(Color32::WHITE)
                    );
                    ui.add_space(5.0);
                    let mut warning_songs_str = self.settings.drone_battery_warning_songs.to_string();
                    if ui.add_sized([120.0, 24.0], 
                        egui::TextEdit::singleline(&mut warning_songs_str)).changed() {
                        if let Ok(songs) = warning_songs_str.parse::<u32>() {
                            self.settings.drone_battery_warning_songs = songs;
                        }
                    }
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Number of songs before battery swap message appears")
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
                        // Update fixture manager RGBW mode
                        if let Some(fm) = &self.fixture_manager {
                            if let Ok(mut fm) = fm.lock() {
                                fm.set_rgbw_mode(self.settings.use_rgbw);
                            }
                        }
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
        
        // Check for folder dialog results
        if let Some(rx) = &self.folder_dialog_rx {
            if let Ok((folder_type, path)) = rx.try_recv() {
                match folder_type.as_str() {
                    "production" => self.settings.production_folder = path,
                    "testing" => self.settings.testing_folder = path,
                    "events" => self.settings.events_folder = path,
                    "drone" => self.settings.drone_folder = path,
                    "open_close" => self.settings.open_close_folder = path,
                    "playlist" => self.settings.playlist_folder = path,
                    "pre_show" => self.settings.pre_show_folder = path,
                    "announcements" => {
                        self.settings.announcements_folder = path.clone();
                        self.playback_panel_state.announcements_folder = path;
                    }
                    _ => {}
                }
                self.folder_dialog_rx = None;
            }
        }
        
        // Update state
        self.update_playback_state();
        self.update_dmx_state();
        self.update_plc_status();
        
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
                &self.plc_status,
                self.settings.use_rgbw
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
                    
                    // If switching to Operator view, re-initialize as if app just loaded
                    if new_view == sidebar::AppView::Operator {
                        // Reset operator panel to fresh state
                        self.operator_panel = crate::gui::operator_panel::OperatorPanel::new();
                        
                        // Load the first Pre-Show song without blocking
                        self.load_first_pre_show_song();
                    }
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
                sidebar::AppView::Settings => {
                    // Settings parent - show nothing or a welcome message
                    ui.vertical_centered(|ui| {
                        ui.add_space(100.0);
                        ui.heading("Settings");
                        ui.add_space(20.0);
                        ui.label("Select a settings category from the menu");
                    });
                }
                sidebar::AppView::SettingsDmxMap => {
                    self.dmx_map_panel.show(ctx, ui);
                }
                sidebar::AppView::SettingsLightGroups => {
                    self.light_group_panel.show(ctx, ui);
                }
                sidebar::AppView::SettingsLegacyColor => {
                    self.legacy_color_panel.show(ctx, ui);
                }
                sidebar::AppView::SettingsStartTime => {
                    self.start_time_panel.show(ctx, ui);
                }
                sidebar::AppView::SettingsProcedures => {
                    self.procedures_panel.show(ctx, ui);
                }
                sidebar::AppView::SettingsApp => {
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
        
        // Show toast notifications on top of everything
        self.toasts.show(ctx);
        
        // Request repaint for smooth animations
        ctx.request_repaint_after(Duration::from_millis(16)); // ~60fps
    }
}
