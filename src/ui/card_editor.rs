use bevy_egui::egui;
use crate::data_models::{ApplicationState, ColorMode};
use crate::modules::ExperimentDesigner;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::SidePanel::left("card_controls")
        .exact_width(230.0)
        .show(ctx, |ui| {
            ui.heading("Card Config");
            ui.separator();

            ui.label("Name:");
            ui.text_edit_singleline(&mut state.card_name);
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label("Rows:");
                ui.add(egui::DragValue::new(&mut state.grid_rows).range(1..=16));
                ui.label("Cols:");
                ui.add(egui::DragValue::new(&mut state.grid_cols).range(1..=24));
            });
            ui.label(format!("Wells: {}", state.grid_rows * state.grid_cols));
            ui.add_space(6.0);

            ui.separator();
            ui.label("Oxidant:");
            egui::ComboBox::from_id_source("ox_type")
                .selected_text(&state.oxidant_type)
                .width(120.0)
                .show_ui(ui, |ui| {
                    for t in ["H2O2", "O2-", "OH"] {
                        ui.selectable_value(&mut state.oxidant_type, t.to_string(), t);
                    }
                });

            ui.add_space(4.0);
            ui.label("Concentration (µM):");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut state.min_concentration).speed(1.0).prefix("min "));
                ui.add(egui::DragValue::new(&mut state.max_concentration).speed(5.0).prefix("max "));
            });

            ui.add_space(6.0);
            ui.separator();
            ui.label("Antioxidants:");
            for (name, enabled) in state.antioxidants.iter_mut() {
                ui.checkbox(enabled, name.as_str());
            }

            ui.add_space(6.0);
            ui.separator();
            ui.label("Exposure times (min):");
            let times_str: Vec<String> = state.exposure_times.iter().map(|t| format!("{:.0}", t)).collect();
            let mut combined = times_str.join(", ");
            if ui.text_edit_singleline(&mut combined).changed() {
                state.exposure_times = combined
                    .split(',')
                    .filter_map(|s| s.trim().parse::<f64>().ok())
                    .collect();
            }

            ui.add_space(10.0);
            ui.separator();

            if ui.button("Generate Matrix").clicked() {
                let active_antioxidants: Vec<String> = state.antioxidants.iter()
                    .filter(|(_, e)| *e)
                    .map(|(n, _)| n.clone())
                    .collect();

                let n_levels = 5usize;
                let step = if n_levels > 1 {
                    (state.max_concentration - state.min_concentration) / (n_levels - 1) as f64
                } else {
                    0.0
                };
                let oxidant_levels: Vec<f64> = (0..n_levels)
                    .map(|i| state.min_concentration + step * i as f64)
                    .collect();

                let max = state.grid_rows * state.grid_cols;
                let matrix = ExperimentDesigner::generate_matrix_from_config(
                    &oxidant_levels,
                    &active_antioxidants,
                    &state.exposure_times,
                    max,
                    &state.oxidant_type,
                );
                state.status_message = format!("Matrix generated: {} droplets", matrix.droplets.len());
                state.droplet_matrix = Some(matrix);
                state.selected_droplet = None;
            }

            if state.droplet_matrix.is_some() {
                ui.add_space(6.0);
                ui.label("Color mode:");
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut state.color_mode, ColorMode::Oxidant, "Ox");
                    ui.selectable_value(&mut state.color_mode, ColorMode::Antioxidant, "Ax");
                });
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut state.color_mode, ColorMode::ExposureTime, "Time");
                    ui.selectable_value(&mut state.color_mode, ColorMode::Treatment, "Treat");
                });
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading(&state.card_name);
        ui.separator();

        let has_matrix = state.droplet_matrix.is_some();
        let max_conc = state.max_concentration;
        let rows = state.grid_rows;
        let cols = state.grid_cols;
        let color_mode = state.color_mode;

        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                let available = ui.available_width();
                let cell = ((available - 30.0) / (cols as f32 + 1.0)).min(50.0).max(20.0);
                let spacing = 2.0;

                ui.horizontal(|ui| {
                    ui.add_space(cell + spacing);
                    for c in 0..cols {
                        let _r = ui.add_sized([cell, 18.0], egui::Label::new(
                            egui::RichText::new(format!("{}", c + 1)).small().strong()
                        ));
                    }
                });

                for row in 0..rows {
                    ui.horizontal(|ui| {
                        let letter = (b'A' + (row % 26) as u8) as char;
                        ui.add_sized([cell, cell], egui::Label::new(
                            egui::RichText::new(format!("{}", letter)).strong()
                        ));

                        for col in 0..cols {
                            let idx = row * cols + col;
                            let (color, tip) = if has_matrix {
                                if let Some(cond) = state.droplet_matrix.as_ref()
                                    .and_then(|m| m.droplets.get(idx))
                                {
                                    let c = super::droplet_color(cond, color_mode, max_conc);
                                    let tip = format!(
                                        "{}{}: {} {:.0}µM\n{} {:.0}U/mL\n{:.0} min",
                                        letter, col + 1,
                                        cond.oxidant_type, cond.oxidant_concentration,
                                        cond.antioxidant, cond.antioxidant_concentration,
                                        cond.exposure_time,
                                    );
                                    (c, tip)
                                } else {
                                    (super::empty_cell_color(), format!("{}{}: empty", letter, col + 1))
                                }
                            } else {
                                (super::empty_cell_color(), format!("{}{}: no matrix", letter, col + 1))
                            };

                            let is_selected = state.selected_droplet == Some(idx);
                            let stroke = if is_selected {
                                egui::Stroke::new(2.0, egui::Color32::YELLOW)
                            } else {
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(180, 180, 180))
                            };

                            let btn = egui::Button::new("")
                                .fill(color)
                                .stroke(stroke)
                                .min_size(egui::vec2(cell, cell));
                            let resp = ui.add(btn);

                            if resp.clicked() {
                                state.selected_droplet = Some(idx);
                            }
                            resp.on_hover_text(&tip);
                        }
                    });
                }

                // Legend
                ui.add_space(10.0);
                ui.separator();
                render_legend(ui, color_mode);

                // Selected droplet details
                if let Some(sel) = state.selected_droplet {
                    if let Some(cond) = state.droplet_matrix.as_ref().and_then(|m| m.droplets.get(sel)) {
                        ui.add_space(6.0);
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.strong("Selected:");
                            ui.label(format!(
                                "{} | {} {:.0}µM | {} {:.0}U/mL | {:.0}min | {}",
                                cond.droplet_id,
                                cond.oxidant_type, cond.oxidant_concentration,
                                cond.antioxidant, cond.antioxidant_concentration,
                                cond.exposure_time,
                                cond.buffer_type,
                            ));
                        });
                    }
                }
            });
    });
}

fn render_legend(ui: &mut egui::Ui, mode: ColorMode) {
    ui.horizontal(|ui| {
        ui.strong("Legend:");
        match mode {
            ColorMode::Oxidant => {
                legend_swatch(ui, egui::Color32::WHITE, "0 µM");
                legend_swatch(ui, egui::Color32::from_rgb(255, 180, 180), "Low");
                legend_swatch(ui, egui::Color32::from_rgb(255, 120, 120), "Med");
                legend_swatch(ui, egui::Color32::from_rgb(255, 60, 60), "High");
            }
            ColorMode::Antioxidant => {
                legend_swatch(ui, egui::Color32::from_rgb(230, 230, 230), "None");
                legend_swatch(ui, egui::Color32::from_rgb(180, 255, 180), "Low");
                legend_swatch(ui, egui::Color32::from_rgb(100, 255, 100), "High");
            }
            ColorMode::ExposureTime => {
                legend_swatch(ui, egui::Color32::from_rgb(220, 220, 255), "Short");
                legend_swatch(ui, egui::Color32::from_rgb(100, 100, 255), "Long");
            }
            ColorMode::Treatment => {
                legend_swatch(ui, egui::Color32::from_rgb(210, 210, 230), "Control");
                legend_swatch(ui, egui::Color32::from_rgb(180, 240, 180), "SOD3");
                legend_swatch(ui, egui::Color32::from_rgb(240, 210, 180), "Catalase");
                legend_swatch(ui, egui::Color32::from_rgb(180, 210, 240), "GPx");
            }
        }
    });
}

fn legend_swatch(ui: &mut egui::Ui, color: egui::Color32, label: &str) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(14.0, 14.0), egui::Sense::hover());
    ui.painter().rect_filled(rect, 2.0, color);
    ui.painter().rect_stroke(rect, 2.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
    ui.label(label);
    ui.add_space(6.0);
}
