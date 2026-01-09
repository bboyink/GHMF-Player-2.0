use super::theme;
use crate::dmx::EnttecDmxPro;
use egui::{Ui, RichText, Slider, Color32};
use std::sync::{Arc, Mutex};

pub fn show(
    ui: &mut Ui,
    selected_fixture: &mut Option<usize>,
    fixture_red: &mut u8,
    fixture_green: &mut u8,
    fixture_blue: &mut u8,
    dmx_controller: &Option<Arc<Mutex<EnttecDmxPro>>>,
) {
    ui.vertical(|ui| {
        ui.heading(RichText::new("Lighting Control").size(18.0));
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
        
        // Fixture selector
        ui.group(|ui| {
            ui.label(RichText::new("Fixture").size(14.0).strong());
            ui.add_space(5.0);
            
            ui.horizontal(|ui| {
                if ui.button("1").clicked() {
                    *selected_fixture = Some(1);
                }
                if ui.button("2").clicked() {
                    *selected_fixture = Some(2);
                }
                if ui.button("3").clicked() {
                    *selected_fixture = Some(3);
                }
                if ui.button("4").clicked() {
                    *selected_fixture = Some(4);
                }
                if ui.button("5").clicked() {
                    *selected_fixture = Some(5);
                }
            });
            
            ui.horizontal(|ui| {
                if ui.button("6").clicked() {
                    *selected_fixture = Some(6);
                }
                if ui.button("7").clicked() {
                    *selected_fixture = Some(7);
                }
                if ui.button("8").clicked() {
                    *selected_fixture = Some(8);
                }
                if ui.button("9").clicked() {
                    *selected_fixture = Some(9);
                }
                if ui.button("10").clicked() {
                    *selected_fixture = Some(10);
                }
            });
            
            if let Some(fixture) = selected_fixture {
                ui.label(RichText::new(format!("Selected: Fixture {}", fixture))
                    .color(theme::AppColors::PRIMARY_LIGHT));
            } else {
                ui.label(RichText::new("No fixture selected")
                    .color(theme::AppColors::TEXT_DISABLED));
            }
        });
        
        ui.add_space(15.0);
        
        // RGB controls
        if selected_fixture.is_some() {
            ui.group(|ui| {
                ui.label(RichText::new("RGB Color").size(14.0).strong());
                ui.add_space(10.0);
                
                // Red slider
                ui.horizontal(|ui| {
                    ui.label("R:");
                    let mut red_f = *fixture_red as f32;
                    if ui.add(Slider::new(&mut red_f, 0.0..=255.0)).changed() {
                        *fixture_red = red_f as u8;
                        update_dmx(selected_fixture, *fixture_red, *fixture_green, *fixture_blue, dmx_controller);
                    }
                });
                
                // Green slider
                ui.horizontal(|ui| {
                    ui.label("G:");
                    let mut green_f = *fixture_green as f32;
                    if ui.add(Slider::new(&mut green_f, 0.0..=255.0)).changed() {
                        *fixture_green = green_f as u8;
                        update_dmx(selected_fixture, *fixture_red, *fixture_green, *fixture_blue, dmx_controller);
                    }
                });
                
                // Blue slider
                ui.horizontal(|ui| {
                    ui.label("B:");
                    let mut blue_f = *fixture_blue as f32;
                    if ui.add(Slider::new(&mut blue_f, 0.0..=255.0)).changed() {
                        *fixture_blue = blue_f as u8;
                        update_dmx(selected_fixture, *fixture_red, *fixture_green, *fixture_blue, dmx_controller);
                    }
                });
                
                ui.add_space(10.0);
                
                // Color preview
                let color = Color32::from_rgb(*fixture_red, *fixture_green, *fixture_blue);
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width(), 60.0),
                    egui::Sense::hover()
                );
                ui.painter().rect_filled(rect, 4.0, color);
                
                ui.add_space(5.0);
                ui.label(format!("#{:02X}{:02X}{:02X}", fixture_red, fixture_green, fixture_blue));
            });
            
            ui.add_space(15.0);
            
            // Quick colors
            ui.group(|ui| {
                ui.label(RichText::new("Quick Colors").size(14.0).strong());
                ui.add_space(5.0);
                
                ui.horizontal(|ui| {
                    if ui.button("Off").clicked() {
                        *fixture_red = 0;
                        *fixture_green = 0;
                        *fixture_blue = 0;
                        update_dmx(selected_fixture, 0, 0, 0, dmx_controller);
                    }
                    if ui.button("White").clicked() {
                        *fixture_red = 255;
                        *fixture_green = 255;
                        *fixture_blue = 255;
                        update_dmx(selected_fixture, 255, 255, 255, dmx_controller);
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("Red").clicked() {
                        *fixture_red = 255;
                        *fixture_green = 0;
                        *fixture_blue = 0;
                        update_dmx(selected_fixture, 255, 0, 0, dmx_controller);
                    }
                    if ui.button("Green").clicked() {
                        *fixture_red = 0;
                        *fixture_green = 255;
                        *fixture_blue = 0;
                        update_dmx(selected_fixture, 0, 255, 0, dmx_controller);
                    }
                    if ui.button("Blue").clicked() {
                        *fixture_red = 0;
                        *fixture_green = 0;
                        *fixture_blue = 255;
                        update_dmx(selected_fixture, 0, 0, 255, dmx_controller);
                    }
                });
                
                ui.horizontal(|ui| {
                    if ui.button("Yellow").clicked() {
                        *fixture_red = 255;
                        *fixture_green = 255;
                        *fixture_blue = 0;
                        update_dmx(selected_fixture, 255, 255, 0, dmx_controller);
                    }
                    if ui.button("Cyan").clicked() {
                        *fixture_red = 0;
                        *fixture_green = 255;
                        *fixture_blue = 255;
                        update_dmx(selected_fixture, 0, 255, 255, dmx_controller);
                    }
                    if ui.button("Magenta").clicked() {
                        *fixture_red = 255;
                        *fixture_green = 0;
                        *fixture_blue = 255;
                        update_dmx(selected_fixture, 255, 0, 255, dmx_controller);
                    }
                });
            });
        } else {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                ui.label(RichText::new("Select a fixture to control")
                    .size(14.0)
                    .color(theme::AppColors::TEXT_DISABLED));
            });
        }
        
        ui.add_space(20.0);
        
        // All off button
        if ui.button("🔴 All Lights Off").clicked() {
            if let Some(dmx) = dmx_controller {
                if let Ok(mut dmx) = dmx.lock() {
                    dmx.clear();
                    let _ = dmx.send_dmx();
                }
            }
            *fixture_red = 0;
            *fixture_green = 0;
            *fixture_blue = 0;
        }
    });
}

fn update_dmx(
    selected_fixture: &Option<usize>,
    red: u8,
    green: u8,
    blue: u8,
    dmx_controller: &Option<Arc<Mutex<EnttecDmxPro>>>,
) {
    if let Some(fixture) = selected_fixture {
        if let Some(dmx) = dmx_controller {
            if let Ok(mut dmx) = dmx.lock() {
                // Assuming RGB fixtures with 3 consecutive channels
                let base_channel = (*fixture - 1) * 3 + 1;
                let _ = dmx.set_channel(base_channel, red);
                let _ = dmx.set_channel(base_channel + 1, green);
                let _ = dmx.set_channel(base_channel + 2, blue);
                let _ = dmx.send_dmx();
            }
        }
    }
}
