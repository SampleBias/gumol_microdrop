use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CurrentPanel {
    #[default]
    SimulationImport,
    CardEditor,
    ParameterMapping,
    ExperimentDesign,
    NucleraConfig,
    ProtocolGenerator,
    DataViewer,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ColorMode {
    #[default]
    Oxidant,
    Antioxidant,
    ExposureTime,
    Treatment,
}

#[derive(Resource)]
pub struct ApplicationState {
    pub simulation_file: Option<PathBuf>,
    pub simulation: Option<GumolSimulation>,
    pub feature_vector: Option<SimulationFeatureVector>,
    pub parameter_ranges: HashMap<String, Vec<f64>>,

    pub card_name: String,
    pub grid_rows: usize,
    pub grid_cols: usize,
    pub oxidant_type: String,
    pub min_concentration: f64,
    pub max_concentration: f64,
    pub antioxidants: Vec<(String, bool)>,
    pub exposure_times: Vec<f64>,

    pub droplet_matrix: Option<DropletMatrix>,
    pub nuclera_config: Option<NucleraCartridgeConfig>,
    pub protocol: Option<ExperimentProtocol>,
    pub correlation_report: Option<CorrelationReport>,

    pub default_volume: f64,
    pub mixing_cycles: u32,
    pub readout_method: String,
    pub wavelength: f64,
    pub readout_exposure: f64,

    pub selected_droplet: Option<usize>,
    pub color_mode: ColorMode,
    pub status_message: String,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            simulation_file: None,
            simulation: None,
            feature_vector: None,
            parameter_ranges: HashMap::new(),
            card_name: "My Experiment Card".to_string(),
            grid_rows: 8,
            grid_cols: 12,
            oxidant_type: "H2O2".to_string(),
            min_concentration: 0.0,
            max_concentration: 250.0,
            antioxidants: vec![
                ("Control".to_string(), true),
                ("SOD3".to_string(), true),
                ("Catalase".to_string(), false),
                ("GPx".to_string(), false),
            ],
            exposure_times: vec![5.0, 10.0, 30.0, 60.0],
            droplet_matrix: None,
            nuclera_config: None,
            protocol: None,
            correlation_report: None,
            default_volume: 5.0,
            mixing_cycles: 5,
            readout_method: "Fluorescence".to_string(),
            wavelength: 520.0,
            readout_exposure: 0.1,
            selected_droplet: None,
            color_mode: ColorMode::default(),
            status_message: "Ready".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GumolSimulation {
    pub simulation_id: String,
    pub radical_species: Vec<String>,
    pub time_series: Vec<TimePoint>,
    pub diffusion_constants: HashMap<String, f64>,
    pub reaction_rates: HashMap<String, f64>,
    pub damage_threshold: f64,
    pub temperature: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePoint {
    pub time: f64,
    pub radical_density: HashMap<String, f64>,
    pub antioxidant_activity: f64,
    pub damage_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFeatureVector {
    pub mean_superoxide_density: f64,
    pub peak_hydrogen_peroxide: f64,
    pub avg_diffusion_rate: f64,
    pub reaction_velocity: f64,
    pub antioxidant_scavenging_rate: f64,
    pub time_to_damage_threshold: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropletCondition {
    pub droplet_id: String,
    pub oxidant_concentration: f64,
    pub oxidant_type: String,
    pub antioxidant: String,
    pub antioxidant_concentration: f64,
    pub exposure_time: f64,
    pub buffer_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropletMatrix {
    pub droplets: Vec<DropletCondition>,
    pub metadata: MatrixMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixMetadata {
    pub experiment_id: String,
    pub created_at: String,
    pub total_droplets: usize,
    pub grid_rows: usize,
    pub grid_cols: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NucleraDropletConfig {
    pub droplet_id: String,
    pub reagent_a: ReagentConfig,
    pub reagent_b: ReagentConfig,
    pub reagent_c: ReagentConfig,
    pub ratio: Vec<u32>,
    pub volume: f64,
    pub mixing_cycles: u32,
    pub incubation_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReagentConfig {
    pub name: String,
    pub type_name: String,
    pub concentration: f64,
    pub well_location: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NucleraCartridgeConfig {
    pub cartridge_id: String,
    pub droplets: Vec<NucleraDropletConfig>,
    pub generation_order: Vec<String>,
    pub readout_step: ReadoutConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadoutConfig {
    pub method: String,
    pub wavelength: Option<f64>,
    pub exposure_time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolStep {
    pub step_number: usize,
    pub description: String,
    pub duration: Option<f64>,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentProtocol {
    pub experiment_id: String,
    pub steps: Vec<ProtocolStep>,
    pub required_reagents: Vec<String>,
    pub equipment: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperimentalDataPoint {
    pub droplet_id: String,
    pub oxidant_concentration: f64,
    pub treatment: String,
    pub time: f64,
    pub fluorescence_signal: f64,
    pub viability_score: f64,
    pub gumol_predicted_damage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationReport {
    pub experiment_id: String,
    pub rmse: f64,
    pub pearson_correlation: f64,
    pub threshold_accuracy: f64,
    pub simulation_error_distribution: Vec<f64>,
    pub experimental_variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterMapping {
    pub simulation_params: HashMap<String, f64>,
    pub experimental_ranges: HashMap<String, Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CardDesign {
    pub name: String,
    pub description: String,
    pub rows: usize,
    pub cols: usize,
    pub droplet_conditions: Vec<DropletCondition>,
}

impl CardDesign {
    pub fn new(name: String) -> Self {
        CardDesign {
            name,
            description: String::new(),
            rows: 8,
            cols: 12,
            droplet_conditions: Vec::new(),
        }
    }

    pub fn total_wells(&self) -> usize {
        self.rows * self.cols
    }
}
