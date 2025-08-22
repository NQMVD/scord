use crate::models::ScoreResult;
use egui::{Ui, ScrollArea, Color32, Stroke, Rounding, Frame, Margin, Shadow};

pub struct ResultsPanel;

impl ResultsPanel {
    pub fn show(ui: &mut Ui, score_results: &[ScoreResult]) {
        // Dashboard-style panel container
        let panel_frame = Frame::none()
            .fill(Color32::from_gray(25)) // #18191A - dashboard surface color
            .stroke(Stroke::new(1.0, Color32::from_gray(43))) // #2A2B2C - dashboard border
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
                        .color(Color32::WHITE)
                        .strong());
                });
                
                ui.add_space(16.0);
                
                if score_results.is_empty() {
                    // Empty state card
                    let empty_card = Frame::none()
                        .fill(Color32::from_gray(43)) // #2A2B2C
                        .stroke(Stroke::new(1.0, Color32::from_gray(60)))
                        .rounding(Rounding::same(12.0))
                        .inner_margin(Margin::same(32.0));
                        
                    empty_card.show(ui, |ui| {
                        ui.centered_and_justified(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(egui::RichText::new("No Results Yet")
                                    .size(14.0)
                                    .color(Color32::from_gray(122)));
                                ui.add_space(4.0);
                                ui.label(egui::RichText::new("Add contestants and properties\nto see scoring results")
                                    .size(13.0)
                                    .color(Color32::from_gray(100)));
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
                            Self::show_result_card(ui, index, result);
                        }
                        
                        ui.add_space(8.0);
                    });
            });
        });
    }
    
    fn show_result_card(ui: &mut Ui, index: usize, result: &ScoreResult) {
        // Dashboard-style result card
        let card_color = if index == 0 {
            Color32::from_rgb(50, 45, 60) // Slightly purple tint for winner
        } else {
            Color32::from_gray(43) // #2A2B2C - standard elevated color
        };
        
        let border_color = if index == 0 {
            Color32::from_rgb(132, 126, 255) // Accent color for winner
        } else {
            Color32::from_gray(60)
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
                        .fill(if index == 0 { Color32::from_rgb(132, 126, 255) } else { Color32::from_gray(60) })
                        .rounding(Rounding::same(16.0))
                        .inner_margin(Margin::symmetric(8.0, 4.0));
                        
                    rank_frame.show(ui, |ui| {
                        ui.label(egui::RichText::new(format!("#{}", index + 1))
                            .size(12.0)
                            .color(Color32::WHITE)
                            .strong());
                    });
                    
                    ui.add_space(12.0);
                    
                    // Contestant name
                    ui.label(egui::RichText::new(&result.name)
                        .size(14.0)
                        .color(Color32::WHITE)
                        .strong());
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Score badge
                        let score_frame = Frame::none()
                            .fill(Color32::from_gray(15)) // Dark background
                            .stroke(Stroke::new(1.0, Color32::from_gray(80)))
                            .rounding(Rounding::same(8.0))
                            .inner_margin(Margin::symmetric(12.0, 6.0));
                            
                        score_frame.show(ui, |ui| {
                            ui.label(egui::RichText::new(format!("{:.1}%", result.score))
                                .size(14.0)
                                .color(if index == 0 { Color32::from_rgb(132, 126, 255) } else { Color32::WHITE })
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
                            .color(Color32::from_gray(122)));
                        
                        for prop in &result.best_properties {
                            let prop_tag = Frame::none()
                                .fill(Color32::from_gray(80))
                                .rounding(Rounding::same(6.0))
                                .inner_margin(Margin::symmetric(8.0, 3.0));
                                
                            prop_tag.show(ui, |ui| {
                                ui.label(egui::RichText::new(prop)
                                    .size(11.0)
                                    .color(Color32::WHITE));
                            });
                        }
                    });
                }
            });
        });
    }
}