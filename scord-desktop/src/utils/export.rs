use crate::models::{Contestant, Property, ScoreResult};
use anyhow::Result;
use serde_json;
use std::collections::HashMap;

#[derive(serde::Serialize)]
pub struct ExportData {
    pub contestants: Vec<ExportContestant>,
    pub properties: Vec<ExportProperty>,
}

#[derive(serde::Serialize)]
pub struct ExportContestant {
    pub name: String,
    pub values: HashMap<String, f64>,
}

#[derive(serde::Serialize)]
pub struct ExportProperty {
    pub name: String,
    pub weight: f64,
    #[serde(rename = "higherIsBetter")]
    pub higher_is_better: bool,
}

#[derive(serde::Serialize)]
pub struct ExportResults {
    pub rank: usize,
    pub name: String,
    pub score: f64,
    #[serde(rename = "bestProperties")]
    pub best_properties: Vec<String>,
}

pub fn export_data_to_json(contestants: &[Contestant], properties: &[Property]) -> Result<String> {
    let export_contestants: Vec<ExportContestant> = contestants
        .iter()
        .map(|c| {
            let values = c.values
                .iter()
                .filter_map(|(prop_id, value)| {
                    properties
                        .iter()
                        .find(|p| &p.id == prop_id)
                        .map(|p| (p.name.clone(), *value))
                })
                .collect();
            
            ExportContestant {
                name: c.name.clone(),
                values,
            }
        })
        .collect();

    let export_properties: Vec<ExportProperty> = properties
        .iter()
        .map(|p| ExportProperty {
            name: p.name.clone(),
            weight: p.weight,
            higher_is_better: p.higher_is_better,
        })
        .collect();

    let export_data = ExportData {
        contestants: export_contestants,
        properties: export_properties,
    };

    Ok(serde_json::to_string_pretty(&export_data)?)
}

pub fn export_results_to_json(results: &[ScoreResult]) -> Result<String> {
    let export_results: Vec<ExportResults> = results
        .iter()
        .enumerate()
        .map(|(index, result)| ExportResults {
            rank: index + 1,
            name: result.name.clone(),
            score: (result.score * 10.0).round() / 10.0, // Round to 1 decimal
            best_properties: result.best_properties.clone(),
        })
        .collect();

    Ok(serde_json::to_string_pretty(&export_results)?)
}

pub fn export_data_to_csv(contestants: &[Contestant], properties: &[Property]) -> Result<String> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    
    // Write header
    let mut headers = vec!["Name".to_string()];
    for property in properties {
        headers.push(property.name.clone());
    }
    wtr.write_record(&headers)?;
    
    // Write data rows
    for contestant in contestants {
        let mut row = vec![contestant.name.clone()];
        for property in properties {
            let value = contestant.get_value(&property.id);
            row.push(format!("{:.1}", value));
        }
        wtr.write_record(&row)?;
    }
    
    let data = wtr.into_inner()?;
    Ok(String::from_utf8(data)?)
}

pub fn export_results_to_csv(results: &[ScoreResult]) -> Result<String> {
    let mut wtr = csv::Writer::from_writer(vec![]);
    
    // Write header
    wtr.write_record(&["Rank", "Name", "Score", "Best Properties"])?;
    
    // Write data rows
    for (index, result) in results.iter().enumerate() {
        let best_props = result.best_properties.join("; ");
        wtr.write_record(&[
            (index + 1).to_string(),
            result.name.clone(),
            format!("{:.1}", result.score),
            best_props,
        ])?;
    }
    
    let data = wtr.into_inner()?;
    Ok(String::from_utf8(data)?)
}

#[derive(serde::Deserialize)]
pub struct ImportData {
    pub contestants: Vec<ImportContestant>,
    pub properties: Vec<ImportProperty>,
}

#[derive(serde::Deserialize)]
pub struct ImportContestant {
    pub name: String,
    pub values: HashMap<String, f64>, // property name -> value
}

#[derive(serde::Deserialize)]
pub struct ImportProperty {
    pub name: String,
    pub weight: f64,
    #[serde(rename = "higherIsBetter")]
    pub higher_is_better: bool,
}

pub fn import_data_from_json(json: &str) -> Result<(Vec<Contestant>, Vec<Property>)> {
    let import_data: ImportData = serde_json::from_str(json)?;
    
    // Create properties first to get their IDs
    let mut properties = Vec::new();
    let mut property_name_to_id = HashMap::new();
    
    for (index, import_prop) in import_data.properties.iter().enumerate() {
        let property = Property::new(
            import_prop.name.clone(),
            import_prop.weight,
            import_prop.higher_is_better,
            index,
        );
        property_name_to_id.insert(import_prop.name.clone(), property.id);
        properties.push(property);
    }
    
    // Create contestants with mapped property IDs
    let mut contestants = Vec::new();
    for import_contestant in import_data.contestants {
        let mut contestant = Contestant::new(import_contestant.name);
        
        for (prop_name, value) in import_contestant.values {
            if let Some(&prop_id) = property_name_to_id.get(&prop_name) {
                contestant.set_value(prop_id, value);
            }
        }
        
        contestants.push(contestant);
    }
    
    Ok((contestants, properties))
}