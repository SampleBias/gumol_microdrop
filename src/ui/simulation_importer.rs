use bevy_egui::egui;
use crate::data_models::{ApplicationState, CurrentPanel};
use crate::modules::{SimulationImporter, ParameterTranslationEngine};
use rfd::FileDialog;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState, panel: &mut CurrentPanel) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::render_header(ui, "1. Simulation Import");
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            if ui.button("Open Simulation File").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("JSON", &["json"])
                    .add_filter("CSV", &["csv"])
                    .pick_file()
                {
                    match SimulationImporter::import_from_json(&path) {
                        Ok(sim) => {
                            let fv = SimulationImporter::extract_feature_vector(&sim);
                            let ranges = ParameterTranslationEngine::translate_simulation_to_experimental(&fv);
                            state.feature_vector = Some(fv);
                            state.parameter_ranges = ranges;
                            state.simulation = Some(sim);
                            state.simulation_file = Some(path);
                            state.status_message = "Simulation loaded successfully".to_string();
                        }
                        Err(e) => {
                            state.status_message = format!("Import error: {}", e);
                        }
                    }
                }
            }

            if ui.button("Load Sample Data").clicked() {
                let sample_path = std::path::PathBuf::from("sample_simulation.json");
                match SimulationImporter::import_from_json(&sample_path) {
                    Ok(sim) => {
                        let fv = SimulationImporter::extract_feature_vector(&sim);
                        let ranges = ParameterTranslationEngine::translate_simulation_to_experimental(&fv);
                        state.feature_vector = Some(fv);
                        state.parameter_ranges = ranges;
                        state.simulation = Some(sim);
                        state.simulation_file = Some(sample_path);
                        state.status_message = "Sample simulation loaded".to_string();
                    }
                    Err(e) => {
                        state.status_message = format!("Sample load error: {}", e);
                    }
                }
            }
        });

        ui.add_space(15.0);

        if let Some(sim) = &state.simulation {
            ui.group(|ui| {
                ui.heading("Loaded Simulation");
                ui.label(format!("ID: {}", sim.simulation_id));
                ui.label(format!("Species: {}", sim.radical_species.join(", ")));
                ui.label(format!("Time points: {}", sim.time_series.len()));
                ui.label(format!("Temperature: {:.1} K", sim.temperature));
                ui.label(format!("Damage threshold: {:.2}", sim.damage_threshold));
            });

            ui.add_space(10.0);

            if let Some(fv) = &state.feature_vector {
                ui.group(|ui| {
                    ui.heading("Extracted Features");
                    ui.label(format!("Mean superoxide density: {:.3}", fv.mean_superoxide_density));
                    ui.label(format!("Peak H2O2: {:.3}", fv.peak_hydrogen_peroxide));
                    ui.label(format!("Avg diffusion rate: {:.3}", fv.avg_diffusion_rate));
                    ui.label(format!("Reaction velocity: {:.3}", fv.reaction_velocity));
                    ui.label(format!("Antioxidant scavenging: {:.3}", fv.antioxidant_scavenging_rate));
                });
            }

            ui.add_space(15.0);
            if ui.button("Next: Card Editor →").clicked() {
                *panel = CurrentPanel::CardEditor;
            }
        } else {
            ui.add_space(20.0);
            ui.group(|ui| {
                ui.heading("Supported Formats");
                ui.label("  JSON (recommended)");
                ui.label("  CSV");
            });

            ui.add_space(10.0);
            ui.group(|ui| {
                ui.heading("Expected JSON Schema");
                ui.monospace("{\n  \"simulation_id\": \"string\",\n  \"radical_species\": [\"O2-\", \"H2O2\", \"OH\"],\n  \"time_series\": [...],\n  \"diffusion_constants\": {...},\n  \"reaction_rates\": {...},\n  \"damage_threshold\": 0.67,\n  \"temperature\": 298.15\n}");
            });
        }
    });
}
