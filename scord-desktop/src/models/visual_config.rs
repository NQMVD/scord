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
}

impl Default for VisualConfig {
    fn default() -> Self {
        Self {
            // Corner radius - unified system
            corner_radius: 8.0,
            button_corner_radius: 8.0,
            panel_corner_radius: 8.0,
            tab_corner_radius: 8.0,
            
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
        egui::Stroke::new(self.border_width, egui::Color32::from_gray(100))
    }
    
    pub fn get_active_border_stroke(&self) -> egui::Stroke {
        egui::Stroke::new(self.active_border_width, egui::Color32::from_gray(150))
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
    
    // Preset configurations
    pub fn minimal() -> Self {
        Self {
            corner_radius: 4.0,
            button_corner_radius: 4.0,
            panel_corner_radius: 4.0,
            tab_corner_radius: 4.0,
            shadow_enabled: false,
            hover_expansion: 0.0,
            active_expansion: 0.0,
            button_hover_lift: false,
            ..Default::default()
        }
    }
    
    pub fn rounded() -> Self {
        Self {
            corner_radius: 12.0,
            button_corner_radius: 12.0,
            panel_corner_radius: 12.0,
            tab_corner_radius: 12.0,
            window_rounding: 12.0,
            shadow_blur: 12.0,
            ..Default::default()
        }
    }
    
    pub fn compact() -> Self {
        Self {
            base_spacing: 4.0,
            item_spacing: 4.0,
            button_padding_x: 8.0,
            button_padding_y: 4.0,
            tab_spacing: 8.0,
            panel_margin: 4.0,
            base_font_size: 14.0,
            heading_font_size: 18.0,
            button_font_size: 13.0,
            small_font_size: 11.0,
            ..Default::default()
        }
    }
    
    pub fn spacious() -> Self {
        Self {
            base_spacing: 12.0,
            item_spacing: 12.0,
            button_padding_x: 16.0,
            button_padding_y: 12.0,
            tab_spacing: 24.0,
            panel_margin: 12.0,
            base_font_size: 18.0,
            heading_font_size: 26.0,
            button_font_size: 17.0,
            small_font_size: 15.0,
            ..Default::default()
        }
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