use crate::ui::render_header;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub fn render_parameter_mapping(mut contexts: EguiContexts) {
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        render_header(ui, "3. Parameter Mapping");

        ui.vertical(|ui| {
            ui.heading("Simulation → Experimental Parameters");
            ui.separator();

            ui.add_space(20.0);

            // Mapping table
            egui::Grid::new("parameter_mapping_grid")
                .num_columns(3)
                .spacing([20.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Gumol Parameter");
                    ui.label("→");
                    ui.label("Wet Lab Variable");
                    ui.end_row();

                    ui.label("Superoxide density");
                    ui.label("→");
                    ui.label("Oxidant concentration");
                    ui.end_row();

                    ui.label("Diffusion coefficient");
                    ui.label("→");
                    ui.label("Droplet mixing ratio");
                    ui.end_row();

                    ui.label("Reaction rate");
                    ui.label("→");
                    ui.label("Exposure time");
                    ui.end_row();

                    ui.label("Antioxidant neutralization");
                    ui.label("→");
                    ui.label("Enzyme concentration");
                    ui.end_row();

                    ui.label("Damage threshold");
                    ui.label("→");
                    ui.label("ROS reporter signal");
                    ui.end_row();
                });

            ui.add_space(30.0);

            ui.heading("Experimental Ranges");
            ui.separator();

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Oxidant concentration:");
                    ui.label("[0, 10, 50, 100, 250] µM");
                });

                ui.horizontal(|ui| {
                    ui.label("Exposure time:");
                    ui.label("[5, 10, 30, 60] min");
                });

                ui.horizontal(|ui| {
                    ui.label("Antioxidant dose:");
                    ui.label("[0, 10, 50, 100] U/mL");
                });
            });

            ui.add_space(30.0);

            ui.group(|ui| {
                ui.heading("Mapping Formula");
                ui.label("experimental_range = f(simulation_parameter_distribution)");
                ui.add_space(10.0);
                ui.label("Example:");
                ui.monospace("oxidant_concentration = mean_superoxide_density × 100");
            });

            ui.add_space(20.0);

            if ui.button("Apply Mappings").clicked() {
                // Apply the parameter mappings
            }
        });
    });
}
