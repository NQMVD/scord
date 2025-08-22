use crate::models::{ScoreResult, VisualConfig};
use egui::{Ui, ScrollArea, Color32, Stroke, Rounding, Frame, Margin, Shadow};

pub struct ResultsPanel;

impl ResultsPanel {
    pub fn show(ui: &mut Ui, score_results: &[ScoreResult], config: &VisualConfig) {
        // Dashboard-style panel container
        let panel_frame = Frame::none()
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
            
        panel_frame.show(ui, |ui| {
            ui.vertical(|ui| {
                // Panel header with dashboard styling
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Scoring Results")
                        .size(16.0)
                        .color(config.get_text_primary())
                        .strong());
                });
                
                ui.add_space(16.0);
                
                if score_results.is_empty() {
                    // Empty state card
                    let empty_card = Frame::none()
                        .fill(config.get_bg_elevated())
                        .stroke(Stroke::new(1.0, config.get_border_hover()))
                        .rounding(Rounding::same(12.0))
                        .inner_margin(Margin::same(32.0));
                        
                    empty_card.show(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("No Results Yet")
                                    .size(14.0)
                                    .color(config.get_text_secondary()));
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("Add contestants and properties\nto see scoring results")
                                    .size(13.0)
                                    .color(config.get_text_secondary()));
                            });
                        });
                    });
                    return;
                }

                ScrollArea::vertical()
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        ui.style_mut().spacing.item_spacing.y = 12.0;
                        
                        for (index, result) in score_results.iter().enumerate() {
                            Self::show_result_card(ui, index, result, config);
                        }
                        
                        ui.add_space(8.0);
                    });
            });
        });
    }
    
    fn show_result_card(ui: &mut Ui, index: usize, result: &ScoreResult, config: &VisualConfig) {
        // Dashboard-style result card
        let card_color = if index == 0 {
            config.get_bg_extreme() // raisin-black-2 for winner
        } else {
            config.get_bg_elevated() // raisin-black for others
        };
        
        let border_color = if index == 0 {
            config.get_accent_color() // Accent color for winner
        } else {
            config.get_border_default() // onyx
        };
        
        let result_card = Frame::none()
            .fill(card_color)
            .stroke(Stroke::new(1.0, border_color))
            .rounding(Rounding::same(12.0))
            .inner_margin(Margin::same(16.0));
            
        result_card.show(ui, |ui| {
            ui.vertical(|ui| {
                // Main result row
                ui.horizontal(|ui| {
                    // Rank badge
                    let rank_frame = Frame::none()
                        .fill(if index == 0 { config.get_accent_color() } else { config.get_border_hover() })
                        .rounding(Rounding::same(16.0))
                        .inner_margin(Margin::symmetric(8.0, 4.0));
                        
                    rank_frame.show(ui, |ui| {
                        ui.label(egui::RichText::new(format!("#{}", index + 1))
                            .size(12.0)
                            .color(config.get_text_primary())
                            .strong());
                    });
                    
                    ui.add_space(12.0);
                    
                    // Contestant name
                    ui.label(egui::RichText::new(&result.name)
                        .size(14.0)
                        .color(config.get_text_primary())
                        .strong());
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Score badge
                        let score_frame = Frame::none()
                            .fill(config.get_bg_primary()) // night background
                            .stroke(Stroke::new(1.0, config.get_border_hover())) // onyx-2
                            .rounding(Rounding::same(8.0))
                            .inner_margin(Margin::symmetric(12.0, 6.0));
                            
                        score_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new(format!("{:.1}%", result.score))
                                .size(14.0)
                                .color(if index == 0 { config.get_accent_color() } else { config.get_text_primary() })
                                .strong());
                        });
                    });
                });
                
                // Best properties section
                if !result.best_properties.is_empty() {
                    ui.add_space(8.0);
                    
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new("Best at:")
                            .size(12.0)
                            .color(config.get_text_secondary()));
                        
                        for prop in &result.best_properties {
                            let prop_tag = Frame::none()
                                .fill(config.get_border_hover()) // onyx-2
                                .rounding(Rounding::same(6.0))
                                .inner_margin(Margin::symmetric(8.0, 3.0));
                                
                            prop_tag.show(ui, |ui| {
                                ui.label(egui::RichText::new(prop)
                                    .size(11.0)
                                    .color(config.get_text_primary()));
                            });
                        }
                    });
                }
            });
        });
    }
}