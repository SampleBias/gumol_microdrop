use bevy_egui::egui;
use crate::data_models::{ApplicationState, ExperimentalDataPoint};
use crate::modules::CorrelationAnalyzer;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::render_header(ui, "7. Data Viewer & Correlation");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_max_width(400.0);

                ui.heading("Experimental Data");
                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Load Experimental CSV").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("CSV", &["csv"])
                            .pick_file()
                        {
                            match load_experimental_csv(&path) {
                                Ok(data) => {
                                    state.status_message = format!("Loaded {} data points", data.len());
                                    state.experimental_data = data;
                                }
                                Err(e) => state.status_message = format!("Load error: {}", e),
                            }
                        }
                    }

                    if ui.button("Load Sample Results").clicked() {
                        let path = std::path::PathBuf::from("sample_experimental_results.csv");
                        match load_experimental_csv(&path) {
                            Ok(data) => {
                                state.status_message = format!("Loaded {} sample data points", data.len());
                                state.experimental_data = data;
                            }
                            Err(e) => state.status_message = format!("Load error: {}", e),
                        }
                    }
                });

                if !state.experimental_data.is_empty() {
                    ui.add_space(6.0);
                    ui.label(format!("Loaded: {} measurements", state.experimental_data.len()));

                    ui.add_space(10.0);

                    if ui.button("Run Correlation Analysis").clicked() {
                        let predictions: Vec<f64> = state.experimental_data.iter()
                            .map(|d| d.gumol_predicted_damage)
                            .collect();
                        let measurements: Vec<f64> = state.experimental_data.iter()
                            .map(|d| 1.0 - (d.viability_score / 100.0))
                            .collect();

                        let report = CorrelationAnalyzer::analyze_correlation(
                            &predictions,
                            &measurements,
                        );
                        state.status_message = format!(
                            "Correlation: r={:.3}, RMSE={:.4}, accuracy={:.1}%",
                            report.pearson_correlation, report.rmse, report.threshold_accuracy * 100.0,
                        );
                        state.scatter_predictions = predictions;
                        state.scatter_measurements = measurements;
                        state.correlation_report = Some(report);
                    }
                }

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
                                let rmse_color = if report.rmse < 0.05 {
                                    egui::Color32::from_rgb(80, 200, 80)
                                } else if report.rmse < 0.1 {
                                    egui::Color32::from_rgb(200, 200, 80)
                                } else {
                                    egui::Color32::from_rgb(200, 80, 80)
                                };
                                ui.colored_label(rmse_color, format!("{:.4}", report.rmse));
                                ui.end_row();

                                ui.label("Pearson r:");
                                let r_color = if report.pearson_correlation > 0.9 {
                                    egui::Color32::from_rgb(80, 200, 80)
                                } else if report.pearson_correlation > 0.7 {
                                    egui::Color32::from_rgb(200, 200, 80)
                                } else {
                                    egui::Color32::from_rgb(200, 80, 80)
                                };
                                ui.colored_label(r_color, format!("{:.4}", report.pearson_correlation));
                                ui.end_row();

                                ui.label("Threshold accuracy:");
                                ui.label(format!("{:.1}%", report.threshold_accuracy * 100.0));
                                ui.end_row();

                                ui.label("Experimental variance:");
                                ui.label(format!("{:.6}", report.experimental_variance));
                                ui.end_row();

                                ui.label("Data points:");
                                ui.label(format!("{}", report.simulation_error_distribution.len()));
                                ui.end_row();
                            });
                    });

                    ui.add_space(8.0);

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
                }

                ui.add_space(15.0);

                if !state.experimental_data.is_empty() {
                    ui.heading("Data Table");
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .max_height(250.0)
                        .show(ui, |ui| {
                            egui::Grid::new("exp_data_table")
                                .num_columns(6)
                                .spacing([8.0, 3.0])
                                .striped(true)
                                .show(ui, |ui| {
                                    ui.strong("ID");
                                    ui.strong("µM");
                                    ui.strong("Treat");
                                    ui.strong("RFU");
                                    ui.strong("Viab%");
                                    ui.strong("Pred");
                                    ui.end_row();

                                    for d in &state.experimental_data {
                                        ui.label(&d.droplet_id);
                                        ui.label(format!("{:.0}", d.oxidant_concentration));
                                        ui.label(&d.treatment);
                                        ui.label(format!("{:.0}", d.fluorescence_signal));
                                        ui.label(format!("{:.1}", d.viability_score));
                                        ui.label(format!("{:.2}", d.gumol_predicted_damage));
                                        ui.end_row();
                                    }
                                });
                        });

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        if ui.button("Export Data (CSV)").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .set_file_name("experiment_data.csv")
                                .add_filter("CSV", &["csv"])
                                .save_file()
                            {
                                if let Some(matrix) = &state.droplet_matrix {
                                    match export_data_csv(matrix, &path) {
                                        Ok(_) => state.status_message = format!("Saved to {}", path.display()),
                                        Err(e) => state.status_message = format!("Export error: {}", e),
                                    }
                                }
                            }
                        }
                        if ui.button("Export Data (JSON)").clicked() {
                            if let Some(matrix) = &state.droplet_matrix {
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
                }
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.heading("Prediction vs Measurement");
                ui.separator();

                if !state.scatter_predictions.is_empty() && !state.scatter_measurements.is_empty() {
                    draw_scatter(ui, &state.scatter_predictions, &state.scatter_measurements);

                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                        ui.painter().circle_filled(rect.center(), 4.0, egui::Color32::from_rgb(80, 220, 80));
                        ui.label("|error| < 0.05  ");

                        let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                        ui.painter().circle_filled(rect.center(), 4.0, egui::Color32::from_rgb(220, 180, 60));
                        ui.label("|error| < 0.10  ");

                        let (rect, _) = ui.allocate_exact_size(egui::vec2(10.0, 10.0), egui::Sense::hover());
                        ui.painter().circle_filled(rect.center(), 4.0, egui::Color32::from_rgb(220, 80, 80));
                        ui.label("|error| >= 0.10");
                    });

                    if state.experimental_data.len() >= 2 {
                        ui.add_space(15.0);
                        ui.heading("SOD3 Dose Response");
                        ui.separator();
                        draw_dose_response(ui, &state.experimental_data);
                    }
                } else {
                    ui.add_space(30.0);
                    ui.label("Load experimental data and run correlation to see plots.");
                    ui.add_space(10.0);
                    ui.label("Demo workflow:");
                    ui.label("  1. Click 'Load Sample Results'");
                    ui.label("  2. Click 'Run Correlation Analysis'");
                }
            });
        });
    });
}

fn draw_scatter(ui: &mut egui::Ui, predictions: &[f64], measurements: &[f64]) {
    let size = egui::vec2(420.0, 320.0);
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = response.rect;

    let margin_l = 45.0;
    let margin_b = 30.0;
    let margin_t = 10.0;
    let margin_r = 10.0;
    let plot_min = egui::pos2(rect.min.x + margin_l, rect.min.y + margin_t);
    let plot_max = egui::pos2(rect.max.x - margin_r, rect.max.y - margin_b);
    let plot_w = plot_max.x - plot_min.x;
    let plot_h = plot_max.y - plot_min.y;

    painter.rect_filled(
        egui::Rect::from_min_max(plot_min, plot_max),
        2.0,
        egui::Color32::from_rgb(30, 30, 40),
    );

    for i in 1..5 {
        let f = i as f32 / 4.0;
        let x = plot_min.x + f * plot_w;
        let y = plot_max.y - f * plot_h;
        painter.line_segment(
            [egui::pos2(x, plot_min.y), egui::pos2(x, plot_max.y)],
            egui::Stroke::new(0.5, egui::Color32::from_rgb(50, 50, 60)),
        );
        painter.line_segment(
            [egui::pos2(plot_min.x, y), egui::pos2(plot_max.x, y)],
            egui::Stroke::new(0.5, egui::Color32::from_rgb(50, 50, 60)),
        );
        painter.text(
            egui::pos2(x, plot_max.y + 4.0),
            egui::Align2::CENTER_TOP,
            format!("{:.1}", f),
            egui::FontId::proportional(10.0),
            egui::Color32::GRAY,
        );
        painter.text(
            egui::pos2(plot_min.x - 4.0, y),
            egui::Align2::RIGHT_CENTER,
            format!("{:.1}", f),
            egui::FontId::proportional(10.0),
            egui::Color32::GRAY,
        );
    }

    painter.line_segment(
        [egui::pos2(plot_min.x, plot_max.y), egui::pos2(plot_max.x, plot_min.y)],
        egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 100, 120)),
    );

    let n = predictions.len().min(measurements.len());
    for i in 0..n {
        let px = plot_min.x + (predictions[i] as f32).clamp(0.0, 1.0) * plot_w;
        let py = plot_max.y - (measurements[i] as f32).clamp(0.0, 1.0) * plot_h;
        let err = (predictions[i] - measurements[i]).abs();
        let color = if err < 0.05 {
            egui::Color32::from_rgb(80, 220, 80)
        } else if err < 0.10 {
            egui::Color32::from_rgb(220, 180, 60)
        } else {
            egui::Color32::from_rgb(220, 80, 80)
        };
        painter.circle_filled(egui::pos2(px, py), 4.0, color);
    }

    painter.text(
        egui::pos2((plot_min.x + plot_max.x) / 2.0, rect.max.y - 2.0),
        egui::Align2::CENTER_BOTTOM,
        "Gumol Predicted Damage",
        egui::FontId::proportional(11.0),
        egui::Color32::GRAY,
    );
    painter.text(
        egui::pos2(rect.min.x + 2.0, (plot_min.y + plot_max.y) / 2.0),
        egui::Align2::LEFT_CENTER,
        "1 - Viability",
        egui::FontId::proportional(11.0),
        egui::Color32::GRAY,
    );
}

fn draw_dose_response(ui: &mut egui::Ui, data: &[ExperimentalDataPoint]) {
    let size = egui::vec2(420.0, 200.0);
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = response.rect;

    let margin_l = 45.0;
    let margin_b = 25.0;
    let margin_t = 5.0;
    let margin_r = 10.0;
    let plot_min = egui::pos2(rect.min.x + margin_l, rect.min.y + margin_t);
    let plot_max = egui::pos2(rect.max.x - margin_r, rect.max.y - margin_b);
    let plot_w = plot_max.x - plot_min.x;
    let plot_h = plot_max.y - plot_min.y;

    painter.rect_filled(
        egui::Rect::from_min_max(plot_min, plot_max),
        2.0,
        egui::Color32::from_rgb(30, 30, 40),
    );

    let max_conc = data.iter().map(|d| d.oxidant_concentration).fold(0.0_f64, f64::max).max(1.0);
    let max_rfu = data.iter().map(|d| d.fluorescence_signal).fold(0.0_f64, f64::max).max(1.0);

    let control_60: Vec<&ExperimentalDataPoint> = data.iter()
        .filter(|d| d.treatment == "Control" && d.time == 60.0)
        .collect();
    let sod3_60: Vec<&ExperimentalDataPoint> = data.iter()
        .filter(|d| d.treatment == "SOD3" && d.time == 60.0)
        .collect();

    let control_color = egui::Color32::from_rgb(220, 80, 80);
    let sod3_color = egui::Color32::from_rgb(80, 180, 80);

    draw_line_series(&painter, &control_60, max_conc, max_rfu, plot_min, plot_w, plot_h, control_color);
    draw_line_series(&painter, &sod3_60, max_conc, max_rfu, plot_min, plot_w, plot_h, sod3_color);

    painter.text(
        egui::pos2((plot_min.x + plot_max.x) / 2.0, rect.max.y - 2.0),
        egui::Align2::CENTER_BOTTOM,
        "H2O2 Concentration (µM)",
        egui::FontId::proportional(10.0),
        egui::Color32::GRAY,
    );
    painter.text(
        egui::pos2(rect.min.x + 2.0, (plot_min.y + plot_max.y) / 2.0),
        egui::Align2::LEFT_CENTER,
        "RFU",
        egui::FontId::proportional(10.0),
        egui::Color32::GRAY,
    );

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.colored_label(control_color, "— Control (60 min)");
        ui.add_space(10.0);
        ui.colored_label(sod3_color, "— SOD3 (60 min)");
    });
}

fn draw_line_series(
    painter: &egui::Painter,
    points: &[&ExperimentalDataPoint],
    max_conc: f64,
    max_rfu: f64,
    plot_min: egui::Pos2,
    plot_w: f32,
    plot_h: f32,
    color: egui::Color32,
) {
    let mut sorted: Vec<&&ExperimentalDataPoint> = points.iter().collect();
    sorted.sort_by(|a, b| a.oxidant_concentration.partial_cmp(&b.oxidant_concentration).unwrap());

    let screen_pts: Vec<egui::Pos2> = sorted.iter().map(|d| {
        let x = plot_min.x + (d.oxidant_concentration / max_conc) as f32 * plot_w;
        let y = plot_min.y + plot_h - (d.fluorescence_signal / max_rfu) as f32 * plot_h;
        egui::pos2(x, y)
    }).collect();

    for window in screen_pts.windows(2) {
        painter.line_segment([window[0], window[1]], egui::Stroke::new(2.0, color));
    }
    for &pt in &screen_pts {
        painter.circle_filled(pt, 4.0, color);
    }
}

fn load_experimental_csv(path: &std::path::Path) -> anyhow::Result<Vec<ExperimentalDataPoint>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut data = Vec::new();

    for result in reader.records() {
        let record = result?;
        if record.len() < 10 { continue; }

        data.push(ExperimentalDataPoint {
            droplet_id: record[0].to_string(),
            oxidant_concentration: record[2].parse().unwrap_or(0.0),
            treatment: record[3].to_string(),
            time: record[5].parse().unwrap_or(0.0),
            fluorescence_signal: record[7].parse().unwrap_or(0.0),
            viability_score: record[8].parse().unwrap_or(0.0),
            gumol_predicted_damage: record[9].parse().unwrap_or(0.0),
        });
    }

    Ok(data)
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
