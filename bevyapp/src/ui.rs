use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::data::{AppState, EditingState, ExportFormat};

pub fn ui_system(
    mut contexts: EguiContexts,
    mut app_state: ResMut<AppState>,
    mut editing_state: ResMut<EditingState>,
) {
    setup_dark_theme(contexts.ctx_mut());

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            // Header
            render_header(ui);
            
            ui.separator();
            
            // Main content area
            ui.horizontal_top(|ui| {
                // Left panel - Main spreadsheet
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width() * 0.7, ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        render_spreadsheet_panel(ui, &mut app_state, &mut editing_state);
                    },
                );
                
                ui.separator();
                
                // Right panel - Results
                ui.allocate_ui_with_layout(
                    egui::Vec2::new(ui.available_width(), ui.available_height()),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        render_results_panel(ui, &app_state);
                    },
                );
            });
        });
    });
}

fn setup_dark_theme(ctx: &mut egui::Context) {
    let mut visuals = egui::Visuals::dark();
    
    // Customize colors to match the original charcoal theme
    visuals.window_fill = egui::Color32::from_rgb(23, 23, 23); // charcoal-950
    visuals.panel_fill = egui::Color32::from_rgb(38, 38, 38); // charcoal-900
    visuals.faint_bg_color = egui::Color32::from_rgb(64, 64, 64); // charcoal-800
    visuals.extreme_bg_color = egui::Color32::from_rgb(23, 23, 23);
    visuals.code_bg_color = egui::Color32::from_rgb(64, 64, 64);
    
    visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(82, 82, 82)); // charcoal-700
    visuals.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(82, 82, 82));
    visuals.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(82, 82, 82));
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(115, 115, 115));
    visuals.widgets.active.bg_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(161, 161, 170));
    
    visuals.selection.bg_fill = egui::Color32::from_rgb(82, 82, 82);
    
    ctx.set_visuals(visuals);
}

fn render_header(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.heading(egui::RichText::new("> scord").size(28.0).color(egui::Color32::WHITE));
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.link("GitHub").clicked() {
                // In a real app, you'd open the URL
                println!("GitHub link clicked");
            }
            
            if ui.link("Home").clicked() {
                // In a real app, you'd open the URL
                println!("Home link clicked");
            }
        });
    });
}

fn render_spreadsheet_panel(
    ui: &mut egui::Ui,
    app_state: &mut AppState,
    editing_state: &mut EditingState,
) {
    ui.vertical(|ui| {
        ui.heading("Contestant Data");
        
        ui.separator();
        
        // Controls section
        render_controls(ui, app_state);
        
        ui.separator();
        
        // Spreadsheet table
        render_spreadsheet_table(ui, app_state, editing_state);
    });
}

fn render_controls(ui: &mut egui::Ui, app_state: &mut AppState) {
    ui.vertical(|ui| {
        // Contestant controls
        ui.horizontal(|ui| {
            ui.label("New:");
            ui.text_edit_singleline(&mut app_state.new_contestant_name);
            if ui.button("Add Contestant").clicked() {
                if !app_state.new_contestant_name.trim().is_empty() {
                    app_state.add_contestant(app_state.new_contestant_name.trim().to_string());
                    app_state.new_contestant_name.clear();
                }
            }
        });
        
        // Property controls
        ui.horizontal(|ui| {
            ui.label("New:");
            ui.text_edit_singleline(&mut app_state.new_property_name);
            ui.add(egui::DragValue::new(&mut app_state.new_property_weight).speed(0.1).range(0.1..=100.0));
            ui.checkbox(&mut app_state.new_property_higher_is_better, "Higher is better");
            if ui.button("Add Property").clicked() {
                if !app_state.new_property_name.trim().is_empty() {
                    app_state.add_property(
                        app_state.new_property_name.trim().to_string(),
                        app_state.new_property_weight,
                        app_state.new_property_higher_is_better,
                    );
                    app_state.new_property_name.clear();
                    app_state.new_property_weight = 1.0;
                    app_state.new_property_higher_is_better = true;
                }
            }
        });
        
        // Export/Import controls
        ui.horizontal(|ui| {
            ui.label("Format:");
            egui::ComboBox::from_label("")
                .selected_text(match app_state.export_format {
                    ExportFormat::Json => "JSON",
                    ExportFormat::Csv => "CSV",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut app_state.export_format, ExportFormat::Json, "JSON");
                    ui.selectable_value(&mut app_state.export_format, ExportFormat::Csv, "CSV");
                });
                
            if ui.button("📄 Export Data").clicked() {
                export_data(app_state);
            }
            
            if ui.button("🏆 Export Results").clicked() {
                export_results(app_state);
            }
            
            if ui.button("📥 Import").clicked() {
                import_data(app_state);
            }
        });
    });
}

fn render_spreadsheet_table(
    ui: &mut egui::Ui,
    app_state: &mut AppState,
    editing_state: &mut EditingState,
) {
    egui::ScrollArea::both().id_source("spreadsheet_table").show(ui, |ui| {
        egui::Grid::new("spreadsheet")
            .num_columns(app_state.properties.len() + 2) // +2 for contestant name and actions
            .spacing([4.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                // Header row
                ui.label("Contestant");
                
                for property in &app_state.properties.clone() {
                    render_property_header(ui, property, app_state, editing_state);
                }
                
                ui.label("Actions");
                ui.end_row();
                
                // Data rows
                for contestant in app_state.contestants.clone() {
                    render_contestant_row(ui, &contestant, app_state, editing_state);
                }
            });
    });
}

fn render_property_header(
    ui: &mut egui::Ui,
    property: &crate::data::Property,
    app_state: &mut AppState,
    editing_state: &mut EditingState,
) {
    ui.vertical(|ui| {
        let cell_id = format!("property-{}", property.id.0);
        let weight_cell_id = format!("property-{}-weight", property.id.0);
        
        // Property name
        if editing_state.editing_cell.as_ref() == Some(&cell_id) {
            let mut text = editing_state.edit_value.clone();
            let response = ui.text_edit_singleline(&mut text);
            editing_state.edit_value = text;
            
            if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if !editing_state.edit_value.trim().is_empty() {
                    app_state.update_property(
                        &property.id,
                        editing_state.edit_value.trim().to_string(),
                        property.weight,
                        property.higher_is_better,
                    );
                }
                editing_state.editing_cell = None;
                editing_state.edit_value.clear();
            }
        } else {
            if ui.button(&property.name).clicked() {
                editing_state.editing_cell = Some(cell_id);
                editing_state.edit_value = property.name.clone();
            }
        }
        
        // Weight and direction
        ui.horizontal(|ui| {
            ui.label("Weight:");
            
            if editing_state.editing_cell.as_ref() == Some(&weight_cell_id) {
                let mut text = editing_state.edit_value.clone();
                let response = ui.text_edit_singleline(&mut text);
                editing_state.edit_value = text;
                
                if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if let Ok(weight) = editing_state.edit_value.parse::<f64>() {
                        if weight > 0.0 {
                            app_state.update_property(
                                &property.id,
                                property.name.clone(),
                                weight,
                                property.higher_is_better,
                            );
                        }
                    }
                    editing_state.editing_cell = None;
                    editing_state.edit_value.clear();
                }
            } else {
                if ui.button(format!("{:.1}", property.weight)).clicked() {
                    editing_state.editing_cell = Some(weight_cell_id);
                    editing_state.edit_value = property.weight.to_string();
                }
            }
            
            let direction_text = if property.higher_is_better { "↑" } else { "↓" };
            if ui.button(direction_text).clicked() {
                app_state.update_property(
                    &property.id,
                    property.name.clone(),
                    property.weight,
                    !property.higher_is_better,
                );
            }
        });
        
        if ui.small_button("Delete").clicked() {
            app_state.delete_property(&property.id);
        }
    });
}

fn render_contestant_row(
    ui: &mut egui::Ui,
    contestant: &crate::data::Contestant,
    app_state: &mut AppState,
    editing_state: &mut EditingState,
) {
    let name_cell_id = format!("contestant-{}", contestant.id.0);
    
    // Contestant name
    if editing_state.editing_cell.as_ref() == Some(&name_cell_id) {
        let mut text = editing_state.edit_value.clone();
        let response = ui.text_edit_singleline(&mut text);
        editing_state.edit_value = text;
        
        if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            if !editing_state.edit_value.trim().is_empty() {
                app_state.update_contestant_name(&contestant.id, editing_state.edit_value.trim().to_string());
            }
            editing_state.editing_cell = None;
            editing_state.edit_value.clear();
        }
    } else {
        if ui.button(&contestant.name).clicked() {
            editing_state.editing_cell = Some(name_cell_id);
            editing_state.edit_value = contestant.name.clone();
        }
    }
    
    // Property values
    for property in &app_state.properties.clone() {
        let value_cell_id = format!("{}-{}", contestant.id.0, property.id.0);
        let value = contestant.values.get(&property.id).copied().unwrap_or(0.0);
        
        if editing_state.editing_cell.as_ref() == Some(&value_cell_id) {
            let mut text = editing_state.edit_value.clone();
            let response = ui.text_edit_singleline(&mut text);
            editing_state.edit_value = text;
            
            if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Ok(new_value) = editing_state.edit_value.parse::<f64>() {
                    app_state.update_contestant_value(&contestant.id, &property.id, new_value);
                }
                editing_state.editing_cell = None;
                editing_state.edit_value.clear();
            }
        } else {
            if ui.button(format!("{:.1}", value)).clicked() {
                editing_state.editing_cell = Some(value_cell_id);
                editing_state.edit_value = value.to_string();
            }
        }
    }
    
    // Actions
    if ui.small_button("Delete").clicked() {
        app_state.delete_contestant(&contestant.id);
    }
    
    ui.end_row();
}

fn render_results_panel(ui: &mut egui::Ui, app_state: &AppState) {
    ui.vertical(|ui| {
        ui.heading("Scoring Results");
        
        ui.separator();
        
        let results = app_state.calculate_scores();
        
        if results.is_empty() {
            ui.label("Add contestants and properties to see scoring results");
        } else {
            egui::ScrollArea::vertical().id_source("results_panel").show(ui, |ui| {
                for (index, result) in results.iter().enumerate() {
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(38, 38, 38))
                        .rounding(4.0)
                        .inner_margin(8.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("#{}", index + 1));
                                ui.label(&result.name);
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.strong(format!("{:.1}%", result.score));
                                });
                            });
                            
                            if !result.best_properties.is_empty() {
                                ui.label(format!("Best at: {}", result.best_properties.join(", ")));
                            }
                        });
                    
                    ui.add_space(4.0);
                }
            });
        }
    });
}

// File I/O functions with real file dialogs
fn export_data(app_state: &AppState) {
    match app_state.export_format {
        ExportFormat::Json => {
            if let Ok(json) = app_state.export_data_as_json() {
                if let Some(file_path) = rfd::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_file_name("contestant-data.json")
                    .save_file()
                {
                    if let Err(e) = std::fs::write(&file_path, json) {
                        println!("Failed to write file: {}", e);
                    } else {
                        println!("Data exported successfully to: {:?}", file_path);
                    }
                }
            }
        }
        ExportFormat::Csv => {
            let (contestants_csv, properties_csv) = app_state.export_data_as_csv();
            
            // Save contestants CSV
            if let Some(file_path) = rfd::FileDialog::new()
                .add_filter("CSV", &["csv"])
                .set_file_name("contestant-data.csv")
                .save_file()
            {
                if let Err(e) = std::fs::write(&file_path, contestants_csv) {
                    println!("Failed to write contestants file: {}", e);
                } else {
                    println!("Contestants data exported to: {:?}", file_path);
                }
            }
            
            // Save properties CSV
            if let Some(file_path) = rfd::FileDialog::new()
                .add_filter("CSV", &["csv"])
                .set_file_name("properties-data.csv")
                .save_file()
            {
                if let Err(e) = std::fs::write(&file_path, properties_csv) {
                    println!("Failed to write properties file: {}", e);
                } else {
                    println!("Properties data exported to: {:?}", file_path);
                }
            }
        }
    }
}

fn export_results(app_state: &AppState) {
    match app_state.export_format {
        ExportFormat::Json => {
            if let Ok(json) = app_state.export_results_as_json() {
                if let Some(file_path) = rfd::FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .set_file_name("scoring-results.json")
                    .save_file()
                {
                    if let Err(e) = std::fs::write(&file_path, json) {
                        println!("Failed to write file: {}", e);
                    } else {
                        println!("Results exported successfully to: {:?}", file_path);
                    }
                }
            }
        }
        ExportFormat::Csv => {
            let csv = app_state.export_results_as_csv();
            if let Some(file_path) = rfd::FileDialog::new()
                .add_filter("CSV", &["csv"])
                .set_file_name("scoring-results.csv")
                .save_file()
            {
                if let Err(e) = std::fs::write(&file_path, csv) {
                    println!("Failed to write file: {}", e);
                } else {
                    println!("Results exported successfully to: {:?}", file_path);
                }
            }
        }
    }
}

fn import_data(app_state: &mut AppState) {
    if let Some(file_path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file()
    {
        match std::fs::read_to_string(&file_path) {
            Ok(file_content) => {
                match app_state.import_data(&file_content) {
                    Ok(()) => {
                        println!("Data imported successfully from: {:?}", file_path);
                    }
                    Err(e) => {
                        println!("Failed to import data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to read file: {}", e);
            }
        }
    }
}