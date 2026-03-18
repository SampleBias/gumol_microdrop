use bevy_egui::egui;
use crate::data_models::{ApplicationState, CurrentPanel};
use crate::modules::ParameterTranslationEngine;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState, panel: &mut CurrentPanel) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::render_header(ui, "3. Parameter Mapping");

        if state.feature_vector.is_none() {
            ui.label("No simulation loaded. Go to Step 1 to import data.");
            if ui.button("← Back to Import").clicked() {
                *panel = CurrentPanel::SimulationImport;
            }
            return;
        }

        ui.add_space(8.0);
        ui.heading("Simulation → Experimental Parameters");
        ui.add_space(4.0);

        egui::Grid::new("param_map_grid")
            .num_columns(3)
            .spacing([30.0, 8.0])
            .striped(true)
            .show(ui, |ui| {
                ui.strong("Gumol Parameter");
                ui.strong("→");
                ui.strong("Experimental Variable");
                ui.end_row();

                let mappings = [
                    ("Superoxide density", "Oxidant concentration (µM)"),
                    ("Diffusion coefficient", "Droplet mixing ratio"),
                    ("Reaction rate", "Exposure time (min)"),
                    ("Antioxidant neutralization", "Enzyme concentration (U/mL)"),
                    ("Damage threshold", "ROS reporter signal"),
                ];

                for (sim_param, lab_param) in mappings {
                    ui.label(sim_param);
                    ui.label("→");
                    ui.label(lab_param);
                    ui.end_row();
                }
            });

        ui.add_space(15.0);

        if let Some(fv) = &state.feature_vector {
            ui.heading("Computed Feature Values");
            ui.add_space(4.0);

            egui::Grid::new("fv_grid")
                .num_columns(2)
                .spacing([20.0, 6.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Mean superoxide density:");
                    ui.label(format!("{:.4}", fv.mean_superoxide_density));
                    ui.end_row();
                    ui.label("Peak H2O2:");
                    ui.label(format!("{:.4}", fv.peak_hydrogen_peroxide));
                    ui.end_row();
                    ui.label("Avg diffusion rate:");
                    ui.label(format!("{:.4}", fv.avg_diffusion_rate));
                    ui.end_row();
                    ui.label("Reaction velocity:");
                    ui.label(format!("{:.4}", fv.reaction_velocity));
                    ui.end_row();
                    ui.label("Antioxidant scavenging:");
                    ui.label(format!("{:.4}", fv.antioxidant_scavenging_rate));
                    ui.end_row();
                });
        }

        ui.add_space(15.0);

        ui.heading("Translated Experimental Ranges");
        ui.add_space(4.0);

        if !state.parameter_ranges.is_empty() {
            for (param, values) in &state.parameter_ranges {
                let vals: Vec<String> = values.iter().map(|v| format!("{:.1}", v)).collect();
                ui.horizontal(|ui| {
                    ui.strong(format!("{}:", param));
                    ui.label(format!("[{}]", vals.join(", ")));
                });
            }
        } else {
            ui.label("No parameter ranges computed yet.");
        }

        ui.add_space(10.0);

        if ui.button("Recalculate Ranges").clicked() {
            if let Some(fv) = &state.feature_vector {
                state.parameter_ranges = ParameterTranslationEngine::translate_simulation_to_experimental(fv);
                state.status_message = "Parameter ranges recalculated".to_string();
            }
        }

        ui.add_space(15.0);
        if ui.button("Next: Experiment Design →").clicked() {
            *panel = CurrentPanel::ExperimentDesign;
        }
    });
}
