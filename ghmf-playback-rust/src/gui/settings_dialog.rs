use super::theme;
use crate::config::Settings;
use crate::dmx::get_network_interfaces;
use egui::{Context, RichText};

pub fn show(ctx: &Context, open: &mut bool, settings: &mut Settings) {
    let should_close = std::cell::Cell::new(false);
    
    egui::Window::new("Settings")
        .open(open)
        .resizable(true)
        .default_width(500.0)
        .show(ctx, |ui| {
            ui.heading("Application Settings");
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            
            // Audio settings
            ui.group(|ui| {
                ui.label(RichText::new("Audio Settings").size(16.0).strong());
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label("Latency (ms):");
                    ui.add(egui::Slider::new(&mut settings.audio_latency_ms, 10..=500));
                });
                
                ui.label(RichText::new("Lower latency = better sync, but may cause audio glitches")
                    .size(11.0)
                    .color(theme::AppColors::TEXT_DISABLED));
            });
            
            ui.add_space(15.0);
            
            // DMX settings
            ui.group(|ui| {
                ui.label(RichText::new("DMX Settings").size(16.0).strong());
                ui.add_space(10.0);
                
                ui.checkbox(&mut settings.dmx_enabled, "Enable DMX Control");
                
                if !settings.dmx_enabled {
                    ui.label(RichText::new("DMX disabled - lighting controls will not function")
                        .color(theme::AppColors::WARNING));
                }
                
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label("Fixture Format:");
                    ui.radio_value(&mut settings.use_rgbw, true, "RGBW (4 channels)");
                    ui.radio_value(&mut settings.use_rgbw, false, "RGB (3 channels)");
                });
                
                ui.label(RichText::new("RGBW: Converts RGB to RGBW | RGB: Sets W channel to 0")
                    .size(11.0)
                    .color(theme::AppColors::TEXT_DISABLED));
            });
            
            ui.add_space(15.0);
            
            // PLC settings
            ui.group(|ui| {
                ui.label(RichText::new("PLC Settings").size(16.0).strong());
                ui.add_space(10.0);
                
                ui.checkbox(&mut settings.plc_enabled, "Enable PLC Communication");
                
                if settings.plc_enabled {
                    ui.horizontal(|ui| {
                        ui.label("IP Address:");
                        ui.text_edit_singleline(&mut settings.plc_ip_address);
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Port:");
                        let mut port_str = settings.plc_port.to_string();
                        if ui.text_edit_singleline(&mut port_str).changed() {
                            if let Ok(port_num) = port_str.parse::<u16>() {
                                settings.plc_port = port_num;
                            }
                        }
                    });
                } else {
                    ui.label(RichText::new("PLC disabled")
                        .color(theme::AppColors::TEXT_DISABLED));
                }
            });
            
            ui.add_space(15.0);
            
            // sACN / E.131 settings
            ui.group(|ui| {
                ui.label(RichText::new("DMX Controller").size(16.0).strong());
                ui.add_space(10.0);
                
                ui.checkbox(&mut settings.sacn_enabled, "Enable E.131 sACN Output");
                
                if settings.sacn_enabled {
                    ui.add_space(5.0);
                    
                    // Network interface selection
                    ui.horizontal(|ui| {
                        ui.label("Network Interface:");
                        
                        // Get available interfaces
                        let interfaces = get_network_interfaces();
                        
                        egui::ComboBox::from_id_salt("sacn_interface")
                            .selected_text(if settings.sacn_interface_ip.is_empty() {
                                "Select interface..."
                            } else {
                                &settings.sacn_interface_ip
                            })
                            .show_ui(ui, |ui| {
                                for (name, ip) in interfaces {
                                    let label = format!("{} ({})", name, ip);
                                    if ui.selectable_value(&mut settings.sacn_interface_ip, ip.clone(), label).clicked() {
                                        // Interface selected
                                    }
                                }
                            });
                    });
                    
                    ui.add_space(5.0);
                    
                    // Filter mode selection
                    ui.horizontal(|ui| {
                        ui.label("Output Mode:");
                        
                        egui::ComboBox::from_id_salt("sacn_filter_mode")
                            .selected_text(if settings.sacn_filter_mode == "900only" {
                                "900 Codes Only"
                            } else {
                                "All Lights"
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.sacn_filter_mode, "all".to_string(), "All Lights");
                                ui.selectable_value(&mut settings.sacn_filter_mode, "900only".to_string(), "900 Codes Only");
                            });
                    });
                    
                    ui.add_space(5.0);
                    
                    ui.label(RichText::new("Universe 1 • Priority 100")
                        .size(11.0)
                        .color(theme::AppColors::TEXT_DISABLED));
                    
                    if settings.sacn_interface_ip.is_empty() {
                        ui.label(RichText::new("⚠ Select a network interface to enable sACN output")
                            .size(11.0)
                            .color(theme::AppColors::WARNING));
                    }
                } else {
                    ui.label(RichText::new("sACN output disabled")
                        .color(theme::AppColors::TEXT_DISABLED));
                }
            });
            
            ui.add_space(20.0);
            ui.separator();
            ui.add_space(10.0);
            
            // Buttons
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    match settings.save() {
                        Ok(_) => {
                            should_close.set(true);
                        }
                        Err(e) => {
                            eprintln!("Failed to save settings: {}", e);
                        }
                    }
                }
                
                if ui.button("Cancel").clicked() {
                    *settings = Settings::load(); // Reload
                    should_close.set(true);
                }
                
                if ui.button("Reset to Defaults").clicked() {
                    *settings = Settings::default();
                }
            });
        });
    
    if should_close.get() {
        *open = false;
    }
}
