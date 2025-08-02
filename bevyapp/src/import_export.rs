use crate::data::{AppState, Contestant, Property, ContestantId, PropertyId};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportData {
    pub contestants: Vec<ExportContestant>,
    pub properties: Vec<ExportProperty>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportContestant {
    pub name: String,
    pub values: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportProperty {
    pub name: String,
    pub weight: f64,
    #[serde(rename = "higherIsBetter")]
    pub higher_is_better: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportResult {
    pub rank: usize,
    pub name: String,
    pub score: f64,
    #[serde(rename = "bestProperties")]
    pub best_properties: Vec<String>,
}

impl AppState {
    pub fn export_data(&self) -> ExportData {
        let contestants = self.contestants
            .iter()
            .map(|c| ExportContestant {
                name: c.name.clone(),
                values: c.values
                    .iter()
                    .map(|(prop_id, value)| {
                        let prop_name = self.properties
                            .iter()
                            .find(|p| &p.id == prop_id)
                            .map(|p| p.name.clone())
                            .unwrap_or_else(|| prop_id.0.clone());
                        (prop_name, *value)
                    })
                    .collect(),
            })
            .collect();

        let properties = self.properties
            .iter()
            .map(|p| ExportProperty {
                name: p.name.clone(),
                weight: p.weight,
                higher_is_better: p.higher_is_better,
            })
            .collect();

        ExportData {
            contestants,
            properties,
        }
    }

    pub fn export_data_as_json(&self) -> Result<String, serde_json::Error> {
        let data = self.export_data();
        serde_json::to_string_pretty(&data)
    }

    pub fn export_results_as_json(&self) -> Result<String, serde_json::Error> {
        let score_results = self.calculate_scores();
        let results: Vec<ExportResult> = score_results
            .into_iter()
            .enumerate()
            .map(|(index, result)| ExportResult {
                rank: index + 1,
                name: result.name,
                score: (result.score * 10.0).round() / 10.0, // Round to 1 decimal
                best_properties: result.best_properties,
            })
            .collect();

        serde_json::to_string_pretty(&results)
    }

    pub fn export_data_as_csv(&self) -> (String, String) {
        let export_data = self.export_data();
        
        // Export contestants CSV
        let mut contestants_csv = String::new();
        if !export_data.contestants.is_empty() {
            // Get all unique property names for headers
            let mut all_property_names: std::collections::HashSet<String> = std::collections::HashSet::new();
            for contestant in &export_data.contestants {
                for prop_name in contestant.values.keys() {
                    all_property_names.insert(prop_name.clone());
                }
            }
            let mut property_names: Vec<String> = all_property_names.into_iter().collect();
            property_names.sort();

            // Write headers
            contestants_csv.push_str("name");
            for prop_name in &property_names {
                contestants_csv.push(',');
                contestants_csv.push_str(prop_name);
            }
            contestants_csv.push('\n');

            // Write data
            for contestant in &export_data.contestants {
                contestants_csv.push_str(&escape_csv_field(&contestant.name));
                for prop_name in &property_names {
                    contestants_csv.push(',');
                    if let Some(value) = contestant.values.get(prop_name) {
                        contestants_csv.push_str(&value.to_string());
                    }
                }
                contestants_csv.push('\n');
            }
        }

        // Export properties CSV
        let mut properties_csv = String::new();
        if !export_data.properties.is_empty() {
            properties_csv.push_str("name,weight,higherIsBetter\n");
            for property in &export_data.properties {
                properties_csv.push_str(&escape_csv_field(&property.name));
                properties_csv.push(',');
                properties_csv.push_str(&property.weight.to_string());
                properties_csv.push(',');
                properties_csv.push_str(&property.higher_is_better.to_string());
                properties_csv.push('\n');
            }
        }

        (contestants_csv, properties_csv)
    }

    pub fn export_results_as_csv(&self) -> String {
        let score_results = self.calculate_scores();
        let mut csv = String::new();
        
        if !score_results.is_empty() {
            csv.push_str("rank,name,score,bestProperties\n");
            for (index, result) in score_results.iter().enumerate() {
                csv.push_str(&(index + 1).to_string());
                csv.push(',');
                csv.push_str(&escape_csv_field(&result.name));
                csv.push(',');
                csv.push_str(&format!("{:.1}", result.score));
                csv.push(',');
                csv.push_str(&escape_csv_field(&result.best_properties.join("; ")));
                csv.push('\n');
            }
        }

        csv
    }

    pub fn import_data(&mut self, json_data: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data: ExportData = serde_json::from_str(json_data)?;
        
        // Clear existing data
        self.contestants.clear();
        self.properties.clear();

        // Import properties first
        for (index, export_prop) in data.properties.into_iter().enumerate() {
            let property = Property {
                id: PropertyId(format!("property_{}", index)),
                name: export_prop.name,
                weight: export_prop.weight,
                higher_is_better: export_prop.higher_is_better,
            };
            self.properties.push(property);
        }

        // Create property name to ID mapping
        let prop_name_to_id: HashMap<String, PropertyId> = self.properties
            .iter()
            .map(|p| (p.name.clone(), p.id.clone()))
            .collect();

        // Import contestants
        for (index, export_contestant) in data.contestants.into_iter().enumerate() {
            let mut values = HashMap::new();
            for (prop_name, value) in export_contestant.values {
                if let Some(prop_id) = prop_name_to_id.get(&prop_name) {
                    values.insert(prop_id.clone(), value);
                }
            }

            let contestant = Contestant {
                id: ContestantId(format!("contestant_{}", index)),
                name: export_contestant.name,
                values,
            };
            self.contestants.push(contestant);
        }

        Ok(())
    }
}

fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}