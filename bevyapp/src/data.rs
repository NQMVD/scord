use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContestantId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropertyId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contestant {
    pub id: ContestantId,
    pub name: String,
    pub values: HashMap<PropertyId, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub id: PropertyId,
    pub name: String,
    pub weight: f64,
    pub higher_is_better: bool,
}

#[derive(Debug, Clone)]
pub struct ScoreResult {
    pub contestant_id: ContestantId,
    pub name: String,
    pub score: f64,
    pub best_properties: Vec<String>,
}

#[derive(Resource, Default)]
pub struct AppState {
    pub contestants: Vec<Contestant>,
    pub properties: Vec<Property>,
    pub new_contestant_name: String,
    pub new_property_name: String,
    pub new_property_weight: f64,
    pub new_property_higher_is_better: bool,
    pub export_format: ExportFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportFormat {
    Json,
    Csv,
}

impl Default for ExportFormat {
    fn default() -> Self {
        ExportFormat::Json
    }
}

#[derive(Resource, Default)]
pub struct EditingState {
    pub editing_cell: Option<String>,
    pub edit_value: String,
}

impl AppState {
    pub fn add_contestant(&mut self, name: String) {
        let id = ContestantId(format!("contestant_{}", self.contestants.len()));
        let contestant = Contestant {
            id,
            name,
            values: HashMap::new(),
        };
        self.contestants.push(contestant);
    }

    pub fn add_property(&mut self, name: String, weight: f64, higher_is_better: bool) {
        let id = PropertyId(format!("property_{}", self.properties.len()));
        let property = Property {
            id,
            name,
            weight,
            higher_is_better,
        };
        self.properties.push(property);
    }

    pub fn delete_contestant(&mut self, contestant_id: &ContestantId) {
        self.contestants.retain(|c| &c.id != contestant_id);
    }

    pub fn delete_property(&mut self, property_id: &PropertyId) {
        self.properties.retain(|p| &p.id != property_id);
        // Remove values for this property from all contestants
        for contestant in &mut self.contestants {
            contestant.values.remove(property_id);
        }
    }

    pub fn update_contestant_name(&mut self, contestant_id: &ContestantId, name: String) {
        if let Some(contestant) = self.contestants.iter_mut().find(|c| &c.id == contestant_id) {
            contestant.name = name;
        }
    }

    pub fn update_property(&mut self, property_id: &PropertyId, name: String, weight: f64, higher_is_better: bool) {
        if let Some(property) = self.properties.iter_mut().find(|p| &p.id == property_id) {
            property.name = name;
            property.weight = weight;
            property.higher_is_better = higher_is_better;
        }
    }

    pub fn update_contestant_value(&mut self, contestant_id: &ContestantId, property_id: &PropertyId, value: f64) {
        if let Some(contestant) = self.contestants.iter_mut().find(|c| &c.id == contestant_id) {
            contestant.values.insert(property_id.clone(), value);
        }
    }

    pub fn get_contestant(&self, contestant_id: &ContestantId) -> Option<&Contestant> {
        self.contestants.iter().find(|c| &c.id == contestant_id)
    }

    pub fn get_property(&self, property_id: &PropertyId) -> Option<&Property> {
        self.properties.iter().find(|p| &p.id == property_id)
    }
}