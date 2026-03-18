use crate::data_models::*;
use std::collections::HashMap;

pub struct ExperimentDesigner;

impl ExperimentDesigner {
    pub fn generate_matrix(
        experimental_ranges: &HashMap<String, Vec<f64>>,
        max_droplets: usize,
    ) -> DropletMatrix {
        let oxidant_levels = experimental_ranges
            .get("oxidant_concentration")
            .cloned()
            .unwrap_or_default();

        let exposure_times = experimental_ranges
            .get("exposure_time")
            .cloned()
            .unwrap_or_default();

        let antioxidants = vec!["control".to_string(), "SOD3".to_string(), "Catalase".to_string()];

        let mut droplets = Vec::new();
        let mut droplet_index = 1;

        let grid_size = (max_droplets as f64).sqrt().ceil() as usize;
        let grid_cols = grid_size;
        let grid_rows = ((max_droplets as f64) / grid_cols as f64).ceil() as usize;

        for (ox_idx, oxidant) in oxidant_levels.iter().enumerate() {
            for (_ant_idx, antioxidant) in antioxidants.iter().enumerate() {
                let time_idx = ox_idx % exposure_times.len();
                let exposure_time = exposure_times[time_idx];

                if droplets.len() >= max_droplets {
                    break;
                }

                let droplet = DropletCondition {
                    droplet_id: format!("D{}", droplet_index),
                    oxidant_concentration: *oxidant,
                    oxidant_type: "H2O2".to_string(),
                    antioxidant: antioxidant.clone(),
                    antioxidant_concentration: if *antioxidant != "control" { 50.0 } else { 0.0 },
                    exposure_time,
                    buffer_type: "PBS".to_string(),
                };

                droplets.push(droplet);
                droplet_index += 1;
            }
        }

        let total_droplets = droplets.len();

        DropletMatrix {
            droplets,
            metadata: MatrixMetadata {
                experiment_id: format!("EXP_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S")),
                created_at: chrono::Utc::now().to_rfc3339(),
                total_droplets,
                grid_rows,
                grid_cols,
            },
        }
    }

    pub fn optimize_for_cartridge(matrix: DropletMatrix, cartridge_capacity: usize) -> DropletMatrix {
        let droplets: Vec<_> = matrix.droplets.into_iter()
            .take(cartridge_capacity)
            .collect();

        let metadata = MatrixMetadata {
            experiment_id: matrix.metadata.experiment_id,
            created_at: matrix.metadata.created_at,
            total_droplets: droplets.len(),
            grid_rows: ((droplets.len() as f64) / 12.0).ceil() as usize,
            grid_cols: 12,
        };

        DropletMatrix {
            droplets,
            metadata,
        }
    }
}
