use super::theme;
use crate::audio::{AudioPlayer, WaveformData};
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
                self.waveform_data = Some(waveform);
            }
            Err(e) => {
                warn!("Failed to load waveform data: {}", e);
                // Use placeholder on error
                self.waveform_data = Some(WaveformData::placeholder(100));
            }
        }
    }
    
    /// Clear waveform data (when no song is loaded)
    pub fn clear_waveform(&mut self) {
        self.waveform_data = None;
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
        let waveform_height = 100.0;
        let timeline_height = 30.0;
        let available_width = ui.available_width() * 0.9;
        
        // Combined waveform + timeline area
        let total_height = waveform_height + timeline_height;
        let (combined_rect, _combined_response) = ui.allocate_exact_size(
            Vec2::new(available_width, total_height),
            Sense::hover()
        );
        
        // Always draw waveform and timeline (don't check visibility)
        let painter = ui.painter();
        
        // Split into waveform and timeline sections
        let waveform_rect = Rect::from_min_max(
                combined_rect.min,
                Pos2::new(combined_rect.max.x, combined_rect.min.y + waveform_height)
            );
            let timeline_rect = Rect::from_min_max(
                Pos2::new(combined_rect.min.x, combined_rect.min.y + waveform_height),
                combined_rect.max
            );
            
            // Background for waveform
            painter.rect_filled(
                waveform_rect,
                3.0,
                Color32::from_rgb(15, 20, 30)
            );
            
            // Background for timeline
            painter.rect_filled(
                timeline_rect,
                0.0,
                Color32::from_rgb(10, 15, 25)
            );
            
            // Get waveform data
            let placeholder = vec![0.3; 100];
            let waveform_samples = if let Some(ref wf) = state.waveform_data {
                &wf.samples
            } else {
                &placeholder
            };
            
            let pos_secs = playback_position.as_secs_f32();
            let dur_secs = playback_duration.as_secs_f32().max(1.0);
            
            // 7-second scrolling window with constant zoom
            let visible_duration = 7.0_f32;
            let total_samples = waveform_samples.len();
            let samples_per_second = total_samples as f32 / dur_secs;
            
            // Window positioning: playhead stays at 1 second mark, window scrolls
            let window_start = if pos_secs <= 1.0 { 
                0.0 
            } else if pos_secs >= dur_secs - 6.0 {
                // Near end: show last 7 seconds
                (dur_secs - visible_duration).max(0.0)
            } else { 
                pos_secs - 1.0 
            };
            
            let window_end = (window_start + visible_duration).min(dur_secs);
            let actual_visible_duration = window_end - window_start;
            
            // Playhead position within the window
            let playhead_offset = if pos_secs < 1.0 { 
                pos_secs - window_start
            } else if pos_secs > dur_secs - 6.0 {
                pos_secs - window_start
            } else { 
                1.0 
            };
            
            // Calculate samples for visible window
            let start_sample = (window_start * samples_per_second) as usize;
            let end_sample = ((window_end * samples_per_second) as usize).min(total_samples);
            let visible_samples = &waveform_samples[start_sample..end_sample];
            
            // Bar width: divide available width by NUMBER of samples in window
            // This ensures we only show the 7-second window samples
            let num_visible_samples = visible_samples.len();
            let bar_width = if num_visible_samples > 0 {
                available_width / num_visible_samples as f32
            } else {
                1.0
            };
            
            for (i, &amplitude) in visible_samples.iter().enumerate() {
                let sample_idx = start_sample + i;
                let sample_time = (sample_idx as f32 / total_samples as f32) * dur_secs;
                
                let x = waveform_rect.min.x + (i as f32 * bar_width);
                
                // Bar height represents RMS volume (grows from bottom up)
                let bar_height = (amplitude * waveform_height * 0.9).max(2.0);
                
                // Color: cyan/blue based on whether it's been played
                let color = if sample_time <= pos_secs {
                    // Already played - bright cyan
                    let intensity = (amplitude * 150.0) as u8 + 105;
                    Color32::from_rgb(0, intensity, 255)
                } else {
                    // Not yet played - darker
                    let intensity = (amplitude * 80.0) as u8 + 60;
                    Color32::from_rgb(50, 70, intensity)
                };
                
                // Draw bar from bottom up
                painter.rect_filled(
                    Rect::from_min_max(
                        Pos2::new(x, waveform_rect.max.y - bar_height),
                        Pos2::new(x + bar_width - 1.0, waveform_rect.max.y)
                    ),
                    0.0,
                    color
                );
            }
            
            // Draw playhead (red vertical line) - stays at fixed position (1 second mark)
            let playhead_x = waveform_rect.min.x + (playhead_offset / actual_visible_duration) * available_width;
            painter.line_segment(
                [
                    Pos2::new(playhead_x, waveform_rect.min.y),
                    Pos2::new(playhead_x, waveform_rect.max.y)
                ],
                Stroke::new(2.0, Color32::from_rgb(255, 60, 60))
            );
            
            // Draw timeline below waveform - shows visible window with ticks every second
            let timeline_y = timeline_rect.min.y + 2.0;
            
            // Draw tick marks for each second in the visible window
            let start_tick = window_start.floor() as i32;
            let end_tick = window_end.ceil() as i32;
            
            for tick_sec in start_tick..=end_tick {
                let tick_time = tick_sec as f32;
                
                if tick_time < window_start || tick_time > window_end {
                    continue;
                }
                
                let x_pos = waveform_rect.min.x + ((tick_time - window_start) / actual_visible_duration) * available_width;
                
                // Draw tick mark (more visible)
                painter.line_segment(
                    [
                        Pos2::new(x_pos, timeline_y),
                        Pos2::new(x_pos, timeline_y + 8.0)
                    ],
                    Stroke::new(2.0, Color32::from_rgb(150, 170, 190))
                );
                
                // Draw time label every second
                let minutes = (tick_time / 60.0) as i32;
                let seconds = (tick_time % 60.0) as i32;
                let time_text = format!("{}:{:02}", minutes, seconds);
                
                painter.text(
                    Pos2::new(x_pos, timeline_y + 10.0),
                    egui::Align2::CENTER_TOP,
                    time_text,
                    egui::FontId::proportional(12.0),
                    Color32::from_rgb(180, 190, 210)
                );
            }
        
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
