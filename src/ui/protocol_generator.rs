use bevy_egui::egui;
use crate::data_models::ApplicationState;
use crate::modules::ProtocolGenerator;

pub fn render(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::render_header(ui, "6. Protocol Generator");

        let can_generate = state.droplet_matrix.is_some() && state.nuclera_config.is_some();

        if !can_generate {
            ui.label("Generate a droplet matrix and Nuclera config first (steps 2-5).");
            return;
        }

        ui.horizontal(|ui| {
            if ui.button("Generate Protocol").clicked() {
                if let (Some(matrix), Some(config)) =
                    (&state.droplet_matrix, &state.nuclera_config)
                {
                    let protocol = ProtocolGenerator::generate_protocol(matrix, config);
                    state.status_message = format!(
                        "Protocol generated: {} steps",
                        protocol.steps.len()
                    );
                    state.protocol = Some(protocol);
                }
            }

            if state.protocol.is_some() {
                if ui.button("Export Markdown").clicked() {
                    if let Some(protocol) = &state.protocol {
                        let md = ProtocolGenerator::export_to_markdown(protocol);
                        if let Some(path) = rfd::FileDialog::new()
                            .set_file_name("protocol.md")
                            .add_filter("Markdown", &["md"])
                            .save_file()
                        {
                            match std::fs::write(&path, &md) {
                                Ok(_) => state.status_message = format!("Saved to {}", path.display()),
                                Err(e) => state.status_message = format!("Write error: {}", e),
                            }
                        }
                    }
                }
            }
        });

        ui.add_space(10.0);

        if let Some(protocol) = &state.protocol {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.heading(format!("Protocol: {}", protocol.experiment_id));
                    ui.add_space(6.0);

                    ui.group(|ui| {
                        ui.strong("Equipment");
                        for item in &protocol.equipment {
                            ui.label(format!("  {}", item));
                        }
                    });

                    ui.add_space(6.0);

                    ui.group(|ui| {
                        ui.strong("Reagents");
                        for item in &protocol.required_reagents {
                            ui.label(format!("  {}", item));
                        }
                    });

                    ui.add_space(10.0);
                    ui.heading("Procedure");
                    ui.separator();

                    for step in &protocol.steps {
                        ui.add_space(4.0);
                        ui.group(|ui| {
                            ui.strong(format!("Step {}: {}", step.step_number, step.description));
                            if let Some(dur) = step.duration {
                                ui.label(format!("Duration: {:.1} min", dur));
                            }
                            for detail in &step.details {
                                ui.label(format!("  - {}", detail));
                            }
                        });
                    }

                    ui.add_space(10.0);
                    let total: f64 = protocol.steps.iter().filter_map(|s| s.duration).sum();
                    ui.strong(format!("Total estimated time: {:.1} min ({:.1} hours)", total, total / 60.0));
                });
        } else {
            ui.label("Click 'Generate Protocol' to create the wet-lab protocol.");
        }
    });
}
