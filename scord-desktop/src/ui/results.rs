use crate::models::ScoreResult;
use egui::{Ui, ScrollArea};

pub struct ResultsPanel;

impl ResultsPanel {
    pub fn show(ui: &mut Ui, score_results: &[ScoreResult]) {
        ui.heading("Scoring Results");
        
        if score_results.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("Add contestants and properties\nto see scoring results");
            });
            return;
        }

        ScrollArea::vertical().show(ui, |ui| {
            ui.style_mut().spacing.item_spacing.y = 4.0;
            ui.add_space(8.0);
            
            for (index, result) in score_results.iter().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        // Use RichText to make rank number darker like webapp
                        ui.label(
                            egui::RichText::new(format!("#{}", index + 1))
                                .color(egui::Color32::from_rgb(136, 136, 136)) // charcoal-400 - darker than default
                        );
                        ui.label(&result.name);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.strong(format!("{:.1}%", result.score));
                        });
                    });
                    
                    if !result.best_properties.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            ui.small(
                                egui::RichText::new("Best at:")
                                    .color(egui::Color32::from_rgb(136, 136, 136)) // charcoal-400
                            );
                            for prop in &result.best_properties {
                                ui.small(prop);
                            }
                        });
                    }
                });
            }
            
            ui.add_space(8.0);
        });
    }
}