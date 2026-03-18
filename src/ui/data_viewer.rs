use bevy_egui::egui;
use crate::data_models::ApplicationState;
use crate::modules::CorrelationAnalyzer;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::render_header(ui, "7. Data Viewer & Correlation");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_max_width(380.0);

                ui.heading("Analysis");
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Run Sample Correlation").clicked() {
                        if let Some(matrix) = &state.droplet_matrix {
                            let predictions: Vec<f64> = matrix.droplets.iter()
                                .map(|d| d.oxidant_concentration / 250.0)
                                .collect();
                            let measurements: Vec<f64> = predictions.iter()
                                .enumerate()
                                .map(|(i, p)| {
                                    p + 0.05 * ((i as f64) * 0.7).sin()
                                })
                                .collect();

                            let report = CorrelationAnalyzer::analyze_correlation(
                                &predictions,
                                &measurements,
                            );
                            state.status_message = format!(
                                "Correlation: r={:.3}, RMSE={:.3}",
                                report.pearson_correlation, report.rmse,
                            );
                            state.correlation_report = Some(report);
                        } else {
                            state.status_message = "Generate a matrix first".to_string();
                        }
                    }
                });

                ui.add_space(10.0);

                if let Some(report) = &state.correlation_report {
                    ui.group(|ui| {
                        ui.strong("Correlation Report");
                        ui.add_space(4.0);

                        egui::Grid::new("corr_grid")
                            .num_columns(2)
                            .spacing([20.0, 6.0])
                            .show(ui, |ui| {
                                ui.label("Experiment ID:");
                                ui.label(&report.experiment_id);
                                ui.end_row();

                                ui.label("RMSE:");
                                ui.label(format!("{:.4}", report.rmse));
                                ui.end_row();

                                ui.label("Pearson r:");
                                ui.label(format!("{:.4}", report.pearson_correlation));
                                ui.end_row();

                                ui.label("Threshold accuracy:");
                                ui.label(format!("{:.1}%", report.threshold_accuracy * 100.0));
                                ui.end_row();

                                ui.label("Experimental variance:");
                                ui.label(format!("{:.4}", report.experimental_variance));
                                ui.end_row();

                                ui.label("Error points:");
                                ui.label(format!("{}", report.simulation_error_distribution.len()));
                                ui.end_row();
                            });
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        if ui.button("Export Report (JSON)").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name("correlation_report.json")
                                .add_filter("JSON", &["json"])
                                .save_file()
                            {
                                match serde_json::to_string_pretty(report) {
                                    Ok(json) => match std::fs::write(&path, &json) {
                                        Ok(_) => state.status_message = format!("Saved to {}", path.display()),
                                        Err(e) => state.status_message = format!("Write error: {}", e),
                                    },
                                    Err(e) => state.status_message = format!("JSON error: {}", e),
                                }
                            }
                        }
                    });
                }

                ui.add_space(15.0);

                if let Some(matrix) = &state.droplet_matrix {
                    ui.heading("Matrix Data");
                    ui.separator();

                    if ui.button("Export Full Data (CSV)").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name("experiment_data.csv")
                            .add_filter("CSV", &["csv"])
                            .save_file()
                        {
                            match export_data_csv(matrix, &path) {
                                Ok(_) => state.status_message = format!("Saved to {}", path.display()),
                                Err(e) => state.status_message = format!("Export error: {}", e),
                            }
                        }
                    }

                    if ui.button("Export Full Data (JSON)").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name("experiment_data.json")
                            .add_filter("JSON", &["json"])
                            .save_file()
                        {
                            match serde_json::to_string_pretty(matrix) {
                                Ok(json) => match std::fs::write(&path, &json) {
                                    Ok(_) => state.status_message = format!("Saved to {}", path.display()),
                                    Err(e) => state.status_message = format!("Write error: {}", e),
                                },
                                Err(e) => state.status_message = format!("JSON error: {}", e),
                            }
                        }
                    }
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.heading("Scatter Plot");
                ui.separator();

                if let Some(_report) = &state.correlation_report {
                    if let Some(matrix) = &state.droplet_matrix {
                        let predictions: Vec<f64> = matrix.droplets.iter()
                            .map(|d| d.oxidant_concentration / 250.0)
                            .collect();
                        let measurements: Vec<f64> = predictions.iter()
                            .enumerate()
                            .map(|(i, p)| p + 0.05 * ((i as f64) * 0.7).sin())
                            .collect();

                        draw_scatter(ui, &predictions, &measurements);

                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label("Green = |error| < 0.05");
                            ui.label("Red = |error| >= 0.05");
                        });
                    }
                } else {
                    ui.add_space(20.0);
                    ui.label("Run a correlation analysis to see the scatter plot.");
                }
            });
        });
    });
}

fn draw_scatter(ui: &mut egui::Ui, predictions: &[f64], measurements: &[f64]) {
    let size = egui::vec2(400.0, 350.0);
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = response.rect;

    let margin = 35.0;
    let plot_min = egui::pos2(rect.min.x + margin, rect.min.y + 10.0);
    let plot_max = egui::pos2(rect.max.x - 10.0, rect.max.y - margin);
    let plot_w = plot_max.x - plot_min.x;
    let plot_h = plot_max.y - plot_min.y;

    painter.rect_filled(
        egui::Rect::from_min_max(plot_min, plot_max),
        0.0,
        egui::Color32::from_rgb(40, 40, 50),
    );

    painter.line_segment(
        [egui::pos2(plot_min.x, plot_max.y), egui::pos2(plot_max.x, plot_min.y)],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 80)),
    );

    let n = predictions.len().min(measurements.len());
    for i in 0..n {
        let px = plot_min.x + (predictions[i] as f32).clamp(0.0, 1.0) * plot_w;
        let py = plot_max.y - (measurements[i] as f32).clamp(0.0, 1.0) * plot_h;
        let err = (predictions[i] - measurements[i]).abs();
        let color = if err < 0.05 {
            egui::Color32::from_rgb(80, 220, 80)
        } else {
            egui::Color32::from_rgb(220, 80, 80)
        };
        painter.circle_filled(egui::pos2(px, py), 3.0, color);
    }

    painter.text(
        egui::pos2((plot_min.x + plot_max.x) / 2.0, rect.max.y - 5.0),
        egui::Align2::CENTER_BOTTOM,
        "Gumol Prediction",
        egui::FontId::proportional(12.0),
        egui::Color32::GRAY,
    );
    painter.text(
        egui::pos2(rect.min.x + 5.0, (plot_min.y + plot_max.y) / 2.0),
        egui::Align2::LEFT_CENTER,
        "Measured",
        egui::FontId::proportional(12.0),
        egui::Color32::GRAY,
    );
}

fn export_data_csv(
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
