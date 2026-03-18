use crate::data_models::*;
use std::collections::HashMap;

pub struct ExperimentDesigner;

impl ExperimentDesigner {
    pub fn generate_matrix_from_config(
        oxidant_levels: &[f64],
        antioxidants: &[String],
        exposure_times: &[f64],
        max_droplets: usize,
        oxidant_type: &str,
    ) -> DropletMatrix {
        let mut droplets = Vec::new();
        let mut idx = 1;

        for &conc in oxidant_levels {
            for antioxidant in antioxidants {
                for &time in exposure_times {
                    if droplets.len() >= max_droplets {
                        break;
                    }
                    let is_control = antioxidant.eq_ignore_ascii_case("control");
                    droplets.push(DropletCondition {
                        droplet_id: format!("D{}", idx),
                        oxidant_concentration: conc,
                        oxidant_type: oxidant_type.to_string(),
                        antioxidant: antioxidant.clone(),
                        antioxidant_concentration: if is_control { 0.0 } else { 50.0 },
                        exposure_time: time,
                        buffer_type: "PBS".to_string(),
                    });
                    idx += 1;
                }
            }
        }

        let total = droplets.len();
        let grid_cols = 12;
        let grid_rows = ((total as f64) / grid_cols as f64).ceil().max(1.0) as usize;

        DropletMatrix {
            droplets,
            metadata: MatrixMetadata {
                experiment_id: format!("EXP_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S")),
                created_at: chrono::Utc::now().to_rfc3339(),
                total_droplets: total,
                grid_rows,
                grid_cols,
            },
        }
    }

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
        let antioxidants = vec!["Control".to_string(), "SOD3".to_string(), "Catalase".to_string()];

        Self::generate_matrix_from_config(
            &oxidant_levels,
            &antioxidants,
            &exposure_times,
            max_droplets,
            "H2O2",
        )
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

        DropletMatrix { droplets, metadata }
    }
}
