use crate::models::{Contestant, Property, ScoreResult, ScoringEngine};
use crate::persistence::AppData;
use crate::utils::export::{export_data_to_json, export_results_to_json, export_data_to_csv, export_results_to_csv, import_data_from_json};
use anyhow::Result;
use std::fs;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
}

#[derive(Debug, Clone)]
pub struct TabContent {
    pub contestants: Vec<Contestant>,
    pub properties: Vec<Property>,
    pub score_results: Vec<ScoreResult>,
    
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
    
    // Tab-specific data
    pub has_unsaved_changes: bool,
}

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: Uuid,
    pub name: String,
    pub file_path: Option<String>,
    pub content: TabContent,
}

#[derive(Debug)]
pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab_index: usize,
    next_tab_number: usize,
}

impl Default for TabContent {
    fn default() -> Self {
        Self {
            contestants: Vec::new(),
            properties: Vec::new(),
            score_results: Vec::new(),
            new_contestant_name: String::new(),
            new_property_name: String::new(),
            new_property_weight: 1.0,
            new_property_higher_is_better: true,
            status_message: String::new(),
            show_status: false,
            export_format: ExportFormat::Json,
            has_unsaved_changes: false,
        }
    }
}

impl TabContent {
    pub fn new() -> Self {
        let mut content = Self::default();
        content.update_scores();
        content
    }

    pub fn from_data(contestants: Vec<Contestant>, properties: Vec<Property>) -> Self {
        let mut content = Self::default();
        content.contestants = contestants;
        content.properties = properties;
        content.update_scores();
        content.has_unsaved_changes = false;
        content
    }

    fn auto_save(&mut self) {
        // TODO: Implement per-tab auto-save functionality
        self.has_unsaved_changes = false;
    }

    fn update_scores(&mut self) {
        self.score_results = ScoringEngine::calculate_scores(&self.contestants, &self.properties);
    }

    pub fn set_status(&mut self, message: &str) {
        self.status_message = message.to_string();
        self.show_status = true;
    }

    pub fn get_status(&self) -> (&str, bool) {
        (&self.status_message, self.show_status)
    }

    pub fn hide_status(&mut self) {
        self.show_status = false;
    }

    pub fn get_export_format(&self) -> ExportFormat {
        self.export_format
    }

    pub fn set_export_format(&mut self, format: ExportFormat) {
        self.export_format = format;
    }

    pub fn add_contestant(&mut self) {
        if !self.new_contestant_name.trim().is_empty() {
            let contestant = Contestant::new(self.new_contestant_name.trim().to_string());
            self.contestants.push(contestant);
            self.new_contestant_name.clear();
            self.update_scores();
            self.has_unsaved_changes = true;
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
            self.has_unsaved_changes = true;
            self.auto_save();
            self.set_status("Property added");
        }
    }

    pub fn delete_contestant(&mut self, contestant_id: Uuid) {
        self.contestants.retain(|c| c.id != contestant_id);
        self.update_scores();
        self.has_unsaved_changes = true;
        self.auto_save();
        self.set_status("Contestant deleted");
    }

    pub fn delete_property(&mut self, property_id: Uuid) {
        self.properties.retain(|p| p.id != property_id);
        for contestant in &mut self.contestants {
            contestant.remove_property(&property_id);
        }
        self.update_scores();
        self.has_unsaved_changes = true;
        self.auto_save();
        self.set_status("Property deleted");
    }

    pub fn update_contestant_name(&mut self, contestant_id: Uuid, name: String) {
        if let Some(contestant) = self.contestants.iter_mut().find(|c| c.id == contestant_id) {
            contestant.name = name;
            self.update_scores();
            self.has_unsaved_changes = true;
            self.auto_save();
        }
    }

    pub fn update_contestant_value(&mut self, contestant_id: Uuid, property_id: Uuid, value: f64) {
        if let Some(contestant) = self.contestants.iter_mut().find(|c| c.id == contestant_id) {
            contestant.set_value(property_id, value);
            self.update_scores();
            self.has_unsaved_changes = true;
            self.auto_save();
        }
    }

    pub fn update_property(&mut self, property_id: Uuid, name: String, weight: f64, higher_is_better: bool) {
        if let Some(property) = self.properties.iter_mut().find(|p| p.id == property_id) {
            property.name = name;
            property.weight = weight;
            property.higher_is_better = higher_is_better;
            self.update_scores();
            self.has_unsaved_changes = true;
            self.auto_save();
        }
    }

    pub fn export_data(&mut self) -> Result<()> {
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
                        return Err(e.into());
                    } else {
                        self.set_status("Data exported successfully");
                        return Ok(());
                    }
                }
                Err(e) => {
                    self.set_status(&format!("Export failed: {}", e));
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn export_results(&mut self) -> Result<()> {
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
                        return Err(e.into());
                    } else {
                        self.set_status("Results exported successfully");
                        return Ok(());
                    }
                }
                Err(e) => {
                    self.set_status(&format!("Export failed: {}", e));
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    pub fn import_data(&mut self) -> Result<()> {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match import_data_from_json(&content) {
                        Ok((contestants, properties)) => {
                            self.contestants = contestants;
                            self.properties = properties;
                            self.update_scores();
                            self.has_unsaved_changes = true;
                            self.auto_save();
                            self.set_status("Data imported successfully");
                            return Ok(());
                        }
                        Err(_) => {
                            match serde_json::from_str::<AppData>(&content) {
                                Ok(data) => {
                                    self.contestants = data.contestants;
                                    self.properties = data.properties;
                                    self.update_scores();
                                    self.has_unsaved_changes = true;
                                    self.auto_save();
                                    self.set_status("Data imported successfully");
                                    return Ok(());
                                }
                                Err(e) => {
                                    self.set_status(&format!("Import failed: {}", e));
                                    return Err(e.into());
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    self.set_status(&format!("Import failed: {}", e));
                    return Err(e.into());
                }
            }
        }
        Ok(())
    }
}

impl Tab {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            file_path: None,
            content: TabContent::new(),
        }
    }

    pub fn from_data(name: String, contestants: Vec<Contestant>, properties: Vec<Property>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            file_path: None,
            content: TabContent::from_data(contestants, properties),
        }
    }

    pub fn display_name(&self) -> String {
        if self.content.has_unsaved_changes {
            format!("{} •", self.name)
        } else {
            self.name.clone()
        }
    }
}

impl TabManager {
    pub fn new() -> Self {
        let initial_tab = Tab::new("Project 1".to_string());
        Self {
            tabs: vec![initial_tab],
            active_tab_index: 0,
            next_tab_number: 2,
        }
    }

    pub fn get_active_tab(&self) -> Option<&Tab> {
        self.tabs.get(self.active_tab_index)
    }

    pub fn get_active_tab_mut(&mut self) -> Option<&mut Tab> {
        self.tabs.get_mut(self.active_tab_index)
    }

    pub fn set_active_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab_index = index;
        }
    }

    pub fn add_tab(&mut self) -> usize {
        let tab_name = format!("Project {}", self.next_tab_number);
        let new_tab = Tab::new(tab_name);
        self.tabs.push(new_tab);
        self.next_tab_number += 1;
        let new_index = self.tabs.len() - 1;
        self.active_tab_index = new_index;
        new_index
    }

    pub fn add_tab_with_data(&mut self, name: String, contestants: Vec<Contestant>, properties: Vec<Property>) -> usize {
        let new_tab = Tab::from_data(name, contestants, properties);
        self.tabs.push(new_tab);
        let new_index = self.tabs.len() - 1;
        self.active_tab_index = new_index;
        new_index
    }

    pub fn close_tab(&mut self, index: usize) -> bool {
        if self.tabs.len() <= 1 {
            return false; // Don't close the last tab
        }

        if index < self.tabs.len() {
            self.tabs.remove(index);
            
            // Adjust active tab index
            if self.active_tab_index >= self.tabs.len() {
                self.active_tab_index = self.tabs.len() - 1;
            } else if index <= self.active_tab_index && self.active_tab_index > 0 {
                self.active_tab_index -= 1;
            }
            
            return true;
        }
        false
    }
    
    pub fn has_unsaved_changes(&self, index: usize) -> bool {
        self.tabs.get(index).map_or(false, |tab| tab.content.has_unsaved_changes)
    }

    pub fn can_close_tab(&self, _index: usize) -> bool {
        self.tabs.len() > 1
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }
}