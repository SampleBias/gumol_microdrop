use crate::ui::render_header;
use crate::data_models::DropletCondition;
use bevy_egui::{egui, EguiContexts};
use crate::ApplicationState;

pub struct CardEditorPanel;
pub struct CardEditorContext;

impl CardEditorPanel {
    pub fn render(
        _context: CardEditorContext,
        mut contexts: EguiContexts,
        app_state: &ApplicationState,
    ) {
        let mut card_name = String::from("My Experiment Card");
        let mut rows = 8usize;
        let mut cols = 12usize;
        let mut oxidant_type = String::from("H2O2");
        let mut min_conc = 0.0f32;
        let mut max_conc = 250.0f32;
        let mut control_enabled = true;
        let mut sod3_enabled = true;
        let mut catalase_enabled = false;
        let mut gpx_enabled = false;
        let mut exposure_times = [5.0f32, 10.0, 30.0, 60.0];

        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            render_header(ui, "2. Card Editor");

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Card Configuration");
                    ui.separator();

                    ui.label("Card Name:");
                    ui.text_edit_singleline(&mut card_name);

                    ui.add_space(10.0);

                    ui.label("Grid Dimensions:");
                    ui.horizontal(|ui| {
                        ui.label("Rows:");
                        ui.add(egui::DragValue::new(&mut rows).clamp_range(1..=16));
                        ui.label("Cols:");
                        ui.add(egui::DragValue::new(&mut cols).clamp_range(1..=24));
                    });

                    ui.add_space(10.0);

                    ui.label(format!("Total Wells: {}", rows * cols));

                    ui.add_space(20.0);

                    ui.group(|ui| {
                        ui.heading("Droplet Parameters");
                        ui.separator();

                        ui.label("Oxidant Type:");
                        egui::ComboBox::from_id_source("oxidant_type")
                            .selected_text(oxidant_type.clone())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut oxidant_type, String::from("H2O2"), "H2O2");
                                ui.selectable_value(&mut oxidant_type, String::from("O2-"), "O2-");
                                ui.selectable_value(&mut oxidant_type, String::from("OH"), "OH");
                            });

                        ui.add_space(10.0);

                        ui.label("Concentration Range (µM):");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut min_conc).speed(1.0));
                            ui.label("to");
                            ui.add(egui::DragValue::new(&mut max_conc).speed(10.0));
                        });

                        ui.add_space(10.0);

                        ui.label("Antioxidants:");
                        ui.checkbox(&mut control_enabled, "Control");
                        ui.checkbox(&mut sod3_enabled, "SOD3");
                        ui.checkbox(&mut catalase_enabled, "Catalase");
                        ui.checkbox(&mut gpx_enabled, "GPx");

                        ui.add_space(10.0);

                        ui.label("Exposure Times (min):");
                        ui.horizontal(|ui| {
                            ui.add(egui::DragValue::new(&mut exposure_times[0]).speed(1.0));
                            ui.add(egui::DragValue::new(&mut exposure_times[1]).speed(1.0));
                            ui.add(egui::DragValue::new(&mut exposure_times[2]).speed(5.0));
                            ui.add(egui::DragValue::new(&mut exposure_times[3]).speed(5.0));
                        });
                    });

                    ui.add_space(20.0);

                    ui.horizontal(|ui| {
                        if ui.button("Generate Matrix").clicked() {
                            // Generate the droplet matrix
                        }
                        if ui.button("Export Design").clicked() {
                            // Export the design
                        }
                    });
                });

                ui.separator();

                // Visual Grid Display
                ui.vertical(|ui| {
                    ui.heading("Card Preview");
                    ui.separator();

                    let grid_size = 45.0;
                    let spacing = 2.0;

                    egui::ScrollArea::both()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(10.0);

                                // Row labels
                                ui.horizontal(|ui| {
                                    ui.add_space(20.0);
                                    for col in 1..=12 {
                                        ui.label(format!("{}", col));
                                        ui.add_space(grid_size - 10.0);
                                    }
                                });

                                for row in 0..8 {
                                    ui.horizontal(|ui| {
                                        ui.label(format!("{}", (b'A' + row as u8) as char));
                                        ui.add_space(10.0);

                                        for col in 0..12 {
                                            let index = row * 12 + col;
                                            let (color, tooltip) = Self::get_droplet_color(index, row, col);

                                            let response = ui.add_sized(
                                                [grid_size, grid_size],
                                                egui::Button::new("")
                                                    .fill(color)
                                                    .stroke(egui::Stroke::new(1.0, egui::Color32::DARK_GRAY))
                                            );

                                            if response.hovered() {
                                                // Show tooltip using response's tooltip method
                                                response.show_tooltip_text(tooltip);
                                            }
                                        }
                                    });
                                }

                                ui.add_space(20.0);

                                ui.separator();
                                ui.add_space(10.0);

                                // Legend
                                ui.heading("Legend");
                                ui.horizontal(|ui| {
                                    Self::legend_item(ui, egui::Color32::from_rgb(240, 240, 240), "Control");
                                    Self::legend_item(ui, egui::Color32::from_rgb(255, 200, 200), "Low Oxidant");
                                    Self::legend_item(ui, egui::Color32::from_rgb(255, 150, 150), "Medium Oxidant");
                                    Self::legend_item(ui, egui::Color32::from_rgb(255, 100, 100), "High Oxidant");
                                });

                                ui.horizontal(|ui| {
                                    Self::legend_item(ui, egui::Color32::from_rgb(200, 255, 200), "Low Antioxidant");
                                    Self::legend_item(ui, egui::Color32::from_rgb(150, 255, 150), "Med Antioxidant");
                                    Self::legend_item(ui, egui::Color32::from_rgb(100, 255, 100), "High Antioxidant");
                                });
                            });
                        });
                });
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Selected Droplet Info:");
                ui.separator();
                ui.label("Click on a droplet in the grid to edit its parameters");
            });
        });
    }

    fn get_droplet_color(index: usize, row: usize, col: usize) -> (egui::Color32, &'static str) {
        // Generate example pattern based on position
        let col_factor = (col % 5) as f32 / 4.0;
        let row_factor = (row % 3) as f32 / 2.0;

        let (r, g, b, text) = if index < 12 {
            // First row: controls
            (240, 240, 240, "Control: 0 µM H2O2")
        } else if index < 36 {
            // Rows 2-3: varying oxidant
            let intensity = 255 - ((index - 12) as f32 * 5.0) as u8;
            (255, 200.min(intensity).max(100), 200.min(intensity).max(100), "Oxidant concentration varying")
        } else if index < 60 {
            // Rows 4-5: with antioxidant
            let green = 150 + ((col * 20) as u8);
            (200, 255.min(green).max(150), 200, "With SOD3 antioxidant")
        } else if index < 84 {
            // Rows 6-7: mixed conditions
            let r = 200 + (col_factor * 50.0) as u8;
            let g = 200 + (row_factor * 50.0) as u8;
            (255.min(r), 255.min(g), 200, "Mixed treatment condition")
        } else {
            // Last row: high oxidant + high antioxidant
            (255, 255, 180, "High concentration condition")
        };

        (egui::Color32::from_rgb(r, g, b), text)
    }

    fn legend_item(ui: &mut egui::Ui, color: egui::Color32, text: &str) {
        ui.horizontal(|ui| {
            let rect = egui::Rect::from_min_size(ui.cursor().min, egui::vec2(20.0, 20.0));
            ui.painter().rect_filled(rect, 2.0, color);
            ui.add_space(25.0);
            ui.label(text);
        });
        ui.add_space(10.0);
    }
}
