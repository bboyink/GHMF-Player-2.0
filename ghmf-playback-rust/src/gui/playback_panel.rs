use super::theme;
use crate::audio::{AudioPlayer, WaveformData, ScrollingWaveformBuffer, BufferBuilder};
use egui::{Ui, RichText, Slider, Button, Color32, Stroke, Rect, Pos2, Vec2, Sense};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::path::PathBuf;

pub struct PlaybackPanelState {
    pub left_volume: f32,  // 0.0 to 1.0 (display as 0-100)
    pub right_volume: f32, // Fixed at 0.45 (45%)
    pub show_announcement_popup: bool,
    pub announcement_files: Vec<PathBuf>,
    pub selected_announcement: Option<usize>,
    pub paused_for_announcement: bool,
    pub announcement_player: Option<Arc<Mutex<AudioPlayer>>>,
    pub show_playing: bool, // Track if main show is playing
    pub waveform_data: Option<WaveformData>, // Real waveform data from audio file
    pub scrolling_buffer: Option<ScrollingWaveformBuffer>, // Optimized scrolling buffer
}

impl Default for PlaybackPanelState {
    fn default() -> Self {
        Self {
            left_volume: 0.35,  // Default 35%
            right_volume: 0.45, // Fixed 45%
            show_announcement_popup: false,
            announcement_files: Vec::new(),
            selected_announcement: None,
            paused_for_announcement: false,
            announcement_player: None,
            show_playing: false,
            scrolling_buffer: None, // Will be created when waveform is loaded
            waveform_data: None, // Will be loaded when a song is loaded
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
                // Use placeholder on error
                let placeholder = WaveformData::placeholder(100);
                let duration = placeholder.duration_secs;
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
) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        
        // ============= 1. NOW PLAYING HEADER =============
        ui.heading(RichText::new(current_song).size(28.0).strong().color(theme::AppColors::TEXT_PRIMARY));
        ui.label(RichText::new(format!("Playlist: {}", current_playlist)).size(16.0).color(theme::AppColors::TEXT_SECONDARY));
        
        ui.add_space(15.0);
        
        // ============= 2. WAVEFORM VISUALIZATION (7-SECOND SCROLLING VIEW) =============
        show_scrolling_waveform(ui, state, playback_position, playback_duration);
        
        ui.add_space(25.0);
        
        // ============= 4. DUAL VOLUME CONTROLS =============
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 700.0) / 2.0);
            
            // Left Volume Control (Interactive)
            ui.vertical(|ui| {
                ui.label(RichText::new("Left Volume").size(14.0).color(theme::AppColors::TEXT_SECONDARY));
                ui.horizontal(|ui| {
                    let volume_percent = (state.left_volume * 100.0) as i32;
                    let slider = Slider::new(&mut state.left_volume, 0.0..=1.0)
                        .text(format!("{}%", volume_percent))
                        .custom_formatter(|val, _| format!("{:.0}%", val * 100.0));
                    
                    if ui.add_sized([250.0, 20.0], slider).changed() {
                        // Apply volume to audio player
                        if let Some(player) = audio_player {
                            if let Ok(player) = player.lock() {
                                player.set_volume(state.left_volume);
                            }
                        }
                    }
                });
            });
            
            ui.add_space(50.0);
            
            // Right Volume Display (Fixed, Non-Interactive)
            ui.vertical(|ui| {
                ui.label(RichText::new("Right Volume").size(14.0).color(theme::AppColors::TEXT_SECONDARY));
                ui.horizontal(|ui| {
                    let volume_percent = (state.right_volume * 100.0) as i32;
                    
                    // Display as a progress bar (non-interactive)
                    ui.add_sized(
                        [250.0, 20.0],
                        egui::ProgressBar::new(state.right_volume)
                            .text(format!("{}% (Fixed)", volume_percent))
                    );
                });
            });
        });
        
        ui.add_space(30.0);
        
        // ============= 5. ACTION BUTTONS =============
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 700.0) / 2.0);
            
            // Restart Button
            let restart_button = Button::new(
                RichText::new("⏮ Restart").size(16.0)
            );
            
            if ui.add_sized([120.0, 60.0], restart_button).clicked() {
                // Restart song from beginning by reloading
                if let Some(path) = current_song_path {
                    if let Some(player) = audio_player {
                        if let Ok(player) = player.lock() {
                            let path_str = path.to_string_lossy();
                            let _ = player.play(&path_str);
                            // Pause if not currently playing
                            if !*is_playing || *is_paused {
                                player.pause();
                            }
                        }
                    }
                }
            }
            
            ui.add_space(10.0);
            
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
            
            // Announcement Button
            let announcement_button = Button::new(
                RichText::new("📢 Announcements").size(18.0)
            );
            
            if ui.add_sized([250.0, 60.0], announcement_button).clicked() {
                state.show_announcement_popup = true;
                // Load announcement files
                if state.announcement_files.is_empty() {
                    state.announcement_files = load_announcement_files();
                }
            }
        });
        
        ui.add_space(20.0);
    });
    
    // ============= ANNOUNCEMENT POPUP MODAL =============
    if state.show_announcement_popup {
        show_announcement_popup(ui.ctx(), state, audio_player, is_playing, is_paused);
    }
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

fn load_announcement_files() -> Vec<PathBuf> {
    let announcements_path = PathBuf::from("Music/Announcements");
    
    if !announcements_path.exists() {
        return Vec::new();
    }
    
    let mut files = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&announcements_path) {
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
                                    // Pause current playback
                                    if *is_playing && !*is_paused {
                                        if let Some(player) = audio_player {
                                            if let Ok(player) = player.lock() {
                                                player.pause();
                                            }
                                        }
                                        state.paused_for_announcement = true;
                                        *is_paused = true;
                                    }
                                    
                                    // Play announcement
                                    // TODO: Create separate announcement player or use main player
                                    if let Some(player) = audio_player {
                                        if let Ok(player) = player.lock() {
                                            if let Some(path) = file.to_str() {
                                                let _ = player.play(path);
                                            }
                                        }
                                    }
                                    
                                    state.selected_announcement = Some(idx);
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

/// Render scrolling waveform with fixed playhead at 1-second mark
/// Uses Scrollscope-inspired optimized buffer for smooth scrolling
fn show_scrolling_waveform(
    ui: &mut Ui,
    state: &PlaybackPanelState,
    playback_position: Duration,
    playback_duration: Duration,
) {
    let pos_secs = playback_position.as_secs_f32();
    let dur_secs = playback_duration.as_secs_f32().max(1.0);
    
    // Get waveform samples from optimized buffer
    let waveform_samples = if let Some(ref buffer) = state.scrolling_buffer {
        buffer.get_samples()
    } else {
        // Fallback placeholder
        vec![0.3; 700] // 100 samples per second * 7 seconds
    };
    
    let waveform_height = 120.0;
    let timeline_height = 35.0;
    let total_height = waveform_height + timeline_height;
    let available_width = ui.available_width() * 0.9;
    
    // Allocate space for waveform + timeline
    let (response, painter) = ui.allocate_painter(
        Vec2::new(available_width, total_height),
        Sense::hover()
    );
    
    let waveform_rect = Rect::from_min_size(
        response.rect.min,
        Vec2::new(available_width, waveform_height)
    );
    
    let timeline_rect = Rect::from_min_size(
        Pos2::new(response.rect.min.x, response.rect.min.y + waveform_height),
        Vec2::new(available_width, timeline_height)
    );
    
    // Draw backgrounds
    painter.rect_filled(waveform_rect, 3.0, Color32::from_rgb(15, 20, 30));
    painter.rect_filled(timeline_rect, 0.0, Color32::from_rgb(10, 15, 25));
    
    // 7-second window parameters
    let visible_duration = 7.0;
    let samples_to_display = waveform_samples.len();
    let samples_per_second = samples_to_display as f32 / visible_duration;
    
    // Calculate which portion of the full waveform to show based on playback position
    // This creates the scrolling effect
    let progress_ratio = (pos_secs / dur_secs).clamp(0.0, 1.0);
    
    // Calculate window start time (what time is shown at the left edge)
    let window_start_time = if pos_secs <= 1.0 {
        // At beginning: playhead moves from 0 to 1 second
        0.0
    } else if pos_secs >= dur_secs - 6.0 {
        // Near end: show last 7 seconds
        (dur_secs - visible_duration).max(0.0)
    } else {
        // Middle: playhead locked at 1 second, window scrolls
        pos_secs - 1.0
    };
    
    let window_end_time = (window_start_time + visible_duration).min(dur_secs);
    
    // Playhead X position on screen
    let playhead_screen_x = if pos_secs <= 1.0 {
        // Playhead moves from left to the 1-second mark
        waveform_rect.min.x + (pos_secs / visible_duration) * available_width
    } else if pos_secs >= dur_secs - 6.0 {
        // Near end: playhead continues moving right
        waveform_rect.min.x + ((pos_secs - window_start_time) / visible_duration) * available_width
    } else {
        // Middle: playhead locked at 1-second mark
        waveform_rect.min.x + (1.0 / visible_duration) * available_width
    };
    
    // Calculate which samples from the buffer to display
    // The buffer represents the full song, we need to extract the visible window
    let buffer_progress = progress_ratio;
    let window_progress_start = (window_start_time / dur_secs).clamp(0.0, 1.0);
    let window_progress_end = (window_end_time / dur_secs).clamp(0.0, 1.0);
    
    // Get the window of samples from the full buffer
    let start_sample_idx = (window_progress_start * samples_to_display as f32) as usize;
    let end_sample_idx = ((window_progress_end * samples_to_display as f32) as usize)
        .min(samples_to_display);
    
    let visible_samples = if end_sample_idx > start_sample_idx {
        &waveform_samples[start_sample_idx..end_sample_idx]
    } else {
        &waveform_samples[..]
    };
    
    let visible_count = visible_samples.len().max(1);
    let pixels_per_sample = available_width / visible_count as f32;
    
    // Draw waveform bars
    for (i, &amplitude) in visible_samples.iter().enumerate() {
        let sample_time = window_start_time + (i as f32 / samples_per_second);
        let x = waveform_rect.min.x + (i as f32 * pixels_per_sample);
        
        let bar_height = (amplitude * waveform_height * 0.85).max(2.0);
        
        // Color based on playback position (cyan for played, gray for unplayed)
        let color = if sample_time <= pos_secs {
            // Played - bright cyan
            let intensity = (amplitude * 150.0) as u8 + 105;
            Color32::from_rgb(0, intensity, 255)
        } else {
            // Unplayed - dark gray
            let intensity = (amplitude * 60.0) as u8 + 50;
            Color32::from_rgb(40, 60, intensity)
        };
        
        // Draw bar from center outward (symmetric waveform)
        let center_y = waveform_rect.center().y;
        painter.rect_filled(
            Rect::from_min_max(
                Pos2::new(x, center_y - bar_height / 2.0),
                Pos2::new(x + pixels_per_sample.max(1.0), center_y + bar_height / 2.0)
            ),
            0.0,
            color
        );
    }
    
    // Draw playhead (red vertical line locked at 1-second mark during middle playback)
    painter.line_segment(
        [
            Pos2::new(playhead_screen_x, waveform_rect.min.y),
            Pos2::new(playhead_screen_x, waveform_rect.max.y)
        ],
        Stroke::new(3.0, Color32::from_rgb(255, 50, 50))
    );
    
    // Draw timeline with 1-second tick marks that scroll with the waveform
    let start_second = window_start_time.floor() as i32;
    let end_second = window_end_time.ceil() as i32;
    
    for tick_second in start_second..=end_second {
        let tick_time = tick_second as f32;
        
        if tick_time < window_start_time || tick_time > window_end_time {
            continue;
        }
        
        let time_offset = tick_time - window_start_time;
        let tick_x = waveform_rect.min.x + (time_offset / visible_duration) * available_width;
        
        // Draw tick mark
        painter.line_segment(
            [
                Pos2::new(tick_x, timeline_rect.min.y + 5.0),
                Pos2::new(tick_x, timeline_rect.min.y + 15.0)
            ],
            Stroke::new(2.0, Color32::from_rgb(150, 170, 200))
        );
        
        // Draw time label (MM:SS format)
        let minutes = tick_second / 60;
        let seconds = tick_second % 60;
        let time_text = format!("{}:{:02}", minutes, seconds);
        
        painter.text(
            Pos2::new(tick_x, timeline_rect.min.y + 18.0),
            egui::Align2::CENTER_TOP,
            time_text,
            egui::FontId::proportional(13.0),
            Color32::from_rgb(180, 200, 220)
        );
    }
    
    // Request repaint for smooth scrolling animation
    ui.ctx().request_repaint();
}
