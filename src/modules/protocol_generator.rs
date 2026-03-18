use crate::data_models::*;

pub struct ProtocolGenerator;

impl ProtocolGenerator {
    pub fn generate_protocol(matrix: &DropletMatrix, config: &NucleraCartridgeConfig) -> ExperimentProtocol {
        let mut steps = Vec::new();

        steps.push(ProtocolStep {
            step_number: 1,
            description: "Load reagents into Nuclera cartridge wells".to_string(),
            duration: Some(10.0),
            details: vec![
                format!("Oxidant solutions in Row A (concentrations: 0-{} µM)",
                    matrix.droplets.iter().map(|d| d.oxidant_concentration).fold(0.0_f64, |a, b| a.max(b)) as u32),
                "Antioxidant samples in Row B".to_string(),
                "ROS fluorescent probe in Row C".to_string(),
            ],
        });

        steps.push(ProtocolStep {
            step_number: 2,
            description: "Initialize droplet routing program".to_string(),
            duration: Some(2.0),
            details: vec![
                "Load cartridge configuration file".to_string(),
                format!("Configuration ID: {}", config.cartridge_id),
                format!("Total droplets: {}", matrix.metadata.total_droplets),
            ],
        });

        steps.push(ProtocolStep {
            step_number: 3,
            description: "Generate droplets according to experiment matrix".to_string(),
            duration: Some(matrix.metadata.total_droplets as f64 * 0.5),
            details: vec![
                format!("Generate {} droplets sequentially", matrix.metadata.total_droplets),
                "Mix reagents in 1:1:1 ratio".to_string(),
                "5 mixing cycles per droplet".to_string(),
                "Target volume: 5 nL per droplet".to_string(),
            ],
        });

        let max_incubation = matrix.droplets.iter()
            .map(|d| d.exposure_time)
            .fold(0.0_f64, f64::max);

        steps.push(ProtocolStep {
            step_number: 4,
            description: "Incubate droplets for exposure time".to_string(),
            duration: Some(max_incubation),
            details: vec![
                format!("Incubation times range from {:.0} to {:.0} minutes",
                    matrix.droplets.iter().map(|d| d.exposure_time).fold(f64::INFINITY, f64::min),
                    max_incubation),
                "Temperature: 37°C".to_string(),
                "Monitor droplet stability".to_string(),
            ],
        });

        steps.push(ProtocolStep {
            step_number: 5,
            description: "Measure fluorescence output".to_string(),
            duration: Some(2.0),
            details: vec![
                format!("Detection wavelength: {} nm", config.readout_step.wavelength.unwrap_or(520.0)),
                format!("Exposure time: {} s", config.readout_step.exposure_time),
                "Record droplet-level fluorescence intensity".to_string(),
            ],
        });

        steps.push(ProtocolStep {
            step_number: 6,
            description: "Export droplet-level data".to_string(),
            duration: Some(1.0),
            details: vec![
                "Export fluorescence data to CSV".to_string(),
                "Save experiment metadata".to_string(),
                "Archive configuration files".to_string(),
            ],
        });

        ExperimentProtocol {
            experiment_id: matrix.metadata.experiment_id.clone(),
            steps,
            required_reagents: vec![
                "Hydrogen peroxide solutions (various concentrations)".to_string(),
                "Superoxide Dismutase 3 (SOD3)".to_string(),
                "Catalase".to_string(),
                "PBS buffer".to_string(),
                "ROS fluorescent probe (e.g., H2DCFDA)".to_string(),
            ],
            equipment: vec![
                "Nuclera eProtein Discovery System".to_string(),
                "Digital microfluidics cartridge".to_string(),
                "Fluorescence detector".to_string(),
                "Temperature controller".to_string(),
            ],
        }
    }

    pub fn export_to_markdown(protocol: &ExperimentProtocol) -> String {
        let mut output = String::new();

        output.push_str(&format!("# Experiment Protocol: {}\n\n", protocol.experiment_id));
        output.push_str(&format!("Generated: {}\n\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")));

        output.push_str("## Required Equipment\n\n");
        for equipment in &protocol.equipment {
            output.push_str(&format!("- {}\n", equipment));
        }
        output.push_str("\n");

        output.push_str("## Required Reagents\n\n");
        for reagent in &protocol.required_reagents {
            output.push_str(&format!("- {}\n", reagent));
        }
        output.push_str("\n");

        output.push_str("## Procedure\n\n");
        for step in &protocol.steps {
            output.push_str(&format!("### Step {}: {}\n\n", step.step_number, step.description));
            if let Some(duration) = step.duration {
                output.push_str(&format!("**Estimated duration:** {:.1} minutes\n\n", duration));
            }
            for detail in &step.details {
                output.push_str(&format!("- {}\n", detail));
            }
            output.push_str("\n");
        }

        output
    }
}
