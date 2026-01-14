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
        show_full_waveform(ui, state, playback_position, playback_duration, audio_player);
        
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

/// Render full waveform with moving playhead
fn show_full_waveform(
    ui: &mut Ui,
    state: &PlaybackPanelState,
    playback_position: Duration,
    playback_duration: Duration,
    audio_player: &Option<Arc<Mutex<AudioPlayer>>>,
) {
    let waveform_height = 60.0;
    let available_width = ui.available_width() * 0.95;
    
    // Allocate space for waveform with click sensing
    let (response, painter) = ui.allocate_painter(
        Vec2::new(available_width, waveform_height),
        Sense::click()
    );
    
    let waveform_rect = response.rect;
    
    // Draw background
    painter.rect_filled(waveform_rect, 3.0, Color32::from_rgb(15, 20, 30));
    
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
    
    // Draw playhead (red vertical line) - moves from left to right across full width
    painter.line_segment(
        [
            Pos2::new(playhead_x, waveform_rect.min.y),
            Pos2::new(playhead_x, waveform_rect.max.y)
        ],
        Stroke::new(3.0, Color32::from_rgb(255, 80, 80))
    );
}
