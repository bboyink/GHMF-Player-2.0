use super::theme;
use crate::audio::AudioPlayer;
use egui::{Ui, RichText, Slider, Button};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub fn show(
    ui: &mut Ui,
    is_playing: &mut bool,
    is_paused: &mut bool,
    master_volume: &mut f32,
    playback_position: Duration,
    playback_duration: Duration,
    current_song: &str,
    current_playlist: &str,
    audio_player: &Option<Arc<Mutex<AudioPlayer>>>,
) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        
        // Current song display
        ui.heading(RichText::new(current_song).size(24.0).color(theme::AppColors::TEXT_PRIMARY));
        ui.label(RichText::new(current_playlist).size(14.0).color(theme::AppColors::TEXT_SECONDARY));
        
        ui.add_space(20.0);
        
        // Progress bar
        let pos_secs = playback_position.as_secs_f32();
        let dur_secs = playback_duration.as_secs_f32().max(1.0);
        let progress = (pos_secs / dur_secs).clamp(0.0, 1.0);
        
        ui.add(
            egui::ProgressBar::new(progress)
                .text(format_duration(playback_position))
                .desired_width(ui.available_width() * 0.8)
        );
        
        ui.add_space(10.0);
        
        // Time display
        ui.horizontal(|ui| {
            ui.label(format_duration(playback_position));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format_duration(playback_duration));
            });
        });
        
        ui.add_space(30.0);
        
        // Playback controls
        ui.horizontal(|ui| {
            ui.add_space((ui.available_width() - 300.0) / 2.0);
            
            // Previous button
            if ui.add_sized([60.0, 50.0], Button::new("⏮")).clicked() {
                // TODO: Previous song
            }
            
            ui.add_space(10.0);
            
            // Play/Pause button
            let play_pause_text = if *is_playing && !*is_paused {
                "⏸"
            } else {
                "▶"
            };
            
            if ui.add_sized([80.0, 50.0], Button::new(RichText::new(play_pause_text).size(24.0))).clicked() {
                if let Some(player) = audio_player {
                    if let Ok(player) = player.lock() {
                        if *is_playing && !*is_paused {
                            player.pause();
                        } else {
                            player.resume();
                        }
                    }
                }
            }
            
            ui.add_space(10.0);
            
            // Stop button
            if ui.add_sized([60.0, 50.0], Button::new("⏹")).clicked() {
                if let Some(player) = audio_player {
                    if let Ok(player) = player.lock() {
                        player.stop();
                    }
                }
            }
            
            ui.add_space(10.0);
            
            // Next button
            if ui.add_sized([60.0, 50.0], Button::new("⏭")).clicked() {
                // TODO: Next song
            }
        });
        
        ui.add_space(40.0);
        
        // Volume control
        ui.horizontal(|ui| {
            ui.label("Volume:");
            ui.add_space(10.0);
            
            let mut volume_changed = false;
            let volume_percent = (*master_volume * 100.0) as i32;
            let slider = Slider::new(master_volume, 0.0..=1.0)
                .text(format!("{}%", volume_percent));
            
            if ui.add(slider).changed() {
                volume_changed = true;
            }
            
            if volume_changed {
                if let Some(player) = audio_player {
                    if let Ok(player) = player.lock() {
                        player.set_volume(*master_volume);
                    }
                }
            }
        });
        
        ui.add_space(20.0);
        
        // VU Meter (placeholder)
        ui.group(|ui| {
            ui.set_min_width(ui.available_width() * 0.8);
            ui.vertical(|ui| {
                ui.label(RichText::new("Audio Level").size(12.0).color(theme::AppColors::TEXT_SECONDARY));
                ui.add_space(5.0);
                
                // Left channel
                ui.horizontal(|ui| {
                    ui.label("L");
                    let level = if *is_playing { 0.6 } else { 0.0 };
                    ui.add(egui::ProgressBar::new(level).desired_width(ui.available_width() - 30.0));
                });
                
                // Right channel
                ui.horizontal(|ui| {
                    ui.label("R");
                    let level = if *is_playing { 0.65 } else { 0.0 };
                    ui.add(egui::ProgressBar::new(level).desired_width(ui.available_width() - 30.0));
                });
            });
        });
    });
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
