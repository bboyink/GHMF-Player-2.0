use super::theme;
use crate::audio::{AudioPlayer, WaveformData, ScrollingWaveformBuffer, BufferBuilder};
use egui::{Ui, RichText, Slider, Button, Color32, Stroke, Rect, Pos2, Vec2, Sense, TextureHandle, ColorImage};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::path::PathBuf;

pub struct PlaybackPanelState {
    pub left_volume: f32,  // 0.0 to 1.0 (display as 0-100)
    pub show_announcement_popup: bool,
    pub announcement_files: Vec<PathBuf>,
    pub announcements_folder: String, // Path to announcements folder from settings
    pub selected_announcement: Option<usize>,
    pub paused_for_announcement: bool,
    pub announcement_player: Option<Arc<Mutex<AudioPlayer>>>,
    pub show_playing: bool, // Track if main show is playing
    pub playing_announcement: bool, // Currently playing an announcement
    pub announcement_path: Option<PathBuf>, // Path to current announcement
    pub saved_position: Duration, // Position in song before announcement
    pub saved_song_path: Option<PathBuf>, // Song path before announcement
    pub waveform_data: Option<WaveformData>, // Real waveform data from audio file
    pub scrolling_buffer: Option<ScrollingWaveformBuffer>, // Optimized scrolling buffer
    pub megaphone_icon: Option<Arc<TextureHandle>>, // Megaphone icon for announcements
    pub audio_up_icon: Option<Arc<TextureHandle>>, // Audio up icon for volume
}

impl Default for PlaybackPanelState {
    fn default() -> Self {
        Self {
            left_volume: 0.35,  // Default 35%
            show_announcement_popup: false,
            announcement_files: Vec::new(),
            announcements_folder: "Music/Announcements".to_string(), // Default fallback
            selected_announcement: None,
            paused_for_announcement: false,
            announcement_player: None,
            show_playing: false,
            playing_announcement: false,
            announcement_path: None,
            saved_position: Duration::from_secs(0),
            saved_song_path: None,
            scrolling_buffer: None, // Will be created when waveform is loaded
            waveform_data: None, // Will be loaded when a song is loaded
            megaphone_icon: None, // Will be loaded on first use
            audio_up_icon: None, // Will be loaded on first use
        }
    }
}

impl PlaybackPanelState {
    /// Load waveform data from an audio file
    pub fn load_waveform<P: AsRef<std::path::Path>>(&mut self, path: P) {
        use tracing::warn;
        
        match WaveformData::from_file(path.as_ref(), 100) {
            Ok(waveform) => {
                // Create optimized scrolling buffer from waveform data
                let duration = waveform.duration_secs;
                let builder = BufferBuilder::from_waveform(waveform.samples.clone(), duration);
                let buffer = builder.build(7.0); // 7-second visible window
                
                self.waveform_data = Some(waveform);
                self.scrolling_buffer = Some(buffer);
            }
            Err(e) => {
                warn!("Failed to load waveform data: {}", e);
                // Use placeholder on error with minimum duration to prevent overflow
                let placeholder = WaveformData::placeholder(100);
                let duration = placeholder.duration_secs.max(1.0); // At least 1 second
                let builder = BufferBuilder::from_waveform(placeholder.samples.clone(), duration);
                let buffer = builder.build(7.0);
                
                self.waveform_data = Some(placeholder);
                self.scrolling_buffer = Some(buffer);
            }
        }
    }
    
    /// Clear waveform data (when no song is loaded)
    pub fn clear_waveform(&mut self) {
        self.waveform_data = None;
        self.scrolling_buffer = None;
    }
    
    /// Load megaphone icon (called lazily when needed)
    pub fn load_megaphone_icon(&mut self, ctx: &egui::Context) {
        if self.megaphone_icon.is_some() {
            return; // Already loaded
        }
        
        let icon_bytes = include_bytes!("../../assets/megaphone.png");
        
        if let Ok(image) = image::load_from_memory(icon_bytes) {
            let rgba = image.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let pixels = rgba.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "megaphone_icon",
                color_image,
                Default::default()
            );
            
            self.megaphone_icon = Some(Arc::new(texture));
        }
    }
    
    /// Load audio up icon (called lazily when needed)
    pub fn load_audio_up_icon(&mut self, ctx: &egui::Context) {
        if self.audio_up_icon.is_some() {
            return; // Already loaded
        }
        
        let icon_bytes = include_bytes!("../../assets/audio_up.png");
        
        if let Ok(image) = image::load_from_memory(icon_bytes) {
            let rgba = image.to_rgba8();
            let size = [rgba.width() as usize, rgba.height() as usize];
            let pixels = rgba.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "audio_up_icon",
                color_image,
                Default::default()
            );
            
            self.audio_up_icon = Some(Arc::new(texture));
        }
    }
}

pub fn show(
    ui: &mut Ui,
    is_playing: &mut bool,
    is_paused: &mut bool,
    playback_position: Duration,
    playback_duration: Duration,
    current_song: &str,
    current_playlist: &str,
    audio_player: &Option<Arc<Mutex<AudioPlayer>>>,
    state: &mut PlaybackPanelState,
    current_song_path: &Option<PathBuf>,
    recent_commands: &[(u64, String)],
) -> bool { // Returns true if step button clicked
    let mut step_clicked = false;
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        
        // Remove file extension from song name
        let song_name = current_song
            .trim_end_matches(".wav")
            .trim_end_matches(".mp3")
            .trim_end_matches(".WAV")
            .trim_end_matches(".MP3");
        
        // ============= 1. NOW PLAYING HEADER =============
        ui.heading(RichText::new(song_name).size(28.0).strong().color(theme::AppColors::TEXT_PRIMARY));
        
        // Show current time / total time (use waveform duration for accuracy)
        let current_time = format_duration(playback_position);
        let total_duration = if let Some(ref wf) = state.waveform_data {
            Duration::from_secs_f32(wf.duration_secs)
        } else {
            playback_duration
        };
        let total_time = format_duration(total_duration);
        ui.label(RichText::new(format!("{} / {}", current_time, total_time)).size(16.0).color(theme::AppColors::TEXT_SECONDARY));
        
        ui.add_space(15.0);
        
        // ============= 2. FULL WAVEFORM WITH MOVING PLAYHEAD =============
        show_full_waveform(ui, state, playback_position, playback_duration, audio_player, *is_playing);
        
        ui.add_space(25.0);
        
        // ============= 4. ACTION BUTTONS =============
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 460.0) / 2.0);
            
            // Play/Pause Button (Large, Prominent)
            let play_pause_text = if *is_playing && !*is_paused {
                "⏸ Pause"
            } else {
                "▶ Play"
            };
            
            let play_pause_color = if *is_playing && !*is_paused {
                Color32::from_rgb(200, 100, 0) // Orange when playing
            } else {
                Color32::from_rgb(0, 180, 0) // Green when stopped
            };
            
            let play_pause_button = Button::new(
                RichText::new(play_pause_text).size(22.0).strong().color(Color32::WHITE)
            ).fill(play_pause_color);
            
            if ui.add_sized([200.0, 70.0], play_pause_button).clicked() {
                if *is_playing && !*is_paused {
                    // Pause
                    *is_paused = true;
                    if let Some(player) = audio_player {
                        if let Ok(player) = player.lock() {
                            player.pause();
                        }
                    }
                } else {
                    // Play or Resume
                    *is_playing = true;
                    *is_paused = false;
                    if let Some(player) = audio_player {
                        if let Ok(player) = player.lock() {
                            if playback_position == Duration::from_secs(0) || playback_position >= playback_duration {
                                // Start from beginning
                                player.resume();
                            } else {
                                // Resume from current position
                                player.resume();
                            }
                        }
                    }
                }
                state.show_playing = *is_playing && !*is_paused;
            }
            
            ui.add_space(10.0);
            
            // Load megaphone icon if not already loaded
            state.load_megaphone_icon(ui.ctx());
            
            // Announcement Button with icon
            let announcement_button = Button::new(RichText::new("Announcements").size(18.0));
            
            let announcement_response = ui.add_sized([250.0, 70.0], announcement_button);
            
            // Draw megaphone icon on top of button (overlay)
            if let Some(ref icon) = state.megaphone_icon {
                let icon_size = egui::Vec2::new(20.0, 20.0);
                let button_rect = announcement_response.rect;
                let icon_pos = Pos2::new(
                    button_rect.left() + 20.0,
                    button_rect.center().y - icon_size.y / 2.0
                );
                let icon_rect = Rect::from_min_size(icon_pos, icon_size);
                ui.put(icon_rect, egui::Image::new(icon.as_ref()).fit_to_exact_size(icon_size));
            }
            
            if announcement_response.clicked() {
                state.show_announcement_popup = true;
                // Load announcement files
                if state.announcement_files.is_empty() {
                    state.announcement_files = load_announcement_files(&state.announcements_folder);
                }
            }
        });
        
        ui.add_space(30.0);
        
        // ============= 5. FOUNTAIN VOLUME CONTROL =============
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 750.0) / 2.0);
            
            // Fountain Volume Control
            ui.vertical(|ui| {
                ui.label(RichText::new("Fountain Volume").size(14.0).color(theme::AppColors::TEXT_SECONDARY));
                
                // Allocate fixed height row and manually position everything
                let row_height = 20.0;
                let row_response = ui.allocate_response(
                    egui::Vec2::new(ui.available_width(), row_height),
                    egui::Sense::hover()
                );
                
                let mut cursor_x = row_response.rect.left();
                let top_y = row_response.rect.top();
                
                let volume_percent = (state.left_volume * 100.0) as i32;
                
                // Slider at exact position
                let slider_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(cursor_x, top_y),
                    egui::Vec2::new(300.0, row_height)
                );
                let mut slider_ui = ui.child_ui(slider_rect, egui::Layout::left_to_right(egui::Align::Min), None);
                let slider = Slider::new(&mut state.left_volume, 0.0..=1.0).show_value(false);
                let slider_response = slider_ui.add(slider);
                if slider_response.changed() {
                    if let Some(player) = audio_player {
                        if let Ok(player) = player.lock() {
                            player.set_volume(state.left_volume);
                        }
                    }
                }
                // Position percentage box 20px after the actual slider response rect
                cursor_x = slider_response.rect.right() + 20.0;
                
                // Percentage box at exact position (after slider)
                let perc_rect = egui::Rect::from_min_size(
                    egui::Pos2::new(cursor_x, top_y),
                    egui::Vec2::new(50.0, row_height)
                );
                let mut perc_ui = ui.child_ui(perc_rect, egui::Layout::centered_and_justified(egui::Direction::LeftToRight), None);
                egui::Frame::none()
                    .fill(theme::AppColors::SURFACE)
                    .stroke(egui::Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                    .show(&mut perc_ui, |ui| {
                        ui.label(RichText::new(format!("{}%", volume_percent))
                            .size(14.0)
                            .color(theme::AppColors::TEXT_PRIMARY));
                    });
                cursor_x += 100.0; // 50px box + 50px spacing
                
                // Icon at exact position (after percentage)
                state.load_audio_up_icon(ui.ctx());
                if let Some(ref icon) = state.audio_up_icon {
                    let icon_rect = egui::Rect::from_min_size(
                        egui::Pos2::new(cursor_x, top_y),
                        egui::Vec2::new(20.0, row_height)
                    );
                    let mut icon_ui = ui.child_ui(icon_rect, egui::Layout::left_to_right(egui::Align::Min), None);
                    icon_ui.add(egui::Image::new(icon.as_ref()).fit_to_exact_size(egui::Vec2::new(20.0, 20.0)));
                    cursor_x += 25.0;
                }
                
                // Buttons 10px higher with 20px spacing
                let button_y = top_y - 10.0;
                for preset in [25, 35, 45, 55, 65, 75] {
                    let btn_rect = egui::Rect::from_min_size(
                        egui::Pos2::new(cursor_x, button_y),
                        egui::Vec2::new(45.0, row_height)
                    );
                    let mut btn_ui = ui.child_ui(btn_rect, egui::Layout::left_to_right(egui::Align::Min), None);
                    if btn_ui.add_sized([45.0, row_height], Button::new(RichText::new(format!("{}%", preset)).size(12.0))).clicked() {
                        state.left_volume = preset as f32 / 100.0;
                        if let Some(player) = audio_player {
                            if let Ok(player) = player.lock() {
                                player.set_volume(state.left_volume);
                            }
                        }
                    }
                    cursor_x += 65.0; // 45px button + 20px spacing
                }
            });
        });
        
        ui.add_space(10.0);
        
        // ============= 6. PLC OUTPUT =============
        show_plc_output(ui, recent_commands);
        
        ui.add_space(10.0);
    });
    
    // ============= ANNOUNCEMENT POPUP MODAL =============
    if state.show_announcement_popup {
        show_announcement_popup(ui.ctx(), state, audio_player, is_playing, is_paused, playback_position, current_song_path);
    }
    
    false // No step button anymore
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

/// Format time in MM:SS.T format (tenths of seconds)
fn format_time_with_tenths(time_ms: u64) -> String {
    let total_secs = time_ms / 1000;
    let tenths = (time_ms % 1000) / 100;
    let minutes = total_secs / 60;
    let seconds = total_secs % 60;
    format!("{:02}:{:02}.{}", minutes, seconds, tenths)
}

/// Check if FCW address is a water command (based on light_groups.json water directives)
fn is_water_command(fcw_address: u16) -> bool {
    match fcw_address {
        1..=13 | 33..=40 | 47..=48 | 87..=91 | 99 | 217..=223 | 249..=255 | 700..=749 => true,
        _ => false,
    }
}

/// Show PLC output display
fn show_plc_output(ui: &mut Ui, recent_commands: &[(u64, String)]) {
    
    // Filter and group water commands by timestamp
    let water_commands: Vec<(u64, Vec<&str>)> = {
        let mut grouped = std::collections::HashMap::new();
        
        for (time_ms, cmd_desc) in recent_commands.iter() {
            // Parse FCW address from command (format: XXX-YYY)
            if let Some(dash_pos) = cmd_desc.find('-') {
                if let Ok(fcw_address) = cmd_desc[..dash_pos].parse::<u16>() {
                    if is_water_command(fcw_address) {
                        grouped.entry(*time_ms)
                            .or_insert_with(Vec::new)
                            .push(cmd_desc.as_str());
                    }
                }
            }
        }
        
        let mut result: Vec<(u64, Vec<&str>)> = grouped.into_iter().collect();
        result.sort_by_key(|(time, _)| *time);
        result
    };
    
    // Lighting commands are now shown in DMX fixture feedback, not here
    
    // Use full width for output sections
    ui.vertical(|ui| {
        // PLC Output Card
            egui::Frame::none()
                .fill(theme::AppColors::SURFACE)
                .stroke(egui::Stroke::new(1.0, theme::AppColors::SURFACE_LIGHT))
                .rounding(8.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.vertical(|ui| {
                        // Header
                        ui.label(RichText::new("PLC Output").size(14.0).color(theme::AppColors::TEXT_SECONDARY));
                        ui.add_space(8.0);
                        
                        // Scrollable area for commands with min height to ensure full width on load
                        egui::ScrollArea::vertical()
                            .id_salt("plc_output_scroll")
                            .min_scrolled_height(100.0)
                            .max_height(100.0)
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                // Remove vertical spacing between lines
                                ui.spacing_mut().item_spacing.y = 0.0;
                                
                                if water_commands.is_empty() {
                                    ui.label(RichText::new("No water commands yet").size(12.0).color(theme::AppColors::TEXT_SECONDARY));
                                } else {
                                    for (time_ms, commands) in water_commands.iter().rev().take(20) {
                                        // Format: MM:SS.T > XXX-XXX XXX-XXX ...
                                        let time_str = format_time_with_tenths(*time_ms);
                                        
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing.x = 0.0;
                                            ui.spacing_mut().item_spacing.y = 0.0;
                                            
                                            // Time and separator
                                            ui.label(RichText::new(format!("{} > ", time_str))
                                                .size(12.0)
                                                .color(theme::AppColors::TEXT_PRIMARY)
                                                .family(egui::FontFamily::Monospace));
                                            
                                            // Commands with 099-000 in yellow
                                            for (i, cmd) in commands.iter().enumerate() {
                                                if i > 0 {
                                                    ui.label(RichText::new(" ")
                                                        .size(12.0)
                                                        .family(egui::FontFamily::Monospace));
                                                }
                                                
                                                let color = if *cmd == "099-000" {
                                                    egui::Color32::from_rgb(255, 215, 0) // Yellow/Gold
                                                } else {
                                                    theme::AppColors::TEXT_PRIMARY
                                                };
                                                
                                                ui.label(RichText::new(*cmd)
                                                    .size(12.0)
                                                    .color(color)
                                                    .family(egui::FontFamily::Monospace));
                                            }
                                        });
                                    }
                                }
                            });
                    });
                });
        
        // Lighting commands are now shown in DMX fixture feedback area
    });
}

fn load_announcement_files(announcements_folder: &str) -> Vec<PathBuf> {
    use tracing::warn;
    
    // Expand tilde and validate path
    let announcements_path = match shellexpand::tilde(announcements_folder).parse::<PathBuf>() {
        Ok(path) => path,
        Err(e) => {
            warn!("Invalid announcements folder path '{}': {}", announcements_folder, e);
            return Vec::new();
        }
    };
    
    // Check if path exists
    if !announcements_path.exists() {
        warn!("Announcements folder does not exist: {:?}", announcements_path);
        return Vec::new();
    }
    
    let mut files = Vec::new();
    
    // Try to read directory contents
    match std::fs::read_dir(&announcements_path) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if ext_str == "wav" || ext_str == "mp3" {
                            files.push(path);
                        }
                    }
                }
            }
        }
        Err(e) => {
            warn!("Failed to read announcements directory {:?}: {}", announcements_path, e);
            return Vec::new();
        }
    }
    
    // Sort files alphabetically
    files.sort();
    files
}

fn show_announcement_popup(
    ctx: &egui::Context,
    state: &mut PlaybackPanelState,
    audio_player: &Option<Arc<Mutex<AudioPlayer>>>,
    is_playing: &mut bool,
    is_paused: &mut bool,
    playback_position: Duration,
    current_song_path: &Option<PathBuf>,
) {
    egui::Window::new("Announcements")
        .fixed_size([500.0, 400.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Select an Announcement");
                ui.add_space(10.0);
                
                ui.separator();
                ui.add_space(10.0);
                
                // List of announcement files
                egui::ScrollArea::vertical()
                    .max_height(250.0)
                    .show(ui, |ui| {
                        if state.announcement_files.is_empty() {
                            ui.label(RichText::new("No announcement files found in Music/Announcements/")
                                .color(Color32::from_rgb(200, 200, 0)));
                        } else {
                            for (idx, file) in state.announcement_files.iter().enumerate() {
                                let file_name = file.file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("Unknown");
                                
                                let button_text = format!("🔊 {}", file_name);
                                
                                if ui.button(RichText::new(button_text).size(14.0)).clicked() {
                                    // Save current playback state
                                    state.saved_position = playback_position;
                                    state.saved_song_path = current_song_path.clone();
                                    
                                    // Pause current playback if playing
                                    if *is_playing && !*is_paused {
                                        if let Some(player) = audio_player {
                                            if let Ok(player) = player.lock() {
                                                player.pause();
                                            }
                                        }
                                        state.paused_for_announcement = true;
                                        *is_paused = true;
                                    }
                                    
                                    // Play announcement (no CTL file needed)
                                    if let Some(player) = audio_player {
                                        if let Ok(player) = player.lock() {
                                            if let Some(path) = file.to_str() {
                                                match player.play(path) {
                                                    Ok(_) => {
                                                        player.resume(); // Start playing immediately
                                                        state.playing_announcement = true;
                                                        state.announcement_path = Some(file.clone());
                                                    }
                                                    Err(_) => {
                                                        state.playing_announcement = false;
                                                        state.announcement_path = None;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    
                                    state.selected_announcement = Some(idx);
                                    state.show_announcement_popup = false; // Close popup after selection
                                }
                                
                                ui.add_space(5.0);
                            }
                        }
                    });
                
                ui.add_space(15.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Control buttons
                ui.horizontal(|ui| {
                    ui.add_space((ui.available_width() - 320.0) / 2.0);
                    
                    // Resume button (if paused for announcement)
                    if state.paused_for_announcement {
                        if ui.add_sized([150.0, 35.0], 
                            Button::new(RichText::new("▶ Resume Show").size(14.0))
                        ).clicked() {
                            // Resume main playback
                            if let Some(player) = audio_player {
                                if let Ok(player) = player.lock() {
                                    player.resume();
                                }
                            }
                            *is_paused = false;
                            state.paused_for_announcement = false;
                            state.show_announcement_popup = false;
                        }
                        ui.add_space(10.0);
                    }
                    
                    // Close button
                    if ui.add_sized([150.0, 35.0], 
                        Button::new(RichText::new("✕ Close").size(14.0))
                    ).clicked() {
                        state.show_announcement_popup = false;
                    }
                });
            });
        });
}

/// Render full waveform with moving playhead
fn show_full_waveform(
    ui: &mut Ui,
    state: &PlaybackPanelState,
    playback_position: Duration,
    playback_duration: Duration,
    audio_player: &Option<Arc<Mutex<AudioPlayer>>>,
    is_playing: bool,
) {
    let waveform_height = 60.0;
    let available_width = ui.available_width() * 0.95;
    
    // Allocate space for waveform with click sensing
    let (response, painter) = ui.allocate_painter(
        Vec2::new(available_width, waveform_height),
        Sense::click()
    );
    
    let waveform_rect = response.rect;
    
    // Draw background with dark gray border
    painter.rect_filled(waveform_rect, 3.0, Color32::from_rgb(15, 20, 30));
    painter.rect_stroke(waveform_rect, 3.0, Stroke::new(2.0, Color32::from_rgb(60, 60, 60)));
    
    // Handle click to seek
    if response.clicked() {
        if let Some(click_pos) = response.interact_pointer_pos() {
            // Calculate the relative position in the waveform
            let click_x = click_pos.x - waveform_rect.min.x;
            let progress = (click_x / available_width).clamp(0.0, 1.0);
            
            // Get duration from waveform data if available
            if let Some(ref wf) = state.waveform_data {
                let seek_time_secs = progress * wf.duration_secs;
                // Clamp to leave at least 0.1 seconds before end to prevent seeking past end
                let max_seek = (wf.duration_secs - 0.1).max(0.0);
                let clamped_seek_time = seek_time_secs.min(max_seek);
                let seek_duration = Duration::from_secs_f32(clamped_seek_time);
                
                // Seek the audio player
                if let Some(player) = audio_player {
                    if let Ok(player) = player.lock() {
                        let _ = player.seek(seek_duration);
                    }
                }
            }
        }
    }
    
    // If no waveform data, just show empty container and return
    let (full_waveform, waveform_duration_secs) = if let Some(ref wf) = state.waveform_data {
        (&wf.samples, wf.duration_secs)
    } else {
        // Show empty waveform container
        return;
    };
    
    let pos_secs = playback_position.as_secs_f32();
    let dur_secs = waveform_duration_secs.max(1.0);
    
    // Calculate playhead position based on waveform's actual duration
    let playhead_progress = (pos_secs / dur_secs).min(1.0);
    let playhead_x = waveform_rect.min.x + (playhead_progress * available_width);
    
    // Use thicker bars (2 pixels wide)
    let num_bars = (available_width / 2.0) as usize;
    let bar_width = 2.0;
    
    let total_samples = full_waveform.len();
    if total_samples == 0 {
        return;
    }
    
    // Draw ALL bars across the full width - map entire song to full width
    for bar_idx in 0..num_bars {
        // Map this bar position to sample position in the waveform
        // Each bar represents a portion of the song
        let bar_progress = bar_idx as f32 / num_bars as f32;
        let sample_start = (bar_progress * total_samples as f32) as usize;
        let next_bar_progress = (bar_idx + 1) as f32 / num_bars as f32;
        let sample_end = (next_bar_progress * total_samples as f32) as usize;
        let sample_end = sample_end.min(total_samples);
        
        // Skip if we're out of range
        if sample_start >= total_samples || sample_end <= sample_start {
            // Draw empty bar to fill the space
            painter.rect_filled(
                Rect::from_min_max(
                    Pos2::new(waveform_rect.min.x + (bar_idx as f32 * bar_width), waveform_rect.max.y - 2.0),
                    Pos2::new(waveform_rect.min.x + ((bar_idx + 1) as f32 * bar_width), waveform_rect.max.y)
                ),
                0.0,
                Color32::from_rgb(40, 50, 60)
            );
            continue;
        }
        
        // Calculate RMS for this bar's samples
        let bar_samples = &full_waveform[sample_start..sample_end];
        let rms = if !bar_samples.is_empty() {
            let sum: f32 = bar_samples.iter().map(|&s| s * s).sum();
            (sum / bar_samples.len() as f32).sqrt()
        } else {
            0.0
        };
        
        let x = waveform_rect.min.x + (bar_idx as f32 * bar_width);
        
        // Bar height from bottom - 3x scaling for tall bars, capped at 90% of container
        let max_bar_height = waveform_height * 0.9;
        let bar_height = (rms * waveform_height * 3.0).max(2.0).min(max_bar_height);
        
        // Color based on playback progress: played (cyan) vs unplayed (gray)
        let color = if bar_progress <= playhead_progress {
            Color32::from_rgb(50, 180, 255) // Bright cyan for played
        } else {
            Color32::from_rgb(80, 100, 120) // Gray for unplayed
        };
        
        // Draw bar from bottom up
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(x, waveform_rect.max.y - bar_height),
                Pos2::new(x + bar_width.max(1.0), waveform_rect.max.y)
            ),
            0.0,
            color
        );
    }
    
    // Draw playhead (red vertical line) - only if not playing announcement
    if is_playing && !state.playing_announcement {
        painter.line_segment(
            [
                Pos2::new(playhead_x, waveform_rect.min.y),
                Pos2::new(playhead_x, waveform_rect.max.y)
            ],
            Stroke::new(3.0, Color32::from_rgb(255, 80, 80))
        );
    }
}
