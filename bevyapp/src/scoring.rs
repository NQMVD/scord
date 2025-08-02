use crate::data::{AppState, ScoreResult, ContestantId, PropertyId};
use std::collections::{HashMap, HashSet};

impl AppState {
    pub fn calculate_scores(&self) -> Vec<ScoreResult> {
        if self.contestants.is_empty() || self.properties.is_empty() {
            return Vec::new();
        }

        // Find who has the best value for each property
        let mut best_in_property: HashMap<PropertyId, HashSet<ContestantId>> = HashMap::new();

        for property in &self.properties {
            let all_values: Vec<(ContestantId, f64)> = self.contestants
                .iter()
                .map(|c| (c.id.clone(), c.values.get(&property.id).copied().unwrap_or(0.0)))
                .collect();

            if all_values.is_empty() {
                continue;
            }

            let best_value = if property.higher_is_better {
                all_values.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max)
            } else {
                all_values.iter().map(|(_, v)| *v).fold(f64::INFINITY, f64::min)
            };

            let best_contestants: HashSet<ContestantId> = all_values
                .into_iter()
                .filter(|(_, v)| (*v - best_value).abs() < f64::EPSILON)
                .map(|(id, _)| id)
                .collect();

            best_in_property.insert(property.id.clone(), best_contestants);
        }

        // Calculate normalized scores for each contestant
        let mut results = Vec::new();

        for contestant in &self.contestants {
            let mut total_score = 0.0;
            let mut total_weight = 0.0;
            let mut best_properties = Vec::new();

            for property in &self.properties {
                let value = contestant.values.get(&property.id).copied().unwrap_or(0.0);

                // Check if this contestant has the best value for this property
                // Only include if they're the only one with the best value (no ties)
                if let Some(best_contestants) = best_in_property.get(&property.id) {
                    if best_contestants.contains(&contestant.id) && best_contestants.len() == 1 {
                        best_properties.push(property.name.clone());
                    }
                }

                // Get all values for this property to find min/max
                let all_values: Vec<f64> = self.contestants
                    .iter()
                    .map(|c| c.values.get(&property.id).copied().unwrap_or(0.0))
                    .filter(|v| !v.is_nan())
                    .collect();

                if all_values.is_empty() {
                    continue;
                }

                let min_value = all_values.iter().copied().fold(f64::INFINITY, f64::min);
                let max_value = all_values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
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
                contestant_id: contestant.id.clone(),
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