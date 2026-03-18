use crate::ui::render_header;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub fn render_nuclera_config(mut contexts: EguiContexts) {
    let mut cartridge_id = String::from("CARTRIDGE_EXP_20240318");
    let mut default_volume = 5.0f32;
    let mut mixing_cycles = 5usize;
    let mut readout_method = String::from("Fluorescence");
    let mut wavelength = 520.0f32;
    let mut exposure_time = 0.1f32;

    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_header(ui, "5. Nuclera Configuration");

        ui.horizontal(|ui| {
            // Left panel: Cartridge configuration
            ui.vertical(|ui| {
                ui.heading("Cartridge Configuration");
                ui.separator();

                ui.add_space(10.0);

                ui.group(|ui| {
                    ui.label("Cartridge ID:");
                    ui.text_edit_singleline(&mut cartridge_id);

                    ui.add_space(10.0);

                    ui.label("Total Droplets:");
                    ui.label("60");

                    ui.add_space(5.0);

                    ui.label("Generation Order:");
                    ui.label("Sequential (D1 → D60)");
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Global Settings");
                    ui.add_space(5.0);

                    ui.horizontal(|ui| {
                        ui.label("Default volume:");
                        ui.add(egui::DragValue::new(&mut default_volume).speed(0.5));
                        ui.label("nL");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Mixing cycles:");
                        ui.add(egui::DragValue::new(&mut mixing_cycles).clamp_range(1..=10));
                    });

                    ui.horizontal(|ui| {
                        ui.label("Mix ratio:");
                        ui.label("1:1:1");
                    });
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Readout Configuration");
                    ui.add_space(5.0);

                    ui.label("Method:");
                    egui::ComboBox::from_id_source("readout_method")
                        .selected_text(readout_method.clone())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut readout_method, String::from("Fluorescence"), "Fluorescence");
                            ui.selectable_value(&mut readout_method, String::from("Absorbance"), "Absorbance");
                            ui.selectable_value(&mut readout_method, String::from("Luminescence"), "Luminescence");
                        });

                    ui.horizontal(|ui| {
                        ui.label("Wavelength:");
                        ui.add(egui::DragValue::new(&mut wavelength).speed(10.0));
                        ui.label("nm");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Exposure time:");
                        ui.add(egui::DragValue::new(&mut exposure_time).speed(0.01));
                        ui.label("s");
                    });
                });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Validate Configuration").clicked() {
                        // Validate the configuration
                    }
                    if ui.button("Export YAML").clicked() {
                        // Export to YAML
                    }
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Preview on Device").clicked() {
                        // Preview on actual device
                    }
                    if ui.button("Send to Nuclera").clicked() {
                        // Send to Nuclera system
                    }
                });
            });

            ui.separator();

            // Right panel: Droplet configurations
            ui.vertical(|ui| {
                ui.heading("Droplet Configurations");
                ui.separator();

                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .max_height(500.0)
                    .show(ui, |ui| {
                        ui.group(|ui| {
                            ui.heading("Sample Droplet Configuration");
                            ui.add_space(5.0);

                            ui.label("Droplet ID: D1");
                            ui.separator();

                            ui.group(|ui| {
                                ui.label("Reagent A (Oxidant)");
                                ui.horizontal(|ui| {
                                    ui.label("Type:");
                                    ui.label("H2O2");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Concentration:");
                                    ui.label("0 µM");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Well:");
                                    ui.label("A01");
                                });
                            });

                            ui.add_space(5.0);

                            ui.group(|ui| {
                                ui.label("Reagent B (Antioxidant)");
                                ui.horizontal(|ui| {
                                    ui.label("Type:");
                                    ui.label("Control");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Concentration:");
                                    ui.label("0 U/mL");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Well:");
                                    ui.label("B01");
                                });
                            });

                            ui.add_space(5.0);

                            ui.group(|ui| {
                                ui.label("Reagent C (Probe)");
                                ui.horizontal(|ui| {
                                    ui.label("Type:");
                                    ui.label("Fluorescent");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Concentration:");
                                    ui.label("10 µM");
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Well:");
                                    ui.label("C01");
                                });
                            });

                            ui.add_space(5.0);

                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.label("Mixing ratio:");
                                ui.label("1:1:1");
                            });
                            ui.horizontal(|ui| {
                                ui.label("Volume:");
                                ui.label("5.0 nL");
                            });
                            ui.horizontal(|ui| {
                                ui.label("Mix cycles:");
                                ui.label("5");
                            });
                            ui.horizontal(|ui| {
                                ui.label("Incubation:");
                                ui.label("300 s");
                            });
                        });

                        ui.add_space(15.0);

                        ui.label("(Additional 59 droplets with varying parameters...)");
                    });
            });
        });

        ui.add_space(20.0);

        // YAML preview
        ui.collapsing("YAML Preview", |ui| {
            ui.monospace(
                r#"cartridge_id: "CARTRIDGE_EXP_20240318"
droplets:
  - droplet_id: "D1"
    reagent_a:
      name: "Oxidant Solution"
      type_name: "H2O2"
      concentration: 0.0
      well_location: "A01"
    reagent_b:
      name: "Antioxidant Sample"
      type_name: "Control"
      concentration: 0.0
      well_location: "B01"
    reagent_c:
      name: "ROS Probe"
      type_name: "Fluorescent"
      concentration: 10.0
      well_location: "C01"
    ratio: [1, 1, 1]
    volume: 5.0
    mixing_cycles: 5
    incubation_time: 300.0
..."#
            );
        });
    });
}
