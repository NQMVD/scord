use crate::app::ScordApp;
use crate::models::VisualConfig;
use egui::{Ui, ScrollArea, Grid, Color32, Stroke, Rounding, Frame, Margin, Shadow};


pub struct SpreadsheetView;

impl SpreadsheetView {
    pub fn show(ui: &mut Ui, app: &mut ScordApp, config: &VisualConfig) {
        // Dashboard-style main content card
        let main_card = Frame::none()
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
            
        main_card.show(ui, |ui| {
            ui.vertical(|ui| {
                // Card header with dashboard styling
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Contestant Data")
                        .size(16.0)
                        .color(Color32::from_gray(252)) // #fcfcfc - white
                        .strong());
                });
                
                ui.add_space(16.0);
                
                ScrollArea::both()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
            // Get active tab data
            let (contestants, properties, _score_results) = match app.get_active_tab_data() {
                Some(data) => data,
                None => {
                    ui.centered_and_justified(|ui| {
                        ui.label("No active tab");
                    });
                    return;
                }
            };
            
            if contestants.is_empty() && properties.is_empty() {
                // Dashboard-style empty state
                let empty_state_card = Frame::none()
                    .fill(config.get_bg_elevated())
                    .stroke(Stroke::new(1.0, config.get_border_default()))
                    .rounding(Rounding::same(12.0))
                    .inner_margin(Margin::same(48.0));
                    
                empty_state_card.show(ui, |ui| {
                    ui.centered_and_justified(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(egui::RichText::new("📊 Get Started")
                                .size(18.0)
                                .color(Color32::from_gray(252)) // #fcfcfc - white
                                .strong());
                            ui.add_space(8.0);
                            ui.label(egui::RichText::new("Add contestants and properties\nto begin scoring")
                                .size(14.0)
                                .color(Color32::from_gray(149))); // #959597 - taupe-gray
                        });
                    });
                });
                return;
            }

            let properties_to_delete = std::cell::RefCell::new(Vec::new());
            let contestants_to_delete = std::cell::RefCell::new(Vec::new());
            let value_updates = std::cell::RefCell::new(Vec::new());
            let property_updates = std::cell::RefCell::new(Vec::new());
            let contestant_name_updates = std::cell::RefCell::new(Vec::new());

            Grid::new("spreadsheet_grid")
                .striped(false) // Disable alternating row colors
                .spacing(egui::vec2(12.0, 8.0)) // Better spacing between rows and columns
                .min_col_width(130.0) // Make columns wider for better readability
                .show(ui, |ui| {
                    // Header row
                    ui.strong("Contestant");
                    
                    for property in properties {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0); // Even wider for property columns
                            
                            // Property name input - full width
                            let mut name = property.name.clone();
                            let textbox = egui::TextEdit::singleline(&mut name)
                                .vertical_align(egui::Align::Center)
                                .margin(egui::Margin::symmetric(8.0, 0.0))
                                .font(egui::TextStyle::Button);
                            let response = ui.add_sized([ui.available_width(), 32.0], textbox);
                            if response.changed() {
                                property_updates.borrow_mut().push((
                                    property.id, 
                                    name, 
                                    property.weight, 
                                    property.higher_is_better
                                ));
                            }
                            
                            ui.add_space(4.0); // Small spacing
                            
                            // Controls in single horizontal line: [Weight Input] [Direction] [Delete]
                            ui.horizontal(|ui| {
                                // Weight input - half width
                                let mut weight = property.weight;
                                if ui.add_sized([45.0, 32.0], egui::DragValue::new(&mut weight)
                                    .speed(0.1)
                                    .range(0.1..=10.0)
                                    .fixed_decimals(1)).changed() {
                                    property_updates.borrow_mut().push((
                                        property.id, 
                                        property.name.clone(), 
                                        weight, 
                                        property.higher_is_better
                                    ));
                                }
                                
                                ui.add_space(4.0);
                                
                                // Direction button - square, icon only
                                let direction_response = ui.add_sized([32.0, 32.0], egui::Button::new(""));
                                
                                // Draw custom arrow on the button
                                if ui.is_rect_visible(direction_response.rect) {
                                    let center = direction_response.rect.center();
                                    let arrow_size = 6.0;
                                    let color = ui.style().visuals.text_color();
                                    
                                    if property.higher_is_better {
                                        // Up arrow
                                        let points = vec![
                                            egui::pos2(center.x, center.y - arrow_size),
                                            egui::pos2(center.x - arrow_size, center.y + arrow_size),
                                            egui::pos2(center.x + arrow_size, center.y + arrow_size),
                                        ];
                                        ui.painter().add(egui::Shape::convex_polygon(
                                            points, color, egui::Stroke::NONE
                                        ));
                                    } else {
                                        // Down arrow
                                        let points = vec![
                                            egui::pos2(center.x, center.y + arrow_size),
                                            egui::pos2(center.x - arrow_size, center.y - arrow_size),
                                            egui::pos2(center.x + arrow_size, center.y - arrow_size),
                                        ];
                                        ui.painter().add(egui::Shape::convex_polygon(
                                            points, color, egui::Stroke::NONE
                                        ));
                                    }
                                }
                                
                                if direction_response.clicked() {
                                    property_updates.borrow_mut().push((
                                        property.id, 
                                        property.name.clone(), 
                                        property.weight, 
                                        !property.higher_is_better
                                    ));
                                }
                                
                                let tooltip = if property.higher_is_better { 
                                    "Higher is better (click to change)"
                                } else { 
                                    "Lower is better (click to change)"
                                };
                                direction_response.on_hover_text(tooltip);
                                
                                ui.add_space(4.0);
                                
                                // Delete button - square, icon only
                                if ui.add_sized([32.0, 32.0], egui::Button::new("×")).on_hover_text("Delete property").clicked() {
                                    properties_to_delete.borrow_mut().push(property.id);
                                }
                            });
                        });
                    }
                    
                    ui.strong("Actions");
                    ui.end_row();

                    // Data rows
                    for contestant in contestants {
                        // Editable contestant name with consistent height and proper text alignment
                        let mut name = contestant.name.clone();
                        let textbox = egui::TextEdit::singleline(&mut name)
                            .vertical_align(egui::Align::Center)
                            .margin(egui::Margin::symmetric(8.0, 0.0));
                        let response = ui.add_sized([ui.available_width(), 32.0], textbox);
                        if response.changed() {
                            contestant_name_updates.borrow_mut().push((contestant.id, name));
                        }
                        
                        // Values for each property with consistent height
                        for property in properties {
                            let value = contestant.get_value(&property.id);
                            let mut temp_value = value;
                            
                            if ui.add_sized([ui.available_width(), 32.0], egui::DragValue::new(&mut temp_value)
                                .speed(0.1)
                                .fixed_decimals(1)).changed() {
                                value_updates.borrow_mut().push((contestant.id, property.id, temp_value));
                            }
                        }
                        
                        // Delete button
                        if ui.small_button("Delete").clicked() {
                            contestants_to_delete.borrow_mut().push(contestant.id);
                        }
                        
                        ui.end_row();
                    }
                });
            
            // Process updates and deletions
            for property_id in properties_to_delete.into_inner() {
                app.delete_property(property_id);
            }
            
            for (property_id, name, weight, higher_is_better) in property_updates.into_inner() {
                app.update_property(property_id, name, weight, higher_is_better);
            }
            
            for (contestant_id, name) in contestant_name_updates.into_inner() {
                app.update_contestant_name(contestant_id, name);
            }
            
            for (contestant_id, property_id, value) in value_updates.into_inner() {
                app.update_contestant_value(contestant_id, property_id, value);
            }
            
            for contestant_id in contestants_to_delete.into_inner() {
                app.delete_contestant(contestant_id);
            }
                    });
                });
            });
    }
}