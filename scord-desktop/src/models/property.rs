use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub id: Uuid,
    pub name: String,
    pub weight: f64,
    pub higher_is_better: bool,
    pub order: usize,
}

impl Property {
    pub fn new(name: String, weight: f64, higher_is_better: bool, order: usize) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            weight,
            higher_is_better,
            order,
        }
    }
}