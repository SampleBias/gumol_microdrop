use bevy_egui::egui;
use crate::data_models::{ApplicationState, CurrentPanel};
use crate::modules::NucleraGenerator;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState, panel: &mut CurrentPanel) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::render_header(ui, "5. Nuclera Configuration");

        if state.droplet_matrix.is_none() {
            ui.label("No droplet matrix generated. Go to Card Editor or Experiment Design first.");
            if ui.button("← Back to Card Editor").clicked() {
                *panel = CurrentPanel::CardEditor;
            }
            return;
        }

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_max_width(350.0);
                ui.heading("Cartridge Settings");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Volume (nL):");
                    ui.add(egui::DragValue::new(&mut state.default_volume).speed(0.5).range(0.1..=100.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Mixing cycles:");
                    ui.add(egui::DragValue::new(&mut state.mixing_cycles).range(1..=20));
                });

                ui.add_space(10.0);
                ui.heading("Readout");
                ui.separator();

                ui.label("Method:");
                egui::ComboBox::from_id_source("readout_method")
                    .selected_text(&state.readout_method)
                    .width(140.0)
                    .show_ui(ui, |ui| {
                        for m in ["Fluorescence", "Absorbance", "Luminescence"] {
                            ui.selectable_value(&mut state.readout_method, m.to_string(), m);
                        }
                    });

                ui.horizontal(|ui| {
                    ui.label("Wavelength (nm):");
                    ui.add(egui::DragValue::new(&mut state.wavelength).speed(5.0).range(200.0..=900.0));
                });
                ui.horizontal(|ui| {
                    ui.label("Exposure (s):");
                    ui.add(egui::DragValue::new(&mut state.readout_exposure).speed(0.01).range(0.001..=10.0));
                });

                ui.add_space(15.0);

                if ui.button("Generate Nuclera Config").clicked() {
                    if let Some(matrix) = &state.droplet_matrix {
                        let config = NucleraGenerator::generate_config(matrix);
                        state.status_message = format!(
                            "Nuclera config generated: {} droplets",
                            config.droplets.len()
                        );
                        state.nuclera_config = Some(config);
                    }
                }

                if state.nuclera_config.is_some() {
                    ui.add_space(6.0);
                    if ui.button("Export YAML").clicked() {
                        if let Some(config) = &state.nuclera_config {
                            match NucleraGenerator::export_to_yaml(config) {
                                Ok(yaml) => {
                                    if let Some(path) = rfd::FileDialog::new()
                                        .set_file_name("nuclera_config.yaml")
                                        .add_filter("YAML", &["yaml", "yml"])
                                        .save_file()
                                    {
                                        match std::fs::write(&path, &yaml) {
                                            Ok(_) => state.status_message = format!("Saved to {}", path.display()),
                                            Err(e) => state.status_message = format!("Write error: {}", e),
                                        }
                                    }
                                }
                                Err(e) => state.status_message = format!("YAML error: {}", e),
                            }
                        }
                    }
                }

                ui.add_space(15.0);
                if ui.button("Next: Protocol →").clicked() {
                    *panel = CurrentPanel::ProtocolGenerator;
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.heading("Config Preview");
                ui.separator();

                if let Some(config) = &state.nuclera_config {
                    egui::ScrollArea::vertical()
                        .max_height(550.0)
                        .show(ui, |ui| {
                            ui.label(format!("Cartridge ID: {}", config.cartridge_id));
                            ui.label(format!("Droplets: {}", config.droplets.len()));
                            ui.label(format!(
                                "Readout: {} @ {:.0}nm, {:.2}s",
                                config.readout_step.method,
                                config.readout_step.wavelength.unwrap_or(0.0),
                                config.readout_step.exposure_time,
                            ));

                            ui.add_space(8.0);
                            ui.separator();

                            match NucleraGenerator::export_to_yaml(config) {
                                Ok(yaml) => {
                                    let preview: String = yaml.lines().take(60).collect::<Vec<_>>().join("\n");
                                    ui.monospace(&preview);
                                    if yaml.lines().count() > 60 {
                                        ui.label("... (truncated)");
                                    }
                                }
                                Err(_) => { ui.label("Error generating YAML preview"); }
                            }
                        });
                } else {
                    ui.label("Click 'Generate Nuclera Config' to see preview.");
                }
            });
        });
    });
}
