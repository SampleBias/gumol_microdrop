use crate::ui::render_header;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub fn render_protocol_generator(mut contexts: EguiContexts) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_header(ui, "6. Protocol Generator");

        ui.horizontal(|ui| {
            // Left panel: Protocol steps
            ui.vertical(|ui| {
                ui.heading("Experiment Protocol");
                ui.separator();

                ui.add_space(10.0);

                ui.label("Experiment ID: EXP_20240318_143022");
                ui.label("Generated: 2024-03-18 14:30:22");

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Required Equipment");
                    ui.add_space(5.0);
                    ui.label("• Nuclera eProtein Discovery System");
                    ui.label("• Digital microfluidics cartridge");
                    ui.label("• Fluorescence detector");
                    ui.label("• Temperature controller");
                });

                ui.add_space(15.0);

                ui.group(|ui| {
                    ui.heading("Required Reagents");
                    ui.add_space(5.0);
                    ui.label("• Hydrogen peroxide solutions (various concentrations)");
                    ui.label("• Superoxide Dismutase 3 (SOD3)");
                    ui.label("• Catalase");
                    ui.label("• PBS buffer");
                    ui.label("• ROS fluorescent probe (e.g., H2DCFDA)");
                });

                ui.add_space(20.0);

                ui.heading("Procedure");
                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(400.0)
                    .show(ui, |ui| {
                        protocol_step(ui, 1, "Load reagents into Nuclera cartridge wells", Some(10.0), &[
                            "Oxidant solutions in Row A (concentrations: 0-250 µM)",
                            "Antioxidant samples in Row B",
                            "ROS fluorescent probe in Row C",
                        ]);

                        protocol_step(ui, 2, "Initialize droplet routing program", Some(2.0), &[
                            "Load cartridge configuration file",
                            "Configuration ID: CARTRIDGE_EXP_20240318",
                            "Total droplets: 60",
                        ]);

                        protocol_step(ui, 3, "Generate droplets according to experiment matrix", Some(30.0), &[
                            "Generate 60 droplets sequentially",
                            "Mix reagents in 1:1:1 ratio",
                            "5 mixing cycles per droplet",
                            "Target volume: 5 nL per droplet",
                        ]);

                        protocol_step(ui, 4, "Incubate droplets for exposure time", Some(60.0), &[
                            "Incubation times range from 5 to 60 minutes",
                            "Temperature: 37°C",
                            "Monitor droplet stability",
                        ]);

                        protocol_step(ui, 5, "Measure fluorescence output", Some(2.0), &[
                            "Detection wavelength: 520 nm",
                            "Exposure time: 0.1 s",
                            "Record droplet-level fluorescence intensity",
                        ]);

                        protocol_step(ui, 6, "Export droplet-level data", Some(1.0), &[
                            "Export fluorescence data to CSV",
                            "Save experiment metadata",
                            "Archive configuration files",
                        ]);
                    });

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    if ui.button("Export Protocol (Markdown)").clicked() {
                        // Export to markdown
                    }
                    if ui.button("Export Protocol (PDF)").clicked() {
                        // Export to PDF
                    }
                    if ui.button("Print Protocol").clicked() {
                        // Print protocol
                    }
                });
            });

            ui.separator();

            // Right panel: Protocol preview
            ui.vertical(|ui| {
                ui.heading("Protocol Preview");
                ui.separator();

                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .max_height(600.0)
                    .show(ui, |ui| {
                        ui.monospace(
                            r#"# Experiment Protocol: EXP_20240318_143022

Generated: 2024-03-18 14:30:22

## Required Equipment

- Nuclera eProtein Discovery System
- Digital microfluidics cartridge
- Fluorescence detector
- Temperature controller

## Required Reagents

- Hydrogen peroxide solutions (various concentrations)
- Superoxide Dismutase 3 (SOD3)
- Catalase
- PBS buffer
- ROS fluorescent probe (e.g., H2DCFDA)

## Procedure

### Step 1: Load reagents into Nuclera cartridge wells

**Estimated duration:** 10.0 minutes

- Oxidant solutions in Row A (concentrations: 0-250 µM)
- Antioxidant samples in Row B
- ROS fluorescent probe in Row C

### Step 2: Initialize droplet routing program

**Estimated duration:** 2.0 minutes

- Load cartridge configuration file
- Configuration ID: CARTRIDGE_EXP_20240318
- Total droplets: 60

### Step 3: Generate droplets according to experiment matrix

**Estimated duration:** 30.0 minutes

- Generate 60 droplets sequentially
- Mix reagents in 1:1:1 ratio
- 5 mixing cycles per droplet
- Target volume: 5 nL per droplet

### Step 4: Incubate droplets for exposure time

**Estimated duration:** 60.0 minutes

- Incubation times range from 5 to 60 minutes
- Temperature: 37°C
- Monitor droplet stability

### Step 5: Measure fluorescence output

**Estimated duration:** 2.0 minutes

- Detection wavelength: 520 nm
- Exposure time: 0.1 s
- Record droplet-level fluorescence intensity

### Step 6: Export droplet-level data

**Estimated duration:** 1.0 minutes

- Export fluorescence data to CSV
- Save experiment metadata
- Archive configuration files

---
*Total estimated duration: ~2.1 hours*"#
                        );
                    });
            });
        });
    });
}

fn protocol_step(
    ui: &mut egui::Ui,
    step_number: usize,
    description: &str,
    duration: Option<f64>,
    details: &[&str],
) {
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.heading(format!("Step {}", step_number));
            ui.label(description);
        });

        if let Some(dur) = duration {
            ui.label(format!("Estimated duration: {:.1} minutes", dur));
        }

        ui.add_space(5.0);

        for detail in details {
            ui.label(format!("• {}", detail));
        }
    });

    ui.add_space(10.0);
}
