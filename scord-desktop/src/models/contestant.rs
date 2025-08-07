use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contestant {
    pub id: Uuid,
    pub name: String,
    pub values: HashMap<Uuid, f64>, // property_id -> value
}

impl Contestant {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            values: HashMap::new(),
        }
    }

    pub fn set_value(&mut self, property_id: Uuid, value: f64) {
        self.values.insert(property_id, value);
    }

    pub fn get_value(&self, property_id: &Uuid) -> f64 {
        self.values.get(property_id).copied().unwrap_or(0.0)
    }

    pub fn remove_property(&mut self, property_id: &Uuid) {
        self.values.remove(property_id);
    }
}