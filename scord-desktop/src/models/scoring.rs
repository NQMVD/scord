use super::{Contestant, Property};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreResult {
    pub contestant_id: Uuid,
    pub name: String,
    pub score: f64,
    pub best_properties: Vec<String>,
}

pub struct ScoringEngine;

impl ScoringEngine {
    pub fn calculate_scores(
        contestants: &[Contestant],
        properties: &[Property],
    ) -> Vec<ScoreResult> {
        if contestants.is_empty() || properties.is_empty() {
            return Vec::new();
        }

        // Find who has the best value for each property
        let mut best_in_property: HashMap<Uuid, HashSet<Uuid>> = HashMap::new();

        for property in properties {
            let all_values: Vec<(Uuid, f64)> = contestants
                .iter()
                .map(|c| (c.id, c.get_value(&property.id)))
                .collect();

            if all_values.is_empty() {
                continue;
            }

            let best_value = if property.higher_is_better {
                all_values
                    .iter()
                    .map(|(_, value)| *value)
                    .fold(f64::NEG_INFINITY, f64::max)
            } else {
                all_values
                    .iter()
                    .map(|(_, value)| *value)
                    .fold(f64::INFINITY, f64::min)
            };

            let best_contestants: HashSet<Uuid> = all_values
                .iter()
                .filter(|(_, value)| (*value - best_value).abs() < f64::EPSILON)
                .map(|(id, _)| *id)
                .collect();

            best_in_property.insert(property.id, best_contestants);
        }

        // Calculate normalized scores for each contestant
        let mut results = Vec::new();

        for contestant in contestants {
            let mut total_score = 0.0;
            let mut total_weight = 0.0;
            let mut best_properties = Vec::new();

            for property in properties {
                let value = contestant.get_value(&property.id);

                // Check if this contestant has the best value for this property
                // Only include if they're the only one with the best value (no ties)
                if let Some(best_contestants) = best_in_property.get(&property.id) {
                    if best_contestants.contains(&contestant.id) && best_contestants.len() == 1 {
                        best_properties.push(property.name.clone());
                    }
                }

                // Get all values for this property to find min/max
                let all_values: Vec<f64> = contestants
                    .iter()
                    .map(|c| c.get_value(&property.id))
                    .filter(|v| !v.is_nan())
                    .collect();

                if all_values.is_empty() {
                    continue;
                }

                let min_value = all_values
                    .iter()
                    .fold(f64::INFINITY, |a, &b| a.min(b));
                let max_value = all_values
                    .iter()
                    .fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let range = max_value - min_value;

                let normalized_score = if range == 0.0 {
                    100.0
                } else if property.higher_is_better {
                    ((value - min_value) / range) * 100.0
                } else {
                    ((max_value - value) / range) * 100.0
                };

                let weighted_score = normalized_score * property.weight;
                total_score += weighted_score;
                total_weight += property.weight;
            }

            let final_score = if total_weight > 0.0 {
                total_score / total_weight
            } else {
                0.0
            };

            results.push(ScoreResult {
                contestant_id: contestant.id,
                name: contestant.name.clone(),
                score: final_score,
                best_properties,
            });
        }

        // Sort by score descending
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}