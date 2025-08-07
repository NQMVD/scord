use crate::app::ScordApp;
use egui::{Ui, ScrollArea, Grid};


pub struct SpreadsheetView;

impl SpreadsheetView {
    pub fn show(ui: &mut Ui, app: &mut ScordApp) {
        ui.heading("Contestant Data");
        
        ScrollArea::both().show(ui, |ui| {
            if app.contestants.is_empty() && app.properties.is_empty() {
                ui.centered_and_justified(|ui| {
                    ui.label("Add contestants and properties to get started");
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
                .spacing(egui::vec2(8.0, 4.0)) // Add spacing between rows and columns
                .min_col_width(120.0) // Make columns wider
                .show(ui, |ui| {
                    // Header row
                    ui.strong("Contestant");
                    
                    for property in &app.properties {
                        ui.vertical(|ui| {
                            ui.set_min_width(150.0); // Even wider for property columns
                            
                            // Editable property name with consistent height and proper text alignment
                            let mut name = property.name.clone();
                            egui::Frame::none()
                                .inner_margin(egui::Margin::symmetric(0.0, 3.0)) // Fix text vertical alignment
                                .show(ui, |ui| {
                                    let mut textbox = egui::TextEdit::singleline(&mut name);
                                    let response = ui.add_sized([ui.available_width(), 22.0], textbox);
                                    if response.changed() {
                                        property_updates.borrow_mut().push((
                                            property.id, 
                                            name, 
                                            property.weight, 
                                            property.higher_is_better
                                        ));
                                    }
                                });
                            
                            // Weight and direction controls with proper alignment
                            let size = egui::Vec2::new(ui.available_width(), ui.spacing().interact_size.y + 4.0);
                            ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(egui::Align::Center), |ui| {
                                ui.label("Weight:");
                                let mut weight = property.weight;
                                if ui.add_sized([80.0, 20.0], egui::DragValue::new(&mut weight)
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
                                
                                let tooltip = if property.higher_is_better { 
                                    "Higher is better (click to change)"
                                } else { 
                                    "Lower is better (click to change)"
                                };
                                
                                // Custom arrow button with drawing
                                let desired_size = egui::vec2(20.0, 16.0);
                                let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
                                
                                if ui.is_rect_visible(rect) {
                                    let visuals = ui.style().interact(&response);
                                    
                                    // Draw button background
                                    ui.painter().rect_filled(rect, 2.0, visuals.bg_fill);
                                    ui.painter().rect_stroke(rect, 2.0, visuals.bg_stroke);
                                    
                                    // Draw arrow triangle
                                    let center = rect.center();
                                    let size = 4.0;
                                    let color = visuals.text_color();
                                    
                                    if property.higher_is_better {
                                        // Up arrow triangle
                                        let points = vec![
                                            egui::pos2(center.x, center.y - size),
                                            egui::pos2(center.x - size, center.y + size),
                                            egui::pos2(center.x + size, center.y + size),
                                        ];
                                        ui.painter().add(egui::Shape::convex_polygon(
                                            points, color, egui::Stroke::NONE
                                        ));
                                    } else {
                                        // Down arrow triangle
                                        let points = vec![
                                            egui::pos2(center.x, center.y + size),
                                            egui::pos2(center.x - size, center.y - size),
                                            egui::pos2(center.x + size, center.y - size),
                                        ];
                                        ui.painter().add(egui::Shape::convex_polygon(
                                            points, color, egui::Stroke::NONE
                                        ));
                                    }
                                }
                                
                                if response.clicked() {
                                    property_updates.borrow_mut().push((
                                        property.id, 
                                        property.name.clone(), 
                                        property.weight, 
                                        !property.higher_is_better
                                    ));
                                }
                                
                                response.on_hover_text(tooltip);
                            });
                            
                            // Delete button
                            if ui.small_button("× Delete").clicked() {
                                properties_to_delete.borrow_mut().push(property.id);
                            }                        });
                    }
                    
                    ui.strong("Actions");
                    ui.end_row();
                    
                    // Add separator line after header
                    for _ in 0..=app.properties.len() + 1 {
                        ui.separator();
                    }
                    ui.end_row();

                    // Data rows
                    for (row_index, contestant) in app.contestants.iter().enumerate() {
                        // Editable contestant name with consistent height and proper text alignment
                        let mut name = contestant.name.clone();
                        egui::Frame::none()
                            .inner_margin(egui::Margin::symmetric(0.0, 3.0)) // Fix text vertical alignment
                            .show(ui, |ui| {
                                let mut textbox = egui::TextEdit::singleline(&mut name);
                                let response = ui.add_sized([ui.available_width(), 22.0], textbox);
                                if response.changed() {
                                    contestant_name_updates.borrow_mut().push((contestant.id, name));
                                }
                            });
                        
                        // Values for each property with consistent height
                        for property in &app.properties {
                            let value = contestant.get_value(&property.id);
                            let mut temp_value = value;
                            
                            if ui.add_sized([ui.available_width(), 20.0], egui::DragValue::new(&mut temp_value)
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
                        
                        // Add separator line between rows (except after last row)
                        if row_index < app.contestants.len() - 1 {
                            for _ in 0..=app.properties.len() + 1 {
                                ui.separator();
                            }
                            ui.end_row();
                        }
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
    }
}