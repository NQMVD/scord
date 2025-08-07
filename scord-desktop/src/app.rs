use crate::models::{Contestant, Property, ScoringEngine, ScoreResult};
use crate::persistence::{Storage, AppData};
use crate::ui::{setup_custom_style, SpreadsheetView, ResultsPanel, ControlsPanel};
use crate::utils::export::{export_data_to_json, export_results_to_json, export_data_to_csv, export_results_to_csv, import_data_from_json};
use anyhow::Result;
use egui::{Context, CentralPanel, SidePanel, TopBottomPanel};
use std::fs;
use uuid::Uuid;

pub struct ScordApp {
    pub contestants: Vec<Contestant>,
    pub properties: Vec<Property>,
    pub score_results: Vec<ScoreResult>,
    storage: Storage,
    
    // UI state
    pub new_contestant_name: String,
    pub new_property_name: String,
    pub new_property_weight: f64,
    pub new_property_higher_is_better: bool,
    
    // Status
    status_message: String,
    show_status: bool,
    
    // Export format
    export_format: ExportFormat,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
}

impl Default for ScordApp {
    fn default() -> Self {
        Self {
            contestants: Vec::new(),
            properties: Vec::new(),
            score_results: Vec::new(),
            storage: Storage::new(),
            new_contestant_name: String::new(),
            new_property_name: String::new(),
            new_property_weight: 1.0,
            new_property_higher_is_better: true,
            status_message: String::new(),
            show_status: false,
            export_format: ExportFormat::Json,
        }
    }
}

impl ScordApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_style(&cc.egui_ctx);
        
        let mut app = Self::default();
        if let Err(e) = app.load_data() {
            app.set_status(&format!("Failed to load data: {}", e));
        }
        app.update_scores();
        app
    }

    fn load_data(&mut self) -> Result<()> {
        let data = self.storage.load()?;
        self.contestants = data.contestants;
        self.properties = data.properties;
        Ok(())
    }

    fn save_data(&mut self) -> Result<()> {
        let data = AppData {
            contestants: self.contestants.clone(),
            properties: self.properties.clone(),
        };
        self.storage.save(&data)
    }

    fn auto_save(&mut self) {
        if let Err(e) = self.save_data() {
            self.set_status(&format!("Auto-save failed: {}", e));
        }
    }

    fn update_scores(&mut self) {
        self.score_results = ScoringEngine::calculate_scores(&self.contestants, &self.properties);
    }

    fn set_status(&mut self, message: &str) {
        self.status_message = message.to_string();
        self.show_status = true;
    }

    pub fn add_contestant(&mut self) {
        if !self.new_contestant_name.trim().is_empty() {
            let contestant = Contestant::new(self.new_contestant_name.trim().to_string());
            self.contestants.push(contestant);
            self.new_contestant_name.clear();
            self.update_scores();
            self.auto_save();
            self.set_status("Contestant added");
        }
    }

    pub fn add_property(&mut self) {
        if !self.new_property_name.trim().is_empty() {
            let order = self.properties.len();
            let property = Property::new(
                self.new_property_name.trim().to_string(),
                self.new_property_weight,
                self.new_property_higher_is_better,
                order,
            );
            self.properties.push(property);
            self.new_property_name.clear();
            self.new_property_weight = 1.0;
            self.new_property_higher_is_better = true;
            self.update_scores();
            self.auto_save();
            self.set_status("Property added");
        }
    }

    pub fn delete_contestant(&mut self, contestant_id: Uuid) {
        self.contestants.retain(|c| c.id != contestant_id);
        self.update_scores();
        self.auto_save();
        self.set_status("Contestant deleted");
    }

    pub fn delete_property(&mut self, property_id: Uuid) {
        self.properties.retain(|p| p.id != property_id);
        // Clean up contestant values
        for contestant in &mut self.contestants {
            contestant.remove_property(&property_id);
        }
        self.update_scores();
        self.auto_save();
        self.set_status("Property deleted");
    }

    pub fn update_contestant_name(&mut self, contestant_id: Uuid, name: String) {
        if let Some(contestant) = self.contestants.iter_mut().find(|c| c.id == contestant_id) {
            contestant.name = name;
            self.update_scores();
            self.auto_save();
        }
    }

    pub fn update_contestant_value(&mut self, contestant_id: Uuid, property_id: Uuid, value: f64) {
        if let Some(contestant) = self.contestants.iter_mut().find(|c| c.id == contestant_id) {
            contestant.set_value(property_id, value);
            self.update_scores();
            self.auto_save();
        }
    }

    pub fn update_property(&mut self, property_id: Uuid, name: String, weight: f64, higher_is_better: bool) {
        if let Some(property) = self.properties.iter_mut().find(|p| p.id == property_id) {
            property.name = name;
            property.weight = weight;
            property.higher_is_better = higher_is_better;
            self.update_scores();
            self.auto_save();
        }
    }

    pub fn export_data(&mut self) {
        let (extension, filter_name, filename) = match self.export_format {
            ExportFormat::Json => ("json", "JSON", "contestant-data.json"),
            ExportFormat::Csv => ("csv", "CSV", "contestant-data.csv"),
        };
        
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(filter_name, &[extension])
            .set_file_name(filename)
            .save_file()
        {
            let result = match self.export_format {
                ExportFormat::Json => export_data_to_json(&self.contestants, &self.properties),
                ExportFormat::Csv => export_data_to_csv(&self.contestants, &self.properties),
            };
            
            match result {
                Ok(content) => {
                    if let Err(e) = fs::write(&path, content) {
                        self.set_status(&format!("Export failed: {}", e));
                    } else {
                        self.set_status("Data exported successfully");
                    }
                }
                Err(e) => self.set_status(&format!("Export failed: {}", e)),
            }
        }
    }

    pub fn export_results(&mut self) {
        let (extension, filter_name, filename) = match self.export_format {
            ExportFormat::Json => ("json", "JSON", "scoring-results.json"),
            ExportFormat::Csv => ("csv", "CSV", "scoring-results.csv"),
        };
        
        if let Some(path) = rfd::FileDialog::new()
            .add_filter(filter_name, &[extension])
            .set_file_name(filename)
            .save_file()
        {
            let result = match self.export_format {
                ExportFormat::Json => export_results_to_json(&self.score_results),
                ExportFormat::Csv => export_results_to_csv(&self.score_results),
            };
            
            match result {
                Ok(content) => {
                    if let Err(e) = fs::write(&path, content) {
                        self.set_status(&format!("Export failed: {}", e));
                    } else {
                        self.set_status("Results exported successfully");
                    }
                }
                Err(e) => self.set_status(&format!("Export failed: {}", e)),
            }
        }
    }

    pub fn import_data(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    // Try the export format first (with property names as keys)
                    match import_data_from_json(&content) {
                        Ok((contestants, properties)) => {
                            self.contestants = contestants;
                            self.properties = properties;
                            self.update_scores();
                            self.auto_save();
                            self.set_status("Data imported successfully");
                        }
                        Err(_) => {
                            // Fall back to internal format (with UUIDs)
                            match serde_json::from_str::<AppData>(&content) {
                                Ok(data) => {
                                    self.contestants = data.contestants;
                                    self.properties = data.properties;
                                    self.update_scores();
                                    self.auto_save();
                                    self.set_status("Data imported successfully");
                                }
                                Err(e) => self.set_status(&format!("Import failed: {}", e)),
                            }
                        }
                    }
                }
                Err(e) => self.set_status(&format!("Import failed: {}", e)),
            }
        }
    }
}

impl eframe::App for ScordApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Header
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("> scord");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Export Results").clicked() {
                        self.export_results();
                    }
                    if ui.button("Export Data").clicked() {
                        self.export_data();
                    }
                    if ui.button("Import").clicked() {
                        self.import_data();
                    }
                    
                    egui::ComboBox::from_label("Format")
                        .selected_text(match self.export_format {
                            ExportFormat::Json => "JSON",
                            ExportFormat::Csv => "CSV",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.export_format, ExportFormat::Json, "JSON");
                            ui.selectable_value(&mut self.export_format, ExportFormat::Csv, "CSV");
                        });
                });
            });
        });

        // Status bar
        if self.show_status {
            TopBottomPanel::bottom("status").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(&self.status_message);
                    if ui.button("×").clicked() {
                        self.show_status = false;
                    }
                });
            });
        }

        // Controls panel
        TopBottomPanel::top("controls").show(ctx, |ui| {
            ControlsPanel::show(ui, self);
        });

        // Results panel
        SidePanel::right("results")
            .resizable(true)
            .default_width(320.0)
            .show(ctx, |ui| {
                ResultsPanel::show(ui, &self.score_results);
            });

        // Main spreadsheet
        CentralPanel::default().show(ctx, |ui| {
            SpreadsheetView::show(ui, self);
        });
    }
}