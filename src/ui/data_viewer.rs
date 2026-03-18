use crate::ui::render_header;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub fn render_data_viewer(mut contexts: EguiContexts) {
    let mut show_prediction = true;
    let mut show_error_dist = true;
    let mut show_residual = false;
    let mut show_confidence = false;
    let mut selected_viz = 0usize;

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_header(ui, "7. Data Viewer & Correlation Analysis");

        ui.horizontal(|ui| {
            // Left panel: Data input and analysis
            ui.vertical(|ui| {
                ui.heading("Data Import");
                ui.separator();

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("📁 Load Experimental Data").clicked() {
                        // Load experimental data
                    }
                    if ui.button("📊 Load Simulation Data").clicked() {
                        // Load simulation data
                    }
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Correlation Analysis");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("RMSE:");
                        ui.label("0.234");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Pearson Correlation:");
                        ui.label("0.876");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Threshold Accuracy:");
                        ui.label("92.3%");
                    });

                    ui.add_space(10.0);

                    ui.separator();
                    ui.add_space(10.0);

                    ui.label("Simulation Error Distribution:");
                    ui.label("Mean: 0.12 ± 0.08");

                    ui.add_space(5.0);

                    ui.label("Experimental Variance:");
                    ui.label("0.156");
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Analysis Options");
                    ui.add_space(5.0);

                    ui.checkbox(&mut show_prediction, "Show prediction vs measurement");
                    ui.checkbox(&mut show_error_dist, "Show error distribution");
                    ui.checkbox(&mut show_residual, "Show residual plot");
                    ui.checkbox(&mut show_confidence, "Show confidence intervals");

                    ui.add_space(10.0);

                    if ui.button("Run Correlation Analysis").clicked() {
                        // Run analysis
                    }

                    if ui.button("Export Report (PDF)").clicked() {
                        // Export report
                    }
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Data Export");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        if ui.button("Export CSV").clicked() {
                            // Export to CSV
                        }
                        if ui.button("Export JSON").clicked() {
                            // Export to JSON
                        }
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Export Parquet").clicked() {
                            // Export to Parquet
                        }
                        if ui.button("Export Excel").clicked() {
                            // Export to Excel
                        }
                    });
                });
            });

            ui.separator();

            // Right panel: Visualization
            ui.vertical(|ui| {
                ui.heading("Visualizations");
                ui.separator();

                ui.add_space(10.0);

                // Tabs for different visualizations
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut selected_viz, 0usize, "Prediction vs Measured");
                    ui.selectable_value(&mut selected_viz, 1usize, "Error Dist.");
                    ui.selectable_value(&mut selected_viz, 2usize, "Heatmap");
                });

                ui.add_space(10.0);

                // Simple visualization placeholder
                ui.group(|ui| {
                    ui.heading("Prediction vs Experimental Measurements");
                    ui.add_space(10.0);

                    let painter = ui.painter();
                    let min = ui.cursor().min;
                    let max_x = min.x + 500.0;
                    let max_y = min.y + 400.0;

                    // Draw axes
                    let origin = egui::pos2(min.x + 40.0, min.y + 360.0);
                    let x_max = min.x + 480.0;
                    let y_max = min.y + 40.0;

                    // X-axis
                    painter.line_segment(
                        [origin, egui::pos2(x_max, origin.y)],
                        (2.0, egui::Color32::GRAY),
                    );

                    // Y-axis
                    painter.line_segment(
                        [origin, egui::pos2(origin.x, y_max)],
                        (2.0, egui::Color32::GRAY),
                    );

                    // Labels
                    painter.text(
                        egui::pos2(min.x + 260.0, min.y + 375.0),
                        egui::Align2::CENTER_TOP,
                        "Gumol Prediction",
                        egui::FontId::default(),
                        egui::Color32::GRAY,
                    );

                    painter.text(
                        egui::pos2(min.x + 10.0, min.y + 200.0),
                        egui::Align2::LEFT_CENTER,
                        "Experiment",
                        egui::FontId::default(),
                        egui::Color32::GRAY,
                    );

                    // Draw diagonal line (perfect correlation)
                    painter.line_segment(
                        [egui::pos2(origin.x, origin.y), egui::pos2(x_max, y_max)],
                        (1.0, egui::Color32::LIGHT_GRAY),
                    );

                    // Draw data points
                    let plot_width = x_max - origin.x;
                    let plot_height = origin.y - y_max;

                    for i in 0..20 {
                        let x = (i as f32) / 20.0;
                        let y = x + 0.1 * (i as f32 / 20.0 - 0.5).sin();

                        let px = origin.x + x * plot_width;
                        let py = origin.y - y * plot_height;

                        let color = if (y - x).abs() < 0.1 {
                            egui::Color32::GREEN
                        } else {
                            egui::Color32::RED
                        };

                        painter.circle_filled(egui::pos2(px, py), 4.0, color);
                    }

                    ui.add_space(400.0);
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Legend");
                    ui.horizontal(|ui| {
                        let painter = ui.painter();
                        let pos = ui.cursor().min;

                        painter.circle_filled(egui::pos2(pos.x + 10.0, pos.y + 10.0), 4.0, egui::Color32::GREEN);
                        ui.label("High accuracy (|error| < 0.1)");
                    });

                    ui.horizontal(|ui| {
                        let painter = ui.painter();
                        let pos = ui.cursor().min;

                        painter.circle_filled(egui::pos2(pos.x + 10.0, pos.y + 10.0), 4.0, egui::Color32::RED);
                        ui.label("Low accuracy (|error| ≥ 0.1)");
                    });
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Data Summary");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Total data points:");
                        ui.label("20");
                    });

                    ui.horizontal(|ui| {
                        ui.label("High accuracy points:");
                        ui.label("16 (80%)");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Low accuracy points:");
                        ui.label("4 (20%)");
                    });
                });
            });
        });
    });
}
