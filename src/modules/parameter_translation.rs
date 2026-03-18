use crate::data_models::*;
use std::collections::HashMap;

pub struct ParameterTranslationEngine;

impl ParameterTranslationEngine {
    pub fn translate_simulation_to_experimental(
        feature_vector: &SimulationFeatureVector,
    ) -> HashMap<String, Vec<f64>> {
        let mut ranges = HashMap::new();

        // Oxidant concentration range (µM)
        let base_concentration = (feature_vector.mean_superoxide_density * 100.0).floor();
        ranges.insert(
            "oxidant_concentration".to_string(),
            vec![
                0.0,
                base_concentration.max(10.0),
                (base_concentration * 5.0).min(50.0),
                (base_concentration * 10.0).min(100.0),
                (base_concentration * 25.0).min(250.0),
            ],
        );

        // Exposure time range (minutes)
        ranges.insert(
            "exposure_time".to_string(),
            vec![5.0, 10.0, 30.0, 60.0],
        );

        // Antioxidant dose
        ranges.insert(
            "antioxidant_dose".to_string(),
            vec![0.0, 10.0, 50.0, 100.0],
        );

        ranges
    }

    pub fn map_parameter_to_lab(param_name: &str, value: f64) -> String {
        match param_name {
            "superoxide_density" => format!("Oxidant: {:.1} µM", value),
            "diffusion_coefficient" => format!("Mix ratio: {:.2}", value),
            "reaction_rate" => format!("Exposure: {:.0} min", value),
            "antioxidant_neutralization" => format!("Antioxidant: {:.1} U/mL", value),
            "damage_threshold" => format!("Reporter intensity: {:.2}", value),
            _ => format!("{}: {:.2}", param_name, value),
        }
    }
}
