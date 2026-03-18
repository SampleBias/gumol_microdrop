use crate::data_models::*;

pub struct NucleraGenerator;

impl NucleraGenerator {
    pub fn generate_config(matrix: &DropletMatrix) -> NucleraCartridgeConfig {
        let droplets: Vec<NucleraDropletConfig> = matrix
            .droplets
            .iter()
            .enumerate()
            .map(|(index, condition)| {
                let _row = index / 12;
                let col = index % 12;

                NucleraDropletConfig {
                    droplet_id: condition.droplet_id.clone(),
                    reagent_a: ReagentConfig {
                        name: "Oxidant Solution".to_string(),
                        type_name: "H2O2".to_string(),
                        concentration: condition.oxidant_concentration,
                        well_location: Self::get_well_location('A', col),
                    },
                    reagent_b: ReagentConfig {
                        name: "Antioxidant Sample".to_string(),
                        type_name: condition.antioxidant.clone(),
                        concentration: condition.antioxidant_concentration,
                        well_location: Self::get_well_location('B', col),
                    },
                    reagent_c: ReagentConfig {
                        name: "ROS Probe".to_string(),
                        type_name: "Fluorescent".to_string(),
                        concentration: 10.0,
                        well_location: Self::get_well_location('C', col),
                    },
                    ratio: vec![1, 1, 1],
                    volume: 5.0, // nL scale
                    mixing_cycles: 5,
                    incubation_time: condition.exposure_time * 60.0, // Convert to seconds
                }
            })
            .collect();

        let generation_order: Vec<String> = droplets.iter().map(|d| d.droplet_id.clone()).collect();

        NucleraCartridgeConfig {
            cartridge_id: format!("CARTRIDGE_{}", matrix.metadata.experiment_id),
            droplets,
            generation_order,
            readout_step: ReadoutConfig {
                method: "fluorescence".to_string(),
                wavelength: Some(520.0),
                exposure_time: 0.1,
            },
        }
    }

    fn get_well_location(row: char, col: usize) -> String {
        format!("{}{:02}", row, col + 1)
    }

    pub fn export_to_yaml(config: &NucleraCartridgeConfig) -> anyhow::Result<String> {
        let yaml = serde_yaml::to_string(config)?;
        Ok(yaml)
    }
}
