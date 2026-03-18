use bevy_egui::egui;
use crate::data_models::{ApplicationState, CurrentPanel};
use crate::modules::ExperimentDesigner;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState, panel: &mut CurrentPanel) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::render_header(ui, "4. Experiment Design");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_max_width(350.0);
                ui.heading("Matrix Configuration");
                ui.separator();

                ui.horizontal(|ui| {
                    ui.label("Rows:");
                    ui.add(egui::DragValue::new(&mut state.grid_rows).range(1..=16));
                    ui.label("Cols:");
                    ui.add(egui::DragValue::new(&mut state.grid_cols).range(1..=24));
                });
                ui.label(format!("Cartridge capacity: {} droplets", state.grid_rows * state.grid_cols));

                ui.add_space(10.0);

                let active_ax: Vec<String> = state.antioxidants.iter()
                    .filter(|(_, e)| *e)
                    .map(|(n, _)| n.clone())
                    .collect();
                let n_ox = if state.parameter_ranges.contains_key("oxidant_concentration") {
                    state.parameter_ranges["oxidant_concentration"].len()
                } else { 5 };

                ui.group(|ui| {
                    ui.label("Design Factors");
                    ui.label(format!("Oxidant levels: {}", n_ox));
                    ui.label(format!("Antioxidant conditions: {}", active_ax.len()));
                    ui.label(format!("Time points: {}", state.exposure_times.len()));
                    let full = n_ox * active_ax.len() * state.exposure_times.len();
                    ui.label(format!("Full factorial: {} conditions", full));
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Generate Matrix").clicked() {
                        let oxidant_levels = state.parameter_ranges
                            .get("oxidant_concentration")
                            .cloned()
                            .unwrap_or_else(|| {
                                let n = 5;
                                let step = (state.max_concentration - state.min_concentration) / (n - 1) as f64;
                                (0..n).map(|i| state.min_concentration + step * i as f64).collect()
                            });

                        let max = state.grid_rows * state.grid_cols;
                        let matrix = ExperimentDesigner::generate_matrix_from_config(
                            &oxidant_levels,
                            &active_ax,
                            &state.exposure_times,
                            max,
                            &state.oxidant_type,
                        );
                        state.status_message = format!("Matrix: {} droplets", matrix.droplets.len());
                        state.droplet_matrix = Some(matrix);
                    }

                    if state.droplet_matrix.is_some() {
                        if ui.button("Optimize for 96").clicked() {
                            if let Some(m) = state.droplet_matrix.take() {
                                let opt = ExperimentDesigner::optimize_for_cartridge(m, 96);
                                state.status_message = format!("Optimized to {} droplets", opt.droplets.len());
                                state.droplet_matrix = Some(opt);
                            }
                        }
                    }
                });

                if let Some(matrix) = &state.droplet_matrix {
                    ui.add_space(10.0);
                    ui.group(|ui| {
                        ui.strong("Matrix Stats");
                        ui.label(format!("Total: {} droplets", matrix.metadata.total_droplets));
                        ui.label(format!("Grid: {}×{}", matrix.metadata.grid_rows, matrix.metadata.grid_cols));
                        ui.label(format!("ID: {}", matrix.metadata.experiment_id));
                        let vol = matrix.droplets.len() as f64 * state.default_volume;
                        ui.label(format!("Est. volume: {:.0} nL", vol));
                    });
                }

                ui.add_space(15.0);
                if ui.button("Next: Nuclera Config →").clicked() {
                    *panel = CurrentPanel::NucleraConfig;
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.heading("Matrix Preview");
                ui.separator();

                if let Some(matrix) = &state.droplet_matrix {
                    egui::ScrollArea::vertical()
                        .max_height(500.0)
                        .show(ui, |ui| {
                            egui::Grid::new("matrix_preview")
                                .num_columns(5)
                                .spacing([12.0, 4.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.strong("ID");
                                    ui.strong("Oxidant (µM)");
                                    ui.strong("Antioxidant");
                                    ui.strong("Ax (U/mL)");
                                    ui.strong("Time (min)");
                                    ui.end_row();

                                    for d in &matrix.droplets {
                                        ui.label(&d.droplet_id);
                                        ui.label(format!("{:.0}", d.oxidant_concentration));
                                        ui.label(&d.antioxidant);
                                        ui.label(format!("{:.0}", d.antioxidant_concentration));
                                        ui.label(format!("{:.0}", d.exposure_time));
                                        ui.end_row();
                                    }
                                });
                        });
                } else {
                    ui.label("Generate a matrix to see preview.");
                }

                if state.droplet_matrix.is_some() {
                    ui.add_space(10.0);
                    if ui.button("Export Matrix (CSV)").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name("droplet_matrix.csv")
                            .add_filter("CSV", &["csv"])
                            .save_file()
                        {
                            if let Some(matrix) = &state.droplet_matrix {
                                match export_matrix_csv(matrix, &path) {
                                    Ok(_) => state.status_message = format!("Exported to {}", path.display()),
                                    Err(e) => state.status_message = format!("Export error: {}", e),
                                }
                            }
                        }
                    }
                }
            });
        });
    });
}

fn export_matrix_csv(
    matrix: &crate::data_models::DropletMatrix,
    path: &std::path::Path,
) -> anyhow::Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    wtr.write_record(["droplet_id", "oxidant_type", "oxidant_uM", "antioxidant", "antioxidant_UmL", "exposure_min", "buffer"])?;
    for d in &matrix.droplets {
        wtr.write_record([
            &d.droplet_id,
            &d.oxidant_type,
            &format!("{:.1}", d.oxidant_concentration),
            &d.antioxidant,
            &format!("{:.1}", d.antioxidant_concentration),
            &format!("{:.0}", d.exposure_time),
            &d.buffer_type,
        ])?;
    }
    wtr.flush()?;
    Ok(())
}
