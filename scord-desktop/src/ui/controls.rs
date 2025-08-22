use crate::app::ScordApp;
use crate::models::VisualConfig;
use egui::{Ui, Color32, Stroke, Rounding, Frame, Margin, Shadow};

pub struct ControlsPanel;

impl ControlsPanel {
    pub fn show(ui: &mut Ui, app: &mut ScordApp) {
        let config = &crate::models::VisualConfig::load();
        
        // Dashboard-style card container
        let card_frame = Frame::none()
            .fill(config.get_bg_surface())
            .stroke(Stroke::new(1.0, config.get_border_default()))
            .rounding(Rounding::same(18.0))
            .shadow(Shadow {
                offset: egui::vec2(0.0, 4.0),
                blur: 12.0,
                spread: 0.0,
                color: Color32::from_black_alpha(40),
            })
            .inner_margin(Margin::same(24.0));
            
        card_frame.show(ui, |ui| {
            ui.vertical(|ui| {
                // Card header
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Add New Items")
                        .size(16.0)
                        .color(config.get_text_primary())
                        .strong());
                });
                
                ui.add_space(16.0);
                
                // Contestant section - dashboard card style
                Self::show_section_card(ui, config, "Contestant", |ui| {
                    ui.horizontal(|ui| {
                        if let Some((new_contestant_name, _, _, _)) = app.get_active_tab_ui_state_mut() {
                            let text_edit = egui::TextEdit::singleline(new_contestant_name)
                                .hint_text("Enter contestant name...")
                                .desired_width(200.0)
                                .font(egui::TextStyle::Body);
                            ui.add_sized([200.0, 36.0], text_edit);
                        }
                        
                        ui.add_space(12.0);
                        
                        let add_btn = egui::Button::new(egui::RichText::new("Add Contestant")
                            .size(14.0)
                            .color(Color32::WHITE))
                            .min_size(egui::vec2(120.0, 36.0))
                            .rounding(Rounding::same(8.0));
                            
                        if ui.add(add_btn).clicked() || 
                           (ui.input(|i| i.key_pressed(egui::Key::Enter)) && ui.memory(|m| m.has_focus(ui.id().with("contestant_input")))) {
                            app.add_contestant();
                        }
                    });
                });

                ui.add_space(16.0);

                // Property section - dashboard card style
                Self::show_section_card(ui, config, "Property", |ui| {
                    ui.horizontal(|ui| {
                        if let Some((_, new_property_name, _, _)) = app.get_active_tab_ui_state_mut() {
                            let text_edit = egui::TextEdit::singleline(new_property_name)
                                .hint_text("Enter property name...")
                                .desired_width(160.0)
                                .font(egui::TextStyle::Body);
                            ui.add_sized([160.0, 36.0], text_edit);
                        }
                        
                        ui.add_space(12.0);
                        
                        if let Some((_, _, new_property_weight, _)) = app.get_active_tab_ui_state_mut() {
                            let weight_frame = Frame::none()
                                .fill(config.get_bg_elevated())
                                .stroke(Stroke::new(1.0, config.get_border_default()))
                                .rounding(Rounding::same(8.0))
                                .inner_margin(Margin::symmetric(12.0, 8.0));
                                
                            weight_frame.show(ui, |ui| {
                                ui.add_sized([100.0, 20.0], egui::DragValue::new(new_property_weight)
                                    .speed(0.1)
                                    .range(0.1..=10.0)
                                    .prefix("Weight: "));
                            });
                        }
                        
                        ui.add_space(12.0);
                        
                        if let Some((_, _, _, new_property_higher_is_better)) = app.get_active_tab_ui_state_mut() {
                            let checkbox_frame = Frame::none()
                                .fill(config.get_bg_elevated())
                                .stroke(Stroke::new(1.0, config.get_border_default()))
                                .rounding(Rounding::same(8.0))
                                .inner_margin(Margin::symmetric(12.0, 8.0));
                                
                            checkbox_frame.show(ui, |ui| {
                                ui.checkbox(new_property_higher_is_better, "Higher is better");
                            });
                        }
                        
                        ui.add_space(12.0);
                        
                        let add_btn = egui::Button::new(egui::RichText::new("Add Property")
                            .size(14.0)
                            .color(Color32::WHITE))
                            .min_size(egui::vec2(110.0, 36.0))
                            .rounding(Rounding::same(8.0));
                            
                        if ui.add(add_btn).clicked() {
                            app.add_property();
                        }
                    });
                });
            });
        });
    }
    
    fn show_section_card<R>(
        ui: &mut Ui, 
        config: &VisualConfig, 
        title: &str, 
        content: impl FnOnce(&mut Ui) -> R
    ) -> R {
        let section_frame = Frame::none()
            .fill(config.get_bg_elevated())
            .stroke(Stroke::new(1.0, config.get_border_default()))
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(16.0));
            
        section_frame.show(ui, |ui| {
            ui.vertical(|ui| {
                // Section title
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new(title)
                        .size(14.0)
                        .color(config.get_text_secondary())
                        .strong());
                });
                
                ui.add_space(8.0);
                
                content(ui)
            }).inner
        }).inner
    }
}