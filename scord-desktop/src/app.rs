use crate::models::{TabManager, VisualConfig};
use crate::ui::{setup_custom_style, setup_custom_style_with_config, SpreadsheetView, ResultsPanel, ControlsPanel, TabBar, TabAction, VisualSettingsPanel};
use egui::{Context, CentralPanel, SidePanel, TopBottomPanel};

pub struct ScordApp {
    tab_manager: TabManager,
    visual_settings: VisualSettingsPanel,
    controls_panel: ControlsPanel,
}


impl Default for ScordApp {
    fn default() -> Self {
        Self {
            tab_manager: TabManager::new(),
            visual_settings: VisualSettingsPanel::new(),
            controls_panel: ControlsPanel::new(),
        }
    }
}

impl ScordApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let config = VisualConfig::load();
        setup_custom_style_with_config(&cc.egui_ctx, &config);
        let mut app = Self::default();
        app.visual_settings.config = config;
        app
    }

    // Delegate methods to active tab
    pub fn add_contestant(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.content.add_contestant();
        }
    }

    pub fn add_property(&mut self) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.content.add_property();
        }
    }

    pub fn delete_contestant(&mut self, contestant_id: uuid::Uuid) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.content.delete_contestant(contestant_id);
        }
    }

    pub fn delete_property(&mut self, property_id: uuid::Uuid) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.content.delete_property(property_id);
        }
    }

    pub fn update_contestant_name(&mut self, contestant_id: uuid::Uuid, name: String) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.content.update_contestant_name(contestant_id, name);
        }
    }

    pub fn update_contestant_value(&mut self, contestant_id: uuid::Uuid, property_id: uuid::Uuid, value: f64) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.content.update_contestant_value(contestant_id, property_id, value);
        }
    }

    pub fn update_property(&mut self, property_id: uuid::Uuid, name: String, weight: f64, higher_is_better: bool) {
        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
            tab.content.update_property(property_id, name, weight, higher_is_better);
        }
    }

    // Access methods for UI components
    pub fn get_active_tab_data(&self) -> Option<(&Vec<crate::models::Contestant>, &Vec<crate::models::Property>, &Vec<crate::models::ScoreResult>)> {
        self.tab_manager.get_active_tab().map(|tab| (
            &tab.content.contestants,
            &tab.content.properties,
            &tab.content.score_results,
        ))
    }

    pub fn get_active_tab_ui_state(&self) -> Option<(&String, &String, f64, bool)> {
        self.tab_manager.get_active_tab().map(|tab| (
            &tab.content.new_contestant_name,
            &tab.content.new_property_name,
            tab.content.new_property_weight,
            tab.content.new_property_higher_is_better,
        ))
    }

    pub fn get_active_tab_ui_state_mut(&mut self) -> Option<(&mut String, &mut String, &mut f64, &mut bool)> {
        self.tab_manager.get_active_tab_mut().map(|tab| (
            &mut tab.content.new_contestant_name,
            &mut tab.content.new_property_name,
            &mut tab.content.new_property_weight,
            &mut tab.content.new_property_higher_is_better,
        ))
    }
}

impl eframe::App for ScordApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Handle keyboard shortcuts
        ctx.input(|i| {
            // Ctrl+T: New tab
            if i.key_pressed(egui::Key::T) && i.modifiers.ctrl {
                self.tab_manager.add_tab();
            }
            // Ctrl+W: Close current tab (if more than one tab exists)
            if i.key_pressed(egui::Key::W) && i.modifiers.ctrl {
                if self.tab_manager.can_close_tab(self.tab_manager.active_tab_index) {
                    self.tab_manager.close_tab(self.tab_manager.active_tab_index);
                }
            }
            // Ctrl+Tab: Next tab
            if i.key_pressed(egui::Key::Tab) && i.modifiers.ctrl && !i.modifiers.shift {
                let next_index = (self.tab_manager.active_tab_index + 1) % self.tab_manager.tab_count();
                self.tab_manager.set_active_tab(next_index);
            }
            // Ctrl+Shift+Tab: Previous tab
            if i.key_pressed(egui::Key::Tab) && i.modifiers.ctrl && i.modifiers.shift {
                let prev_index = if self.tab_manager.active_tab_index == 0 {
                    self.tab_manager.tab_count() - 1
                } else {
                    self.tab_manager.active_tab_index - 1
                };
                self.tab_manager.set_active_tab(prev_index);
            }
            // Ctrl+1-9: Switch to tab by number
            for i_key in 1..=9 {
                let key = match i_key {
                    1 => egui::Key::Num1,
                    2 => egui::Key::Num2,
                    3 => egui::Key::Num3,
                    4 => egui::Key::Num4,
                    5 => egui::Key::Num5,
                    6 => egui::Key::Num6,
                    7 => egui::Key::Num7,
                    8 => egui::Key::Num8,
                    9 => egui::Key::Num9,
                    _ => continue,
                };
                if i.key_pressed(key) && i.modifiers.ctrl {
                    let tab_index = (i_key - 1) as usize;
                    if tab_index < self.tab_manager.tab_count() {
                        self.tab_manager.set_active_tab(tab_index);
                    }
                }
            }
        });

        // Dashboard-style header
        TopBottomPanel::top("header").show(ctx, |ui| {
            let config = self.visual_settings.get_config();
            
            // Header background with dashboard styling
            let header_frame = egui::Frame::none()
                .fill(config.get_bg_surface())
                .stroke(egui::Stroke::new(1.0, config.get_border_default()))
                .inner_margin(egui::Margin::symmetric(24.0, 16.0));
                
            header_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    // App logo/title - dashboard style
                    ui.label(egui::RichText::new("Scord")
                        .size(18.0)
                        .color(egui::Color32::from_gray(252)) // #fcfcfc - white
                        .strong());
                    
                    ui.add_space(32.0);
                    
                    // Dashboard breadcrumb style
                    ui.label(egui::RichText::new("Scoring Dashboard")
                        .size(13.0)
                        .color(egui::Color32::from_gray(149))); // #959597 - taupe-gray
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Settings button - dashboard style
                        let settings_btn = egui::Button::new(egui::RichText::new("⚙ Settings")
                            .size(14.0)
                            .color(egui::Color32::from_gray(252))) // #fcfcfc - white
                            .min_size(egui::vec2(80.0, 32.0))
                            .rounding(egui::Rounding::same(8.0));
                            
                        if ui.add(settings_btn).clicked() {
                            self.visual_settings.toggle();
                        }
                    });
                });
                
                ui.add_space(12.0);
                
                // Tab bar with dashboard styling
                if let Some(action) = TabBar::show(ui, &mut self.tab_manager, self.visual_settings.get_config()) {
                    match action {
                        TabAction::SwitchTo(index) => {
                            self.tab_manager.set_active_tab(index);
                        }
                        TabAction::Close(index) => {
                            if self.tab_manager.can_close_tab(index) {
                                self.tab_manager.close_tab(index);
                            }
                        }
                        TabAction::NewTab => {
                            self.tab_manager.add_tab();
                        }
                    }
                }
            });
        });

        // Tab-specific content area
        let (status_message, show_status) = if let Some(active_tab) = self.tab_manager.get_active_tab() {
            active_tab.content.get_status()
        } else {
            ("", false)
        };
        
        // Status bar
        if show_status {
            let status_message = status_message.to_string(); // Copy the string
            TopBottomPanel::bottom("status").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&status_message);
                    if ui.button("×").clicked() {
                        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
                            tab.content.hide_status();
                        }
                    }
                });
            });
        }

        // Dashboard-style toolbar
        TopBottomPanel::top("toolbar").show(ctx, |ui| {
            let config = self.visual_settings.get_config();
            
            let toolbar_frame = egui::Frame::none()
                .fill(config.get_bg_elevated())
                .stroke(egui::Stroke::new(1.0, config.get_border_default()))
                .inner_margin(egui::Margin::symmetric(24.0, 12.0));
                
            toolbar_frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let button_height = 36.0;
                        
                        // Export Results - primary action
                        let export_results_btn = egui::Button::new(egui::RichText::new("📊 Export Results")
                            .size(14.0)
                            .color(egui::Color32::from_gray(252))) // #fcfcfc - white
                            .min_size(egui::vec2(130.0, button_height))
                            .rounding(egui::Rounding::same(8.0));
                            
                        if ui.add(export_results_btn).clicked() {
                            if let Some(tab) = self.tab_manager.get_active_tab_mut() {
                                let _ = tab.content.export_results();
                            }
                        }
                        
                        ui.add_space(12.0);
                        
                        // Export Data - secondary action
                        let export_data_btn = egui::Button::new(egui::RichText::new("💾 Export Data")
                            .size(14.0)
                            .color(egui::Color32::from_gray(149))) // #959597 - taupe-gray
                            .min_size(egui::vec2(110.0, button_height))
                            .rounding(egui::Rounding::same(8.0));
                            
                        if ui.add(export_data_btn).clicked() {
                            if let Some(tab) = self.tab_manager.get_active_tab_mut() {
                                let _ = tab.content.export_data();
                            }
                        }
                        
                        ui.add_space(12.0);
                        
                        // Import - secondary action
                        let import_btn = egui::Button::new(egui::RichText::new("📁 Import")
                            .size(14.0)
                            .color(egui::Color32::from_gray(149))) // #959597 - taupe-gray
                            .min_size(egui::vec2(90.0, button_height))
                            .rounding(egui::Rounding::same(8.0));
                            
                        if ui.add(import_btn).clicked() {
                            if let Some(tab) = self.tab_manager.get_active_tab_mut() {
                                let _ = tab.content.import_data();
                            }
                        }
                        
                        ui.add_space(20.0);
                        
                        // Format selector - dashboard style dropdown
                        let current_format = if let Some(tab) = self.tab_manager.get_active_tab() {
                            tab.content.get_export_format()
                        } else {
                            crate::models::ExportFormat::Json
                        };
                        
                        let selector_frame = egui::Frame::none()
                            .fill(config.get_bg_surface())
                            .stroke(egui::Stroke::new(1.0, config.get_border_default()))
                            .rounding(egui::Rounding::same(8.0))
                            .inner_margin(egui::Margin::symmetric(4.0, 2.0));
                            
                        selector_frame.show(ui, |ui| {
                            ui.allocate_ui_with_layout(
                                egui::Vec2::new(120.0, button_height - 8.0),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    let mut export_format = current_format;
                                    egui::ComboBox::from_label("")
                                        .selected_text(format!("Format: {}", match export_format {
                                            crate::models::ExportFormat::Json => "JSON",
                                            crate::models::ExportFormat::Csv => "CSV",
                                        }))
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut export_format, crate::models::ExportFormat::Json, "JSON");
                                            ui.selectable_value(&mut export_format, crate::models::ExportFormat::Csv, "CSV");
                                        });
                                    
                                    if export_format != current_format {
                                        if let Some(tab) = self.tab_manager.get_active_tab_mut() {
                                            tab.content.set_export_format(export_format);
                                        }
                                    }
                                }
                            );
                        });
                    });
                });
            });
        });

        // Dashboard main content area with sidebar layout
        let config = self.visual_settings.get_config().clone();
        
        // Right sidebar for results - dashboard style
        SidePanel::right("results_sidebar")
            .resizable(true)
            .default_width(350.0)
            .min_width(280.0)
            .max_width(500.0)
            .frame(egui::Frame::none()
                .fill(config.get_bg_primary())
                .inner_margin(egui::Margin::same(16.0)))
            .show(ctx, |ui| {
                if let Some((_contestants, _properties, score_results)) = self.get_active_tab_data() {
                    ResultsPanel::show(ui, score_results, &config);
                }
            });

        // Central content area with dashboard layout
        CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(config.get_bg_primary())
                .inner_margin(egui::Margin::same(16.0)))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Controls section
                    let mut is_collapsed = self.controls_panel.is_collapsed;
                    ControlsPanel::show(ui, self, &mut is_collapsed);
                    self.controls_panel.is_collapsed = is_collapsed;
                    
                    ui.add_space(20.0);
                    
                    // Main data section
                    SpreadsheetView::show(ui, self, &config);
                });
            });
        
        // Visual settings panel
        if self.visual_settings.show(ctx) {
            setup_custom_style_with_config(ctx, self.visual_settings.get_config());
        }
        
        // Handle keyboard shortcut for visual settings
        ctx.input(|i| {
            if i.key_pressed(egui::Key::Comma) && i.modifiers.ctrl {
                self.visual_settings.toggle();
            }
        });
    }
}