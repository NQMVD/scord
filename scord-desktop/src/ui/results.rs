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
            for (index, result) in score_results.iter().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("#{}", index + 1));
                        ui.label(&result.name);
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.strong(format!("{:.1}%", result.score));
                        });
                    });
                    
                    if !result.best_properties.is_empty() {
                        ui.horizontal_wrapped(|ui| {
                            ui.small("Best at:");
                            for prop in &result.best_properties {
                                ui.small(prop);
                            }
                        });
                    }
                });
                
                ui.add_space(4.0);
            }
        });
    }
}