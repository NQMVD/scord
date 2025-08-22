use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualConfig {
    // Corner radius and rounding
    pub corner_radius: f32,
    pub button_corner_radius: f32,
    pub panel_corner_radius: f32,
    pub tab_corner_radius: f32,
    
    // Spacing system
    pub base_spacing: f32,
    pub item_spacing: f32,
    pub button_padding_x: f32,
    pub button_padding_y: f32,
    pub tab_spacing: f32,
    pub panel_margin: f32,
    
    // Typography
    pub base_font_size: f32,
    pub heading_font_size: f32,
    pub button_font_size: f32,
    pub small_font_size: f32,
    
    // Stroke widths
    pub border_width: f32,
    pub active_border_width: f32,
    pub focus_border_width: f32,
    
    // Visual effects
    pub shadow_enabled: bool,
    pub shadow_blur: f32,
    pub shadow_spread: f32,
    pub window_rounding: f32,
    
    // Interaction feedback
    pub hover_expansion: f32,
    pub active_expansion: f32,
    pub button_hover_lift: bool,
    
    // Animation settings
    pub animation_duration: f32,
    pub smooth_scrolling: bool,
    
    // Color system - RGB values for exact color palette
    // Background colors
    pub bg_primary_r: u8, pub bg_primary_g: u8, pub bg_primary_b: u8,
    pub bg_surface_r: u8, pub bg_surface_g: u8, pub bg_surface_b: u8,
    pub bg_elevated_r: u8, pub bg_elevated_g: u8, pub bg_elevated_b: u8,
    pub bg_extreme_r: u8, pub bg_extreme_g: u8, pub bg_extreme_b: u8,
    
    // Border colors
    pub border_default_r: u8, pub border_default_g: u8, pub border_default_b: u8,
    pub border_active_r: u8, pub border_active_g: u8, pub border_active_b: u8,
    pub border_hover_r: u8, pub border_hover_g: u8, pub border_hover_b: u8,
    
    // Text colors
    pub text_primary_r: u8, pub text_primary_g: u8, pub text_primary_b: u8,
    pub text_secondary_r: u8, pub text_secondary_g: u8, pub text_secondary_b: u8,
    pub text_muted_r: u8, pub text_muted_g: u8, pub text_muted_b: u8,
    
    // Interactive element colors
    pub widget_bg_r: u8, pub widget_bg_g: u8, pub widget_bg_b: u8,
    pub widget_bg_hover_r: u8, pub widget_bg_hover_g: u8, pub widget_bg_hover_b: u8,
    pub widget_bg_active_r: u8, pub widget_bg_active_g: u8, pub widget_bg_active_b: u8,
    
    // Selection color
    pub selection_bg_r: u8, pub selection_bg_g: u8, pub selection_bg_b: u8,
    
    // Accent colors (purple from design)
    pub accent_r: u8,
    pub accent_g: u8,
    pub accent_b: u8,
    
    // Status colors (these can remain RGB for colored warnings/errors)
    pub error_r: u8,
    pub error_g: u8,
    pub error_b: u8,
    
    pub warning_r: u8,
    pub warning_g: u8,
    pub warning_b: u8,
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            // Corner radius - Dashboard design (18px for cards, 8px for buttons)
            corner_radius: 18.0,
            button_corner_radius: 8.0,
            panel_corner_radius: 18.0,
            tab_corner_radius: 6.0,
            
            // Spacing system - Dashboard design (button padding: 14px x 8px)
            base_spacing: 8.0,
            item_spacing: 8.0,
            button_padding_x: 14.0,
            button_padding_y: 8.0,
            tab_spacing: 16.0,
            panel_margin: 12.0,
            
            // Typography - Dashboard design (Inter font: body 14px, heading 28px, button 14px)
            base_font_size: 14.0,
            heading_font_size: 28.0,
            button_font_size: 14.0,
            small_font_size: 13.0,
            
            // Stroke widths
            border_width: 1.0,
            active_border_width: 2.0,
            focus_border_width: 1.5,
            
            // Visual effects - Dashboard design
            shadow_enabled: true,
            shadow_blur: 12.0,
            shadow_spread: 0.0,
            window_rounding: 12.0,
            
            // Interaction feedback
            hover_expansion: 2.0,
            active_expansion: 2.0,
            button_hover_lift: true,
            
            // Animation settings
            animation_duration: 0.15,
            smooth_scrolling: true,
            
            // Exact color palette - night: #151517, eerie-black: #1b1b1d, raisin-black: #222224, raisin-black-2: #28282a, onyx: #3f3f41, onyx-2: #434345, taupe-gray: #959597, white: #fcfcfc
            // Background colors
            bg_primary_r: 0x15, bg_primary_g: 0x15, bg_primary_b: 0x17,       // #151517 - night
            bg_surface_r: 0x1b, bg_surface_g: 0x1b, bg_surface_b: 0x1d,       // #1b1b1d - eerie-black
            bg_elevated_r: 0x22, bg_elevated_g: 0x22, bg_elevated_b: 0x24,     // #222224 - raisin-black
            bg_extreme_r: 0x28, bg_extreme_g: 0x28, bg_extreme_b: 0x2a,        // #28282a - raisin-black-2
            
            // Border colors
            border_default_r: 0x3f, border_default_g: 0x3f, border_default_b: 0x41,  // #3f3f41 - onyx
            border_active_r: 0x95, border_active_g: 0x95, border_active_b: 0x97,     // #959597 - taupe-gray
            border_hover_r: 0x43, border_hover_g: 0x43, border_hover_b: 0x45,        // #434345 - onyx-2
            
            // Text colors
            text_primary_r: 0xfc, text_primary_g: 0xfc, text_primary_b: 0xfc,        // #fcfcfc - white
            text_secondary_r: 0x95, text_secondary_g: 0x95, text_secondary_b: 0x97,  // #959597 - taupe-gray
            text_muted_r: 0x70, text_muted_g: 0x70, text_muted_b: 0x72,              // Muted between taupe-gray and onyx-2
            
            // Interactive element colors
            widget_bg_r: 0x1b, widget_bg_g: 0x1b, widget_bg_b: 0x1d,                // #1b1b1d - eerie-black
            widget_bg_hover_r: 0x22, widget_bg_hover_g: 0x22, widget_bg_hover_b: 0x24,  // #222224 - raisin-black
            widget_bg_active_r: 0x28, widget_bg_active_g: 0x28, widget_bg_active_b: 0x2a, // #28282a - raisin-black-2
            
            // Selection color
            selection_bg_r: 0x22, selection_bg_g: 0x22, selection_bg_b: 0x24,        // #222224 - raisin-black
            
            // Accent colors - purple from design image
            accent_r: 132, accent_g: 126, accent_b: 255,
            
            // Status colors
            error_r: 255, error_g: 100, error_b: 100,
            warning_r: 255, warning_g: 165, warning_b: 0,
        }
    }
}

impl VisualConfig {
    pub fn load() -> Self {
        match Self::load_from_file() {
            Ok(config) => config,
            Err(_) => Self::default(),
        }
    }
    
    fn load_from_file() -> Result<Self> {
        let path = Self::config_path();
        let content = fs::read_to_string(&path)?;
        let config: VisualConfig = serde_json::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }
    
    fn config_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".scord");
        path.push("visual_config.json");
        path
    }
    
    // Convenience methods for common UI patterns
    pub fn get_rounding(&self) -> egui::Rounding {
        egui::Rounding::same(self.corner_radius)
    }
    
    pub fn get_button_rounding(&self) -> egui::Rounding {
        egui::Rounding::same(self.button_corner_radius)
    }
    
    pub fn get_panel_rounding(&self) -> egui::Rounding {
        egui::Rounding::same(self.panel_corner_radius)
    }
    
    pub fn get_tab_rounding(&self) -> egui::Rounding {
        egui::Rounding::same(self.tab_corner_radius)
    }
    
    pub fn get_item_spacing(&self) -> egui::Vec2 {
        egui::vec2(self.item_spacing, self.item_spacing)
    }
    
    pub fn get_button_padding(&self) -> egui::Vec2 {
        egui::vec2(self.button_padding_x, self.button_padding_y)
    }
    
    pub fn get_border_stroke(&self) -> egui::Stroke {
        egui::Stroke::new(self.border_width, self.get_border_default())
    }
    
    pub fn get_active_border_stroke(&self) -> egui::Stroke {
        egui::Stroke::new(self.active_border_width, self.get_border_active())
    }
    
    pub fn get_shadow(&self) -> egui::Shadow {
        if self.shadow_enabled {
            egui::Shadow {
                offset: egui::vec2(4.0, 4.0),
                blur: self.shadow_blur,
                spread: self.shadow_spread,
                color: egui::Color32::from_black_alpha(100),
            }
        } else {
            egui::Shadow::NONE
        }
    }
    
    // Color helper methods - return RGB colors
    pub fn get_bg_primary(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.bg_primary_r, self.bg_primary_g, self.bg_primary_b)
    }
    
    pub fn get_bg_surface(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.bg_surface_r, self.bg_surface_g, self.bg_surface_b)
    }
    
    pub fn get_bg_elevated(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.bg_elevated_r, self.bg_elevated_g, self.bg_elevated_b)
    }
    
    pub fn get_bg_extreme(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.bg_extreme_r, self.bg_extreme_g, self.bg_extreme_b)
    }
    
    pub fn get_border_default(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.border_default_r, self.border_default_g, self.border_default_b)
    }
    
    pub fn get_border_active(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.border_active_r, self.border_active_g, self.border_active_b)
    }
    
    pub fn get_border_hover(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.border_hover_r, self.border_hover_g, self.border_hover_b)
    }
    
    pub fn get_text_primary(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.text_primary_r, self.text_primary_g, self.text_primary_b)
    }
    
    pub fn get_text_secondary(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.text_secondary_r, self.text_secondary_g, self.text_secondary_b)
    }
    
    pub fn get_text_muted(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.text_muted_r, self.text_muted_g, self.text_muted_b)
    }
    
    pub fn get_widget_bg(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.widget_bg_r, self.widget_bg_g, self.widget_bg_b)
    }
    
    pub fn get_widget_bg_hover(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.widget_bg_hover_r, self.widget_bg_hover_g, self.widget_bg_hover_b)
    }
    
    pub fn get_widget_bg_active(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.widget_bg_active_r, self.widget_bg_active_g, self.widget_bg_active_b)
    }
    
    pub fn get_selection_bg(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.selection_bg_r, self.selection_bg_g, self.selection_bg_b)
    }
    
    pub fn get_error_color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.error_r, self.error_g, self.error_b)
    }
    
    pub fn get_warning_color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.warning_r, self.warning_g, self.warning_b)
    }
    
    pub fn get_accent_color(&self) -> egui::Color32 {
        egui::Color32::from_rgb(self.accent_r, self.accent_g, self.accent_b)
    }
    
    // Preset configurations
    pub fn minimal() -> Self {
        let mut config = Self::default();
        config.corner_radius = 4.0;
        config.button_corner_radius = 4.0;
        config.panel_corner_radius = 4.0;
        config.tab_corner_radius = 4.0;
        config.shadow_enabled = false;
        config.hover_expansion = 0.0;
        config.active_expansion = 0.0;
        config.button_hover_lift = false;
        config.accent_r = 132;
        config.accent_g = 126;
        config.accent_b = 255;
        config
    }
    
    pub fn rounded() -> Self {
        let mut config = Self::default();
        config.corner_radius = 12.0;
        config.button_corner_radius = 12.0;
        config.panel_corner_radius = 12.0;
        config.tab_corner_radius = 12.0;
        config.window_rounding = 12.0;
        config.shadow_blur = 12.0;
        config.accent_r = 132;
        config.accent_g = 126;
        config.accent_b = 255;
        config
    }
    
    pub fn compact() -> Self {
        let mut config = Self::default();
        config.base_spacing = 4.0;
        config.item_spacing = 4.0;
        config.button_padding_x = 8.0;
        config.button_padding_y = 4.0;
        config.tab_spacing = 8.0;
        config.panel_margin = 4.0;
        config.base_font_size = 14.0;
        config.heading_font_size = 18.0;
        config.button_font_size = 13.0;
        config.small_font_size = 11.0;
        config.accent_r = 132;
        config.accent_g = 126;
        config.accent_b = 255;
        config
    }
    
    pub fn spacious() -> Self {
        let mut config = Self::default();
        config.base_spacing = 12.0;
        config.item_spacing = 12.0;
        config.button_padding_x = 16.0;
        config.button_padding_y = 12.0;
        config.tab_spacing = 24.0;
        config.panel_margin = 12.0;
        config.base_font_size = 18.0;
        config.heading_font_size = 26.0;
        config.button_font_size = 17.0;
        config.small_font_size = 15.0;
        config.accent_r = 132;
        config.accent_g = 126;
        config.accent_b = 255;
        config
    }
    
    pub fn dashboard() -> Self {
        let mut config = Self::default();
        // Dashboard theme is already the default, but we'll make it explicit
        config.corner_radius = 18.0;
        config.button_corner_radius = 8.0;
        config.panel_corner_radius = 18.0;
        config.tab_corner_radius = 6.0;
        config.window_rounding = 12.0;
        config.shadow_blur = 12.0;
        
        // Exact color palette - night: #151517, eerie-black: #1b1b1d, raisin-black: #222224, raisin-black-2: #28282a, onyx: #3f3f41, onyx-2: #434345, taupe-gray: #959597, white: #fcfcfc
        // Background colors
        config.bg_primary_r = 0x15; config.bg_primary_g = 0x15; config.bg_primary_b = 0x17;       // #151517 - night
        config.bg_surface_r = 0x1b; config.bg_surface_g = 0x1b; config.bg_surface_b = 0x1d;       // #1b1b1d - eerie-black
        config.bg_elevated_r = 0x22; config.bg_elevated_g = 0x22; config.bg_elevated_b = 0x24;     // #222224 - raisin-black
        config.bg_extreme_r = 0x28; config.bg_extreme_g = 0x28; config.bg_extreme_b = 0x2a;        // #28282a - raisin-black-2
        
        // Border colors
        config.border_default_r = 0x3f; config.border_default_g = 0x3f; config.border_default_b = 0x41;  // #3f3f41 - onyx
        config.border_active_r = 0x95; config.border_active_g = 0x95; config.border_active_b = 0x97;     // #959597 - taupe-gray
        config.border_hover_r = 0x43; config.border_hover_g = 0x43; config.border_hover_b = 0x45;        // #434345 - onyx-2
        
        // Text colors
        config.text_primary_r = 0xfc; config.text_primary_g = 0xfc; config.text_primary_b = 0xfc;        // #fcfcfc - white
        config.text_secondary_r = 0x95; config.text_secondary_g = 0x95; config.text_secondary_b = 0x97;  // #959597 - taupe-gray
        config.text_muted_r = 0x70; config.text_muted_g = 0x70; config.text_muted_b = 0x72;              // Muted between taupe-gray and onyx-2
        
        // Interactive element colors
        config.widget_bg_r = 0x1b; config.widget_bg_g = 0x1b; config.widget_bg_b = 0x1d;                // #1b1b1d - eerie-black
        config.widget_bg_hover_r = 0x22; config.widget_bg_hover_g = 0x22; config.widget_bg_hover_b = 0x24;  // #222224 - raisin-black
        config.widget_bg_active_r = 0x28; config.widget_bg_active_g = 0x28; config.widget_bg_active_b = 0x2a; // #28282a - raisin-black-2
        
        // Selection color
        config.selection_bg_r = 0x22; config.selection_bg_g = 0x22; config.selection_bg_b = 0x24;        // #222224 - raisin-black
        
        // Dashboard typography (Inter font)
        config.base_font_size = 14.0;
        config.heading_font_size = 28.0;
        config.button_font_size = 14.0;
        config.button_padding_x = 14.0;
        config.button_padding_y = 8.0;
        
        config
    }
}

#[derive(Debug, Clone)]
pub struct VisualPreset {
    pub name: String,
    pub description: String,
    pub config: VisualConfig,
}

impl VisualPreset {
    pub fn get_default_presets() -> Vec<VisualPreset> {
        vec![
            VisualPreset {
                name: "Dashboard".to_string(),
                description: "Dark modern dashboard theme with Inter font".to_string(),
                config: VisualConfig::dashboard(),
            },
            VisualPreset {
                name: "Default".to_string(),
                description: "Balanced design with modern styling".to_string(),
                config: VisualConfig::default(),
            },
            VisualPreset {
                name: "Minimal".to_string(),
                description: "Clean and minimal with subtle effects".to_string(),
                config: VisualConfig::minimal(),
            },
            VisualPreset {
                name: "Rounded".to_string(),
                description: "Soft and friendly with large corner radius".to_string(),
                config: VisualConfig::rounded(),
            },
            VisualPreset {
                name: "Compact".to_string(),
                description: "Dense layout for maximum screen usage".to_string(),
                config: VisualConfig::compact(),
            },
            VisualPreset {
                name: "Spacious".to_string(),
                description: "Comfortable spacing for larger screens".to_string(),
                config: VisualConfig::spacious(),
            },
        ]
    }
}