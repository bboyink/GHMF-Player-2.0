use egui::{Color32, Style, Visuals, FontId, FontFamily, Stroke, Rounding};

/// Custom color palette for the application - inspired by modern dark themes
pub struct AppColors;

impl AppColors {
    // Cyan/Blue gradient theme (matching web design)
    pub const CYAN: Color32 = Color32::from_rgb(0, 198, 255);         // #00c6ff
    pub const CYAN_DARK: Color32 = Color32::from_rgb(0, 114, 255);    // #0072ff
    pub const CYAN_LIGHT: Color32 = Color32::from_rgb(96, 165, 250);  // #60a5fa
    
    pub const PRIMARY: Color32 = Color32::from_rgb(0, 198, 255);      // Cyan as primary
    pub const PRIMARY_DARK: Color32 = Color32::from_rgb(0, 114, 255);
    pub const PRIMARY_LIGHT: Color32 = Color32::from_rgb(96, 165, 250);
    
    pub const ACCENT: Color32 = Color32::from_rgb(255, 193, 7);       // Gold/Yellow
    pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(255, 183, 77);
    
    // Dark gradient background (matching web: #1e293b to #0f172a)
    pub const BACKGROUND: Color32 = Color32::from_rgb(15, 23, 42);    // #0f172a
    pub const BACKGROUND_LIGHT: Color32 = Color32::from_rgb(30, 41, 59); // #1e293b
    pub const SURFACE: Color32 = Color32::from_rgb(30, 41, 59);       // #1e293b
    pub const SURFACE_LIGHT: Color32 = Color32::from_rgb(51, 65, 85); // #334155
    pub const PANEL: Color32 = Color32::from_rgb(20, 28, 46);         // Slightly lighter than background
    
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(226, 232, 240);   // #e2e8f0
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(148, 163, 184); // #94a3b8
    pub const TEXT_DISABLED: Color32 = Color32::from_rgb(100, 116, 139);  // #64748b
    
    pub const ERROR: Color32 = Color32::from_rgb(239, 68, 68);        // #ef4444
    pub const WARNING: Color32 = Color32::from_rgb(255, 193, 7);      // #ffc107
    pub const SUCCESS: Color32 = Color32::from_rgb(0, 255, 136);      // #00ff88
    pub const INFO: Color32 = Color32::from_rgb(0, 198, 255);         // #00c6ff
    
    pub const DMX_ACTIVE: Color32 = Color32::from_rgb(0, 255, 136);
    pub const DMX_INACTIVE: Color32 = Color32::from_rgb(239, 68, 68);
}

// Glass effect colors (non-const due to alpha channel function)
pub fn glass_fill() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 255, 255, 13)
}

pub fn glass_stroke() -> Color32 {
    Color32::from_rgba_unmultiplied(255, 255, 255, 26)
}

pub fn configure_theme(ctx: &egui::Context) {
    let mut style = Style::default();
    
    // Start with dark visuals
    style.visuals = Visuals::dark();
    
    // Core background colors - FORCE dark backgrounds
    style.visuals.panel_fill = AppColors::PANEL;
    style.visuals.window_fill = AppColors::BACKGROUND;
    style.visuals.faint_bg_color = AppColors::SURFACE;
    style.visuals.extreme_bg_color = AppColors::BACKGROUND;
    style.visuals.code_bg_color = AppColors::SURFACE_LIGHT;
    
    // Text colors - HIGH CONTRAST for readability
    style.visuals.override_text_color = Some(AppColors::TEXT_PRIMARY);
    style.visuals.warn_fg_color = AppColors::WARNING;
    style.visuals.error_fg_color = AppColors::ERROR;
    style.visuals.hyperlink_color = AppColors::CYAN_LIGHT;
    
    // Window styling
    style.visuals.window_rounding = Rounding::same(12.0);
    style.visuals.window_fill = AppColors::PANEL;
    style.visuals.window_stroke = Stroke::new(1.0, AppColors::SURFACE_LIGHT);
    style.visuals.window_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 8.0),
        blur: 20.0,
        spread: 0.0,
        color: Color32::from_black_alpha(100),
    };
    
    // Popup styling
    style.visuals.popup_shadow = egui::epaint::Shadow {
        offset: egui::vec2(0.0, 4.0),
        blur: 16.0,
        spread: 0.0,
        color: Color32::from_black_alpha(120),
    };
    
    // Widget colors - Modern button styling
    style.visuals.widgets.noninteractive.bg_fill = AppColors::SURFACE;
    style.visuals.widgets.noninteractive.weak_bg_fill = AppColors::SURFACE;
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, AppColors::SURFACE_LIGHT);
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, AppColors::TEXT_PRIMARY);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(8.0);
    
    // Inactive widgets (buttons, etc)
    style.visuals.widgets.inactive.bg_fill = AppColors::SURFACE;
    style.visuals.widgets.inactive.weak_bg_fill = AppColors::SURFACE;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.5, AppColors::CYAN_DARK);
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.5, AppColors::TEXT_PRIMARY);
    style.visuals.widgets.inactive.rounding = Rounding::same(10.0);
    
    // Hovered widgets - Cyan glow
    style.visuals.widgets.hovered.bg_fill = AppColors::CYAN_DARK;
    style.visuals.widgets.hovered.weak_bg_fill = AppColors::CYAN_DARK;
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(2.0, AppColors::CYAN);
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(2.0, Color32::WHITE);
    style.visuals.widgets.hovered.rounding = Rounding::same(10.0);
    style.visuals.widgets.hovered.expansion = 2.0;
    
    // Active/clicked widgets - Bright cyan
    style.visuals.widgets.active.bg_fill = AppColors::CYAN;
    style.visuals.widgets.active.weak_bg_fill = AppColors::CYAN;
    style.visuals.widgets.active.bg_stroke = Stroke::new(2.0, AppColors::CYAN_LIGHT);
    style.visuals.widgets.active.fg_stroke = Stroke::new(2.0, Color32::WHITE);
    style.visuals.widgets.active.rounding = Rounding::same(10.0);
    style.visuals.widgets.active.expansion = 1.0;
    
    // Open menu/combo boxes
    style.visuals.widgets.open.bg_fill = AppColors::SURFACE_LIGHT;
    style.visuals.widgets.open.weak_bg_fill = AppColors::SURFACE_LIGHT;
    style.visuals.widgets.open.bg_stroke = Stroke::new(2.0, AppColors::CYAN);
    style.visuals.widgets.open.fg_stroke = Stroke::new(2.0, AppColors::TEXT_PRIMARY);
    style.visuals.widgets.open.rounding = Rounding::same(10.0);
    
    // Selection colors
    style.visuals.selection.bg_fill = AppColors::CYAN_DARK;
    style.visuals.selection.stroke = Stroke::new(1.5, AppColors::CYAN);
    
    // Separator/stroke colors
    style.visuals.window_stroke = Stroke::new(1.0, AppColors::SURFACE_LIGHT);
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, AppColors::SURFACE_LIGHT);
    
    // Spacing and sizing - more generous
    style.spacing.item_spacing = egui::vec2(10.0, 10.0);
    style.spacing.button_padding = egui::vec2(16.0, 10.0);
    style.spacing.window_margin = egui::Margin::same(12.0);
    style.spacing.menu_margin = egui::Margin::same(10.0);
    style.spacing.indent = 20.0;
    style.spacing.slider_width = 200.0;
    style.spacing.combo_width = 150.0;
    
    // Interaction settings
    style.interaction.resize_grab_radius_side = 8.0;
    style.interaction.resize_grab_radius_corner = 12.0;
    
    ctx.set_style(style);
    
    // Also set the central panel frame to dark
    ctx.set_visuals(Visuals::dark());
}

pub fn heading_font() -> FontId {
    FontId::new(18.0, FontFamily::Proportional)
}

pub fn body_font() -> FontId {
    FontId::new(14.0, FontFamily::Proportional)
}

pub fn button_font() -> FontId {
    FontId::new(14.0, FontFamily::Proportional)
}
