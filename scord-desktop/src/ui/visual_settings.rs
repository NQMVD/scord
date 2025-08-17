use crate::models::{VisualConfig, VisualPreset};
use egui::{Ui, Window, ScrollArea, Grid, Slider, Checkbox, ComboBox};

pub struct VisualSettingsPanel {
    pub is_open: bool,
    pub config: VisualConfig,
    selected_preset: Option<usize>,
    presets: Vec<VisualPreset>,
}

impl Default for VisualSettingsPanel {
    fn default() -> Self {
        Self {
            is_open: false,
            config: VisualConfig::load(),
            selected_preset: Some(0), // Default preset
            presets: VisualPreset::get_default_presets(),
        }
    }
}

impl VisualSettingsPanel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn toggle(&mut self) {
        self.is_open = !self.is_open;
    }
    
    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let mut config_changed = false;
        
        if self.is_open {
            Window::new("🎨 Visual Settings")
                .default_size([400.0, 600.0])
                .resizable(true)
                .collapsible(false)
                .open(&mut self.is_open)
                .show(ctx, |ui| {
                    ScrollArea::vertical().show(ui, |ui| {
                        // Presets section
                        ui.heading("Presets");
                        ui.separator();
                        
                        ui.horizontal(|ui| {
                            ComboBox::from_label("Choose Preset")
                                .selected_text(
                                    self.selected_preset
                                        .and_then(|i| self.presets.get(i))
                                        .map(|p| p.name.as_str())
                                        .unwrap_or("Custom")
                                )
                                .show_ui(ui, |ui| {
                                    for (i, preset) in self.presets.iter().enumerate() {
                                        if ui.selectable_value(&mut self.selected_preset, Some(i), &preset.name).clicked() {
                                            self.config = preset.config.clone();
                                            config_changed = true;
                                        }
                                    }
                                });
                                
                            if ui.button("Reset to Default").clicked() {
                                self.config = VisualConfig::default();
                                self.selected_preset = Some(0);
                                config_changed = true;
                            }
                        });
                        
                        ui.add_space(16.0);
                        
                        // Corner Radius section
                        ui.heading("Corner Radius");
                        ui.separator();
                        
                        Grid::new("corner_radius_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("General Radius:");
                                if ui.add(Slider::new(&mut self.config.corner_radius, 0.0..=20.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None; // Mark as custom
                                }
                                ui.end_row();
                                
                                ui.label("Button Radius:");
                                if ui.add(Slider::new(&mut self.config.button_corner_radius, 0.0..=20.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Panel Radius:");
                                if ui.add(Slider::new(&mut self.config.panel_corner_radius, 0.0..=20.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Tab Radius:");
                                if ui.add(Slider::new(&mut self.config.tab_corner_radius, 0.0..=20.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Spacing section
                        ui.heading("Spacing & Layout");
                        ui.separator();
                        
                        Grid::new("spacing_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Base Spacing:");
                                if ui.add(Slider::new(&mut self.config.base_spacing, 2.0..=20.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Item Spacing:");
                                if ui.add(Slider::new(&mut self.config.item_spacing, 2.0..=20.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Tab Spacing:");
                                if ui.add(Slider::new(&mut self.config.tab_spacing, 2.0..=40.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Button Padding X:");
                                if ui.add(Slider::new(&mut self.config.button_padding_x, 4.0..=24.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Button Padding Y:");
                                if ui.add(Slider::new(&mut self.config.button_padding_y, 2.0..=16.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Typography section
                        ui.heading("Typography");
                        ui.separator();
                        
                        Grid::new("typography_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Base Font Size:");
                                if ui.add(Slider::new(&mut self.config.base_font_size, 10.0..=24.0)
                                    .suffix("pt")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Heading Size:");
                                if ui.add(Slider::new(&mut self.config.heading_font_size, 14.0..=32.0)
                                    .suffix("pt")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Button Font Size:");
                                if ui.add(Slider::new(&mut self.config.button_font_size, 10.0..=20.0)
                                    .suffix("pt")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Small Font Size:");
                                if ui.add(Slider::new(&mut self.config.small_font_size, 8.0..=16.0)
                                    .suffix("pt")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Visual Effects section
                        ui.heading("Visual Effects");
                        ui.separator();
                        
                        Grid::new("effects_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Enable Shadows:");
                                if ui.add(Checkbox::new(&mut self.config.shadow_enabled, "")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                if self.config.shadow_enabled {
                                    ui.label("Shadow Blur:");
                                    if ui.add(Slider::new(&mut self.config.shadow_blur, 0.0..=20.0)
                                        .suffix("px")).changed() {
                                        config_changed = true;
                                        self.selected_preset = None;
                                    }
                                    ui.end_row();
                                }
                                
                                ui.label("Hover Expansion:");
                                if ui.add(Slider::new(&mut self.config.hover_expansion, 0.0..=8.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Button Hover Lift:");
                                if ui.add(Checkbox::new(&mut self.config.button_hover_lift, "")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Border section
                        ui.heading("Borders");
                        ui.separator();
                        
                        Grid::new("border_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Border Width:");
                                if ui.add(Slider::new(&mut self.config.border_width, 0.5..=4.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Active Border:");
                                if ui.add(Slider::new(&mut self.config.active_border_width, 1.0..=6.0)
                                    .suffix("px")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Colors section
                        ui.heading("Colors");
                        ui.separator();
                        
                        // Background Colors
                        ui.strong("Background Colors");
                        Grid::new("bg_colors_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Primary Background:");
                                if ui.add(Slider::new(&mut self.config.bg_primary, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Surface Background:");
                                if ui.add(Slider::new(&mut self.config.bg_surface, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Elevated Background:");
                                if ui.add(Slider::new(&mut self.config.bg_elevated, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Extreme Background:");
                                if ui.add(Slider::new(&mut self.config.bg_extreme, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Border Colors
                        ui.strong("Border Colors");
                        Grid::new("border_colors_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Default Border:");
                                if ui.add(Slider::new(&mut self.config.border_default, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Active Border:");
                                if ui.add(Slider::new(&mut self.config.border_active, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Hover Border:");
                                if ui.add(Slider::new(&mut self.config.border_hover, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Text Colors
                        ui.strong("Text Colors");
                        Grid::new("text_colors_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Primary Text:");
                                if ui.add(Slider::new(&mut self.config.text_primary, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Secondary Text:");
                                if ui.add(Slider::new(&mut self.config.text_secondary, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Muted Text:");
                                if ui.add(Slider::new(&mut self.config.text_muted, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Interactive Elements
                        ui.strong("Interactive Elements");
                        Grid::new("widget_colors_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Widget Background:");
                                if ui.add(Slider::new(&mut self.config.widget_bg, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Widget Hover:");
                                if ui.add(Slider::new(&mut self.config.widget_bg_hover, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Widget Active:");
                                if ui.add(Slider::new(&mut self.config.widget_bg_active, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Selection:");
                                if ui.add(Slider::new(&mut self.config.selection_bg, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(16.0);
                        
                        // Status Colors (keep RGB for colored warnings/errors)
                        ui.strong("Status Colors");
                        Grid::new("status_colors_grid")
                            .num_columns(2)
                            .spacing([8.0, 8.0])
                            .show(ui, |ui| {
                                ui.label("Error Red:");
                                if ui.add(Slider::new(&mut self.config.error_r, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Error Green:");
                                if ui.add(Slider::new(&mut self.config.error_g, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Error Blue:");
                                if ui.add(Slider::new(&mut self.config.error_b, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Warning Red:");
                                if ui.add(Slider::new(&mut self.config.warning_r, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Warning Green:");
                                if ui.add(Slider::new(&mut self.config.warning_g, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                                
                                ui.label("Warning Blue:");
                                if ui.add(Slider::new(&mut self.config.warning_b, 0..=255).suffix("")).changed() {
                                    config_changed = true;
                                    self.selected_preset = None;
                                }
                                ui.end_row();
                            });
                        
                        ui.add_space(24.0);
                        
                        // Save/Load section
                        ui.separator();
                        ui.horizontal(|ui| {
                            if ui.button("💾 Save Settings").clicked() {
                                if let Err(e) = self.config.save() {
                                    eprintln!("Failed to save visual config: {}", e);
                                }
                            }
                            
                            if ui.button("📁 Load Settings").clicked() {
                                self.config = VisualConfig::load();
                                config_changed = true;
                            }
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.small("Changes apply immediately");
                            });
                        });
                        
                        // Demo section
                        ui.add_space(16.0);
                        ui.heading("Preview");
                        ui.separator();
                        
                        ui.horizontal(|ui| {
                            ui.button("Sample Button");
                            ui.small_button("Small");
                        });
                        
                        ui.group(|ui| {
                            ui.label("Sample panel with current styling");
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut false, "Checkbox");
                                ui.add(egui::Slider::new(&mut 50.0, 0.0..=100.0).text("Slider"));
                            });
                        });
                    });
                });
        }
        
        config_changed
    }
    
    pub fn get_config(&self) -> &VisualConfig {
        &self.config
    }
}