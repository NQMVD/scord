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
    
    // Color system - Grayscale values (0-255)
    pub bg_primary: u8,
    pub bg_surface: u8,
    pub bg_elevated: u8,
    pub bg_extreme: u8,
    
    // Border grayscale values
    pub border_default: u8,
    pub border_active: u8,
    pub border_hover: u8,
    
    // Text grayscale values
    pub text_primary: u8,
    pub text_secondary: u8,
    pub text_muted: u8,
    
    // Interactive element grayscale values
    pub widget_bg: u8,
    pub widget_bg_hover: u8,
    pub widget_bg_active: u8,
    
    // Selection grayscale value
    pub selection_bg: u8,
    
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
            // Corner radius - unified system, more rounded like design
            corner_radius: 12.0,
            button_corner_radius: 12.0,
            panel_corner_radius: 12.0,
            tab_corner_radius: 12.0,
            
            // Spacing system
            base_spacing: 8.0,
            item_spacing: 8.0,
            button_padding_x: 12.0,
            button_padding_y: 8.0,
            tab_spacing: 16.0,
            panel_margin: 8.0,
            
            // Typography
            base_font_size: 16.0,
            heading_font_size: 22.0,
            button_font_size: 15.0,
            small_font_size: 13.0,
            
            // Stroke widths
            border_width: 1.0,
            active_border_width: 2.0,
            focus_border_width: 1.5,
            
            // Visual effects
            shadow_enabled: true,
            shadow_blur: 8.0,
            shadow_spread: 0.0,
            window_rounding: 8.0,
            
            // Interaction feedback
            hover_expansion: 2.0,
            active_expansion: 2.0,
            button_hover_lift: true,
            
            // Animation settings
            animation_duration: 0.15,
            smooth_scrolling: true,
            
            // Color system - Dark theme matching design image
            bg_primary: 13,    // Exact #0D0D0D background
            bg_surface: 18,    // Slightly lighter surface
            bg_elevated: 23,   // Elevated elements
            bg_extreme: 8,     // Deepest background
            
            // Border grayscale values
            border_default: 40,  // Subtle borders
            border_active: 132,  // Purple accent for active borders
            border_hover: 60,    // Hover borders
            
            // Text grayscale values
            text_primary: 245,   // Very light text
            text_secondary: 180, // Secondary text
            text_muted: 120,     // Muted text
            
            // Interactive element grayscale values
            widget_bg: 25,        // Widget background
            widget_bg_hover: 35,  // Widget hover
            widget_bg_active: 45, // Widget active
            
            // Selection grayscale value
            selection_bg: 40,     // Selection background
            
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
    
    // Color helper methods - convert grayscale to RGB
    pub fn get_bg_primary(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.bg_primary)
    }
    
    pub fn get_bg_surface(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.bg_surface)
    }
    
    pub fn get_bg_elevated(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.bg_elevated)
    }
    
    pub fn get_bg_extreme(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.bg_extreme)
    }
    
    pub fn get_border_default(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.border_default)
    }
    
    pub fn get_border_active(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.border_active)
    }
    
    pub fn get_border_hover(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.border_hover)
    }
    
    pub fn get_text_primary(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.text_primary)
    }
    
    pub fn get_text_secondary(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.text_secondary)
    }
    
    pub fn get_text_muted(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.text_muted)
    }
    
    pub fn get_widget_bg(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.widget_bg)
    }
    
    pub fn get_widget_bg_hover(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.widget_bg_hover)
    }
    
    pub fn get_widget_bg_active(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.widget_bg_active)
    }
    
    pub fn get_selection_bg(&self) -> egui::Color32 {
        egui::Color32::from_gray(self.selection_bg)
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