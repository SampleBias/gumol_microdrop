use crate::data_models::*;
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use std::collections::HashMap;

pub struct SimulationImporter;

impl SimulationImporter {
    pub fn import_from_json(path: &Path) -> Result<GumolSimulation> {
        let content = fs::read_to_string(path)
            .context("Failed to read simulation file")?;

        let simulation: GumolSimulation = serde_json::from_str(&content)
            .context("Failed to parse JSON")?;

        Ok(simulation)
    }

    pub fn import_from_csv(path: &Path) -> Result<GumolSimulation> {
        // Parse CSV format
        let mut reader = csv::Reader::from_path(path)?;

        // This would need specific CSV format parsing
        // For now, return a basic structure
        Ok(GumolSimulation {
            simulation_id: "csv_import".to_string(),
            radical_species: vec!["O2-".to_string(), "H2O2".to_string(), "OH".to_string()],
            time_series: Vec::new(),
            diffusion_constants: HashMap::new(),
            reaction_rates: HashMap::new(),
            damage_threshold: 0.67,
            temperature: 298.15,
        })
    }

    pub fn extract_feature_vector(simulation: &GumolSimulation) -> SimulationFeatureVector {
        let mut total_superoxide: f64 = 0.0;
        let mut peak_h2o2: f64 = 0.0;
        let mut total_diffusion: f64 = 0.0;
        let mut total_reaction: f64 = 0.0;
        let mut total_antioxidant: f64 = 0.0;

        for time_point in &simulation.time_series {
            if let Some(o2_density) = time_point.radical_density.get("O2-") {
                total_superoxide += o2_density;
            }
            if let Some(h2o2_density) = time_point.radical_density.get("H2O2") {
                peak_h2o2 = peak_h2o2.max(*h2o2_density);
            }
            total_antioxidant += time_point.antioxidant_activity;
        }

        let count = simulation.time_series.len() as f64;
        let avg_diffusion = simulation.diffusion_constants.values().sum::<f64>()
            / simulation.diffusion_constants.len().max(1) as f64;
        let avg_reaction = simulation.reaction_rates.values().sum::<f64>()
            / simulation.reaction_rates.len().max(1) as f64;

        SimulationFeatureVector {
            mean_superoxide_density: if count > 0.0 { total_superoxide / count } else { 0.0 },
            peak_hydrogen_peroxide: peak_h2o2,
            avg_diffusion_rate: avg_diffusion,
            reaction_velocity: avg_reaction,
            antioxidant_scavenging_rate: if count > 0.0 { total_antioxidant / count } else { 0.0 },
            time_to_damage_threshold: None,
        }
    }
}
