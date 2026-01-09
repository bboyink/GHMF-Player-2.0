use egui::{Context, Ui, Vec2, Rect, Pos2, Color32, Stroke, FontId, FontFamily, TextureHandle, ColorImage};
use super::theme::AppColors;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Operator,
    Testing,
    Playlist,
    DmxMap,
    Settings,
}

impl AppView {
    pub fn icon(&self) -> &'static str {
        match self {
            AppView::Operator => "▶",    // Play icon
            AppView::Testing => "⚡",     // Testing/bolt icon
            AppView::Playlist => "♫",    // Music note icon
            AppView::DmxMap => "◉",      // DMX grid icon
            AppView::Settings => "⚙",    // Settings gear icon
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            AppView::Operator => "Operator",
            AppView::Testing => "Testing",
            AppView::Playlist => "Playlist",
            AppView::DmxMap => "DMX Map",
            AppView::Settings => "Settings",
        }
    }
    
    pub fn tooltip(&self) -> &'static str {
        match self {
            AppView::Operator => "Operator Mode - Playback Controls",
            AppView::Testing => "Testing Mode - Light & System Testing",
            AppView::Playlist => "Playlist Manager",
            AppView::DmxMap => "DMX Mapper - Assign Fixtures to DMX Channels",
            AppView::Settings => "Application Settings",
        }
    }
}

pub struct Sidebar {
    pub selected_view: AppView,
    pub collapsed: bool,
    logo_texture: Option<Arc<TextureHandle>>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            selected_view: AppView::Operator,
            collapsed: false,
            logo_texture: None,
        }
    }
}

impl Sidebar {
    /// Load the logo texture from embedded image bytes
    fn load_logo(&mut self, ctx: &Context) {
        if self.logo_texture.is_some() {
            return;
        }

        // Load the embedded logo image
        let logo_bytes = include_bytes!("../../assets/logo.png");
        
        if let Ok(image) = image::load_from_memory(logo_bytes) {
            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
            
            let texture = ctx.load_texture(
                "ghmf_logo",
                color_image,
                Default::default()
            );
            
            self.logo_texture = Some(Arc::new(texture));
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui) -> Option<AppView> {
        // Load logo texture if not already loaded
        self.load_logo(ctx);
        
        let mut clicked_view = None;
        
        // Set dark background explicitly for sidebar
        let sidebar_frame = egui::Frame::none()
            .fill(AppColors::BACKGROUND_LIGHT)
            .inner_margin(0.0);
        
        sidebar_frame.show(ui, |ui| {
            ui.set_min_width(160.0);
            ui.set_max_width(160.0);
            
            // Header with logo/title
            ui.add_space(10.0);
            ui.vertical_centered(|ui| {
                // Show logo
                if let Some(logo) = &self.logo_texture {
                    // Calculate width to maintain aspect ratio
                    let aspect_ratio = logo.size()[0] as f32 / logo.size()[1] as f32;
                    let logo_height = 80.0;
                    let logo_width = logo_height * aspect_ratio;
                    let logo_size = Vec2::new(logo_width, logo_height);
                    ui.add(egui::Image::new(logo.as_ref()).fit_to_exact_size(logo_size));
                } else {
                    // Fallback text
                    ui.label(
                        egui::RichText::new("GHMF Playback")
                            .size(18.0)
                            .strong()
                            .color(AppColors::CYAN)
                    );
                }
                ui.label(
                    egui::RichText::new("v2.0")
                        .size(11.0)
                        .color(AppColors::TEXT_SECONDARY)
                );
            });
            
            ui.add_space(10.0);
            
            // Separator
            ui.add(egui::Separator::default().spacing(0.0));
            
            ui.add_space(10.0);
            
            // Navigation buttons
            let views = [
                AppView::Operator,
                AppView::Testing,
                AppView::Playlist,
                AppView::DmxMap,
                AppView::Settings,
            ];
            
            for view in views {
                if self.nav_button(ui, view) {
                    clicked_view = Some(view);
                }
            }
            

        });
        
        clicked_view
    }
    
    fn nav_button(&mut self, ui: &mut Ui, view: AppView) -> bool {
        let is_selected = self.selected_view == view;
        
        let button_color = if is_selected {
            Color32::from_rgba_unmultiplied(0, 198, 255, 40) // Cyan with alpha
        } else {
            Color32::TRANSPARENT
        };
        
        let hover_color = if is_selected {
            Color32::from_rgba_unmultiplied(0, 198, 255, 60)
        } else {
            AppColors::SURFACE
        };
        
        let text_color = if is_selected {
            Color32::WHITE
        } else {
            AppColors::TEXT_SECONDARY
        };
        
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), 48.0),
            egui::Sense::click()
        );
        
        // Draw background with rounded corners
        if response.hovered() {
            ui.painter().rect_filled(
                rect.shrink(4.0),
                8.0,
                hover_color
            );
        } else if is_selected {
            ui.painter().rect_filled(
                rect.shrink(4.0),
                8.0,
                button_color
            );
        }
        
        // Draw selection indicator (left border) as a rounded rect
        if is_selected {
            let indicator_rect = Rect::from_min_max(
                Pos2::new(rect.min.x + 6.0, rect.min.y + 10.0),
                Pos2::new(rect.min.x + 10.0, rect.max.y - 10.0)
            );
            ui.painter().rect_filled(indicator_rect, 2.0, AppColors::CYAN);
        }
        
        // Draw icon and label
        if self.collapsed {
            // Centered icon only
            let icon_pos = rect.center();
            ui.painter().text(
                icon_pos,
                egui::Align2::CENTER_CENTER,
                view.icon(),
                FontId::new(26.0, FontFamily::Proportional),
                text_color
            );
        } else {
            // Icon + label
            let icon_pos = Pos2::new(rect.min.x + 24.0, rect.center().y);
            ui.painter().text(
                icon_pos,
                egui::Align2::LEFT_CENTER,
                view.icon(),
                FontId::new(22.0, FontFamily::Proportional),
                text_color
            );
            
            let label_pos = Pos2::new(rect.min.x + 56.0, rect.center().y);
            ui.painter().text(
                label_pos,
                egui::Align2::LEFT_CENTER,
                view.label(),
                FontId::new(15.0, FontFamily::Proportional),
                text_color
            );
        }
        
        let tooltip_response = response.on_hover_text(view.tooltip());
        
        if tooltip_response.clicked() {
            self.selected_view = view;
            true
        } else {
            false
        }
    }
}
