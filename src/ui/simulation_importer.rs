use crate::ui::render_header;
use crate::{CurrentPanel, modules::SimulationImporter};
use bevy_egui::{egui, EguiContexts};
use rfd::FileDialog;

pub struct SimulationImporterPanel;
pub struct SimulationImporterContext;

impl SimulationImporterPanel {
    pub fn render(
        _context: SimulationImporterContext,
        mut contexts: EguiContexts,
        current_panel: &mut CurrentPanel,
    ) {
        egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
            render_header(ui, "1. Simulation Import");

            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.label("Import Gumol Simulation Data");
                ui.add_space(10.0);

                if ui.button("📁 Select Simulation File").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("JSON", &["json"])
                        .add_filter("CSV", &["csv"])
                        .pick_file()
                    {
                        // File selected - in a real app, we would load it here
                        eprintln!("Selected file: {:?}", path);

                        // Move to card editor
                        *current_panel = CurrentPanel::CardEditor;
                    }
                }

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.heading("Supported Formats");
                    ui.label("• JSON (recommended)");
                    ui.label("• CSV");
                    ui.label("• HDF5 (coming soon)");
                });

                ui.add_space(20.0);

                ui.group(|ui| {
                    ui.heading("Expected Data Fields");
                    ui.monospace("{
  simulation_id: string,
  radical_species: array,
  time_series: array,
  diffusion_constants: object,
  reaction_rates: object,
  damage_threshold: float
}");
                });

                ui.add_space(20.0);

                if ui.button("Create Sample Data").clicked() {
                    // In a real app, this would generate sample data
                    *current_panel = CurrentPanel::CardEditor;
                }

                ui.add_space(20.0);

                ui.label("Or proceed directly to Card Editor:");
                if ui.button("→ Skip to Card Editor").clicked() {
                    *current_panel = CurrentPanel::CardEditor;
                }
            });
        });
    }
}
