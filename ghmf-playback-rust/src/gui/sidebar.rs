use egui::{Context, Ui, Vec2, Rect, Pos2, Color32, Stroke, FontId, FontFamily, TextureHandle, ColorImage};
use super::theme::AppColors;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppView {
    Operator,
    Testing,
    Playlist,
    Settings,
    SettingsDmxMap,
    SettingsLightGroups,
    SettingsLightsLayout,
    SettingsLegacyColor,
    SettingsStartTime,
    SettingsProcedures,
    SettingsApp,
}

impl AppView {
    pub fn icon(&self) -> &'static str {
        match self {
            AppView::Operator => "▶",    // Play icon
            AppView::Testing => "⚡",     // Testing/bolt icon
            AppView::Playlist => "♫",    // Music note icon
            AppView::Settings => "⚙",    // Settings gear icon
            AppView::SettingsDmxMap => "◉",      // DMX grid icon
            AppView::SettingsLightGroups => "💡", // Light bulb icon
            AppView::SettingsLightsLayout => "▦", // Grid layout icon
            AppView::SettingsLegacyColor => "🎨", // Palette icon
            AppView::SettingsStartTime => "🕐",   // Clock icon
            AppView::SettingsProcedures => "📋",   // Checklist icon
            AppView::SettingsApp => "⚙",    // Settings gear icon
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            AppView::Operator => "Operator",
            AppView::Testing => "Testing",
            AppView::Playlist => "Playlist",
            AppView::Settings => "Settings",
            AppView::SettingsDmxMap => "DMX Map",
            AppView::SettingsLightGroups => "Light Groups",
            AppView::SettingsLightsLayout => "Lights Layout",
            AppView::SettingsLegacyColor => "Legacy Color",
            AppView::SettingsStartTime => "Start Time",
            AppView::SettingsProcedures => "Procedures",
            AppView::SettingsApp => "Application",
        }
    }
    
    pub fn tooltip(&self) -> &'static str {
        match self {
            AppView::Operator => "Operator Mode - Playback Controls",
            AppView::Testing => "Testing Mode - Light & System Testing",
            AppView::Playlist => "Playlist Manager",
            AppView::Settings => "Settings Menu",
            AppView::SettingsDmxMap => "DMX Mapper - Assign Fixtures to DMX Channels",
            AppView::SettingsLightGroups => "Light Group Mapping - Create FWC Light Groups",
            AppView::SettingsLightsLayout => "Lights Layout - Visual Layout for Operator Screen",
            AppView::SettingsLegacyColor => "Legacy Color Mapping - Map FWC Values to RGB Colors",
            AppView::SettingsStartTime => "Start Time Configuration - Set Show Date and Time",
            AppView::SettingsProcedures => "Procedures - Configure Pre-Show Reminders",
            AppView::SettingsApp => "Application Settings",
        }
    }
}

pub struct Sidebar {
    pub selected_view: AppView,
    pub collapsed: bool,
    pub settings_expanded: bool,
    logo_texture: Option<Arc<TextureHandle>>,
    light_groups_icon: Option<Arc<TextureHandle>>,
    lights_layout_icon: Option<Arc<TextureHandle>>,
    app_settings_icon: Option<Arc<TextureHandle>>,
    dmx_map_icon: Option<Arc<TextureHandle>>,
    sort_down_icon: Option<Arc<TextureHandle>>,
    palette_icon: Option<Arc<TextureHandle>>,
    clock_icon: Option<Arc<TextureHandle>>,
    checklist_icon: Option<Arc<TextureHandle>>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            selected_view: AppView::Operator,
            collapsed: false,
            settings_expanded: false,
            logo_texture: None,
            light_groups_icon: None,
            lights_layout_icon: None,
            app_settings_icon: None,
            dmx_map_icon: None,
            sort_down_icon: None,
            palette_icon: None,
            clock_icon: None,
            checklist_icon: None,
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

    /// Load icon textures from embedded image bytes
    fn load_icons(&mut self, ctx: &Context) {
        // Load Light Groups icon
        if self.light_groups_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/led-light.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "light_groups_icon",
                    color_image,
                    Default::default()
                );
                
                self.light_groups_icon = Some(Arc::new(texture));
            }
        }

        // Load Application Settings icon
        if self.app_settings_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/window_settings.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "app_settings_icon",
                    color_image,
                    Default::default()
                );
                
                self.app_settings_icon = Some(Arc::new(texture));
            }
        }

        // Load DMX Map icon
        if self.dmx_map_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/switches.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "dmx_map_icon",
                    color_image,
                    Default::default()
                );
                
                self.dmx_map_icon = Some(Arc::new(texture));
            }
        }

        // Load Palette icon
        if self.palette_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/palette.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "palette_icon",
                    color_image,
                    Default::default()
                );
                
                self.palette_icon = Some(Arc::new(texture));
            }
        }

        // Load Sort Down icon
        if self.sort_down_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/sort_down.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "sort_down_icon",
                    color_image,
                    Default::default()
                );
                
                self.sort_down_icon = Some(Arc::new(texture));
            }
        }

        // Load Clock icon
        if self.clock_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/clock.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "clock_icon",
                    color_image,
                    Default::default()
                );
                
                self.clock_icon = Some(Arc::new(texture));
            }
        }

        // Load Grid icon for Lights Layout
        if self.lights_layout_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/grid.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "lights_layout_icon",
                    color_image,
                    Default::default()
                );
                
                self.lights_layout_icon = Some(Arc::new(texture));
            }
        }

        // Load Checklist icon
        if self.checklist_icon.is_none() {
            let icon_bytes = include_bytes!("../../assets/check_list.png");
            if let Ok(image) = image::load_from_memory(icon_bytes) {
                let size = [image.width() as _, image.height() as _];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
                
                let texture = ctx.load_texture(
                    "checklist_icon",
                    color_image,
                    Default::default()
                );
                
                self.checklist_icon = Some(Arc::new(texture));
            }
        }
    }

    pub fn show(&mut self, ctx: &Context, ui: &mut Ui, is_playing: bool) -> Option<AppView> {
        // Load logo texture if not already loaded
        self.load_logo(ctx);
        self.load_icons(ctx);
        
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
                        egui::RichText::new("Fountain Director")
                            .size(18.0)
                            .strong()
                            .color(AppColors::CYAN)
                    );
                }
                ui.label(
                    egui::RichText::new("v1.0")
                        .size(16.0)
                        .strong()
                        .color(AppColors::CYAN)
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
            ];
            
            for view in views {
                if self.nav_button(ui, view, is_playing) {
                    clicked_view = Some(view);
                }
            }
            
            // Settings menu with submenus
            if self.settings_menu_button(ui, is_playing) {
                self.settings_expanded = !self.settings_expanded;
            }
            
            // Show submenus if expanded
            if self.settings_expanded {
                let subviews = [
                    AppView::SettingsDmxMap,
                    AppView::SettingsLightGroups,
                    AppView::SettingsLightsLayout,
                    AppView::SettingsLegacyColor,
                    AppView::SettingsStartTime,
                    AppView::SettingsProcedures,
                    AppView::SettingsApp,
                ];
                
                for view in subviews {
                    if self.submenu_button(ui, view, is_playing) {
                        clicked_view = Some(view);
                    }
                }
            }
            

        });
        
        clicked_view
    }
    
    fn nav_button(&mut self, ui: &mut Ui, view: AppView, is_playing: bool) -> bool {
        let is_selected = self.selected_view == view;
        
        // Disable all buttons except Operator when playing
        let is_disabled = is_playing && view != AppView::Operator;
        
        let button_color = if is_selected {
            Color32::from_rgba_unmultiplied(0, 198, 255, 40) // Cyan with alpha
        } else {
            Color32::TRANSPARENT
        };
        
        let hover_color = if is_disabled {
            button_color // No hover effect when disabled
        } else if is_selected {
            Color32::from_rgba_unmultiplied(0, 198, 255, 60)
        } else {
            AppColors::SURFACE
        };
        
        let text_color = if is_disabled {
            Color32::from_rgba_unmultiplied(100, 100, 100, 120) // Dimmed when disabled
        } else if is_selected {
            Color32::WHITE
        } else {
            AppColors::TEXT_SECONDARY
        };
        
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), 48.0),
            egui::Sense::click()
        );
        
        // Draw background with rounded corners
        if !is_disabled && response.hovered() {
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
        if is_selected && !is_disabled {
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
    
    fn settings_menu_button(&mut self, ui: &mut Ui, is_playing: bool) -> bool {
        // Don't highlight Settings button when a submenu is selected
        let is_selected = false;
        let is_disabled = is_playing;
        
        let button_color = Color32::TRANSPARENT;
        
        let hover_color = if is_disabled {
            button_color
        } else {
            AppColors::SURFACE
        };
        
        let text_color = if is_disabled {
            Color32::from_rgba_unmultiplied(100, 100, 100, 120)
        } else {
            AppColors::TEXT_SECONDARY
        };
        
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), 48.0),
            egui::Sense::click()
        );
        
        if !is_disabled && response.hovered() {
            ui.painter().rect_filled(
                rect.shrink(4.0),
                8.0,
                hover_color
            );
        }
        
        // Icon
        let icon_pos = Pos2::new(rect.min.x + 24.0, rect.center().y);
        ui.painter().text(
            icon_pos,
            egui::Align2::LEFT_CENTER,
            "⚙",
            FontId::new(22.0, FontFamily::Proportional),
            text_color
        );
        
        // Label
        let label_pos = Pos2::new(rect.min.x + 56.0, rect.center().y);
        ui.painter().text(
            label_pos,
            egui::Align2::LEFT_CENTER,
            "Settings",
            FontId::new(15.0, FontFamily::Proportional),
            text_color
        );
        
        // Expand/collapse arrow - use image when expanded
        let arrow_size = 12.0;
        let arrow_pos = Pos2::new(rect.max.x - 20.0, rect.center().y);
        
        if self.settings_expanded {
            if let Some(icon) = &self.sort_down_icon {
                let icon_rect = Rect::from_center_size(
                    arrow_pos,
                    Vec2::new(arrow_size, arrow_size)
                );
                ui.painter().image(
                    icon.id(),
                    icon_rect,
                    Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                    Color32::WHITE
                );
            } else {
                ui.painter().text(
                    arrow_pos,
                    egui::Align2::CENTER_CENTER,
                    "▼",
                    FontId::new(arrow_size, FontFamily::Proportional),
                    text_color
                );
            }
        } else {
            ui.painter().text(
                arrow_pos,
                egui::Align2::CENTER_CENTER,
                "▶",
                FontId::new(arrow_size, FontFamily::Proportional),
                text_color
            );
        }
        
        !is_disabled && response.on_hover_text("Settings Menu").clicked()
    }
    
    fn submenu_button(&mut self, ui: &mut Ui, view: AppView, is_playing: bool) -> bool {
        let is_disabled = is_playing;
        let is_selected = self.selected_view == view;
        
        let button_color = if is_selected {
            Color32::from_rgba_unmultiplied(0, 198, 255, 40)
        } else {
            Color32::TRANSPARENT
        };
        
        let hover_color = if is_selected {
            Color32::from_rgba_unmultiplied(0, 198, 255, 60)
        } else {
            AppColors::SURFACE
        };
        
        let text_color = if is_disabled {
            Color32::from_rgba_unmultiplied(100, 100, 100, 120)
        } else if is_selected {
            Color32::WHITE
        } else {
            AppColors::TEXT_SECONDARY
        };
        
        let (rect, response) = ui.allocate_exact_size(
            Vec2::new(ui.available_width(), 40.0),
            egui::Sense::click()
        );
        
        if !is_disabled && response.hovered() {
            ui.painter().rect_filled(
                rect.shrink2(Vec2::new(8.0, 2.0)),
                6.0,
                hover_color
            );
        } else if is_selected {
            ui.painter().rect_filled(
                rect.shrink2(Vec2::new(8.0, 2.0)),
                6.0,
                button_color
            );
        }
        
        // Icon (smaller and indented) - use image for DMX Map, Light Groups and App Settings
        let icon_size = 18.0;
        let icon_pos = Pos2::new(rect.min.x + 40.0, rect.center().y);
        
        match view {
            AppView::SettingsDmxMap => {
                if let Some(icon) = &self.dmx_map_icon {
                    let icon_rect = Rect::from_center_size(
                        icon_pos,
                        Vec2::new(icon_size, icon_size)
                    );
                    ui.painter().image(
                        icon.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE
                    );
                } else {
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        view.icon(),
                        FontId::new(icon_size, FontFamily::Proportional),
                        text_color
                    );
                }
            }
            AppView::SettingsLightGroups => {
                if let Some(icon) = &self.light_groups_icon {
                    let icon_rect = Rect::from_center_size(
                        icon_pos,
                        Vec2::new(icon_size, icon_size)
                    );
                    ui.painter().image(
                        icon.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE
                    );
                } else {
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        view.icon(),
                        FontId::new(icon_size, FontFamily::Proportional),
                        text_color
                    );
                }
            }
            AppView::SettingsLightsLayout => {
                if let Some(icon) = &self.lights_layout_icon {
                    let icon_rect = Rect::from_center_size(
                        icon_pos,
                        Vec2::new(icon_size, icon_size)
                    );
                    ui.painter().image(
                        icon.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE
                    );
                } else {
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        view.icon(),
                        FontId::new(icon_size, FontFamily::Proportional),
                        text_color
                    );
                }
            }
            AppView::SettingsLegacyColor => {
                if let Some(icon) = &self.palette_icon {
                    let icon_rect = Rect::from_center_size(
                        icon_pos,
                        Vec2::new(icon_size, icon_size)
                    );
                    ui.painter().image(
                        icon.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE
                    );
                } else {
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        view.icon(),
                        FontId::new(icon_size, FontFamily::Proportional),
                        text_color
                    );
                }
            }
            AppView::SettingsApp => {
                if let Some(icon) = &self.app_settings_icon {
                    let icon_rect = Rect::from_center_size(
                        icon_pos,
                        Vec2::new(icon_size, icon_size)
                    );
                    ui.painter().image(
                        icon.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE
                    );
                } else {
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        view.icon(),
                        FontId::new(icon_size, FontFamily::Proportional),
                        text_color
                    );
                }
            }
            AppView::SettingsStartTime => {
                if let Some(icon) = &self.clock_icon {
                    let icon_rect = Rect::from_center_size(
                        icon_pos,
                        Vec2::new(icon_size, icon_size)
                    );
                    ui.painter().image(
                        icon.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE
                    );
                } else {
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        view.icon(),
                        FontId::new(icon_size, FontFamily::Proportional),
                        text_color
                    );
                }
            }
            AppView::SettingsProcedures => {
                if let Some(icon) = &self.checklist_icon {
                    let icon_rect = Rect::from_center_size(
                        icon_pos,
                        Vec2::new(icon_size, icon_size)
                    );
                    ui.painter().image(
                        icon.id(),
                        icon_rect,
                        Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                        Color32::WHITE
                    );
                } else {
                    ui.painter().text(
                        icon_pos,
                        egui::Align2::LEFT_CENTER,
                        view.icon(),
                        FontId::new(icon_size, FontFamily::Proportional),
                        text_color
                    );
                }
            }
            _ => {
                ui.painter().text(
                    icon_pos,
                    egui::Align2::LEFT_CENTER,
                    view.icon(),
                    FontId::new(icon_size, FontFamily::Proportional),
                    text_color
                );
            }
        }
        
        // Label (smaller)
        let label_pos = Pos2::new(rect.min.x + 66.0, rect.center().y);
        ui.painter().text(
            label_pos,
            egui::Align2::LEFT_CENTER,
            view.label(),
            FontId::new(13.0, FontFamily::Proportional),
            text_color
        );
        
        let tooltip_response = response.on_hover_text(view.tooltip());
        
        if !is_disabled && tooltip_response.clicked() {
            self.selected_view = view;
            true
        } else {
            false
        }
    }
}
