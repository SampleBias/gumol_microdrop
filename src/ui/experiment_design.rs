use crate::ui::render_header;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub fn render_experiment_design(mut contexts: EguiContexts) {
    let mut rows = 8usize;
    let mut cols = 12usize;
    let mut min_replicates = 3usize;
    let mut ensure_stats = true;
    let mut minimize_usage = false;

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_header(ui, "4. Experiment Design");

        ui.horizontal(|ui| {
            // Left panel: Matrix configuration
            ui.vertical(|ui| {
                ui.heading("Droplet Matrix Configuration");
                ui.separator();

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Matrix Dimensions");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Rows:");
                        ui.add(egui::DragValue::new(&mut rows).clamp_range(1..=16));
                        ui.label("Cols:");
                        ui.add(egui::DragValue::new(&mut cols).clamp_range(1..=24));
                    });

                    ui.label(format!("Total droplets: {}", rows * cols));
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.label("Optimization Constraints");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Nuclera capacity:");
                        ui.label("96 droplets");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Min replicates:");
                        ui.add(egui::DragValue::new(&mut min_replicates).clamp_range(1..=10));
                    });

                    ui.checkbox(&mut ensure_stats, "Ensure statistical power");
                    ui.checkbox(&mut minimize_usage, "Minimize reagent usage");
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.label("Design Factors");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Oxidant levels:");
                        ui.label("5");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Antioxidant conditions:");
                        ui.label("3");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Time points:");
                        ui.label("4");
                    });

                    ui.label(format!("Full factorial: {}", 5 * 3 * 4));
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Generate Matrix").clicked() {
                        // Generate the experiment matrix
                    }
                    if ui.button("Optimize for Cartridge").clicked() {
                        // Optimize for cartridge constraints
                    }
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Matrix Statistics");
                    ui.add_space(5.0);

                    ui.label("Total conditions: 60");
                    ui.label("Reagent volume: ~300 nL");
                    ui.label("Estimated time: 2.5 hours");
                });
            });

            ui.separator();

            // Right panel: Matrix preview
            ui.vertical(|ui| {
                ui.heading("Matrix Preview");
                ui.separator();

                ui.add_space(10.0);

                // Sample matrix table
                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        egui::Grid::new("matrix_preview")
                            .num_columns(4)
                            .spacing([10.0, 5.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Droplet");
                                ui.label("Oxidant");
                                ui.label("Antioxidant");
                                ui.label("Time");
                                ui.end_row();

                                // Sample data
                                for i in 1..=20 {
                                    let oxidant = match i {
                                        1 => "0 µM",
                                        2..=5 => "10 µM",
                                        6..=10 => "50 µM",
                                        11..=15 => "100 µM",
                                        _ => "250 µM",
                                    };

                                    let antioxidant = match i % 3 {
                                        0 => "Control",
                                        1 => "SOD3",
                                        _ => "Catalase",
                                    };

                                    let time = match i % 4 {
                                        0 => "5m",
                                        1 => "10m",
                                        2 => "30m",
                                        _ => "60m",
                                    };

                                    ui.label(format!("D{}", i));
                                    ui.label(oxidant);
                                    ui.label(antioxidant);
                                    ui.label(time);
                                    ui.end_row();
                                }
                            });
                    });

                ui.add_space(15.0);

                ui.horizontal(|ui| {
                    if ui.button("Export Matrix (CSV)").clicked() {
                        // Export to CSV
                    }
                    if ui.button("View Full Matrix").clicked() {
                        // Show full matrix
                    }
                });
            });
        });
    });
}
