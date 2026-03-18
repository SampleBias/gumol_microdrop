pub mod simulation_importer;
pub mod card_editor;
pub mod parameter_mapping;
pub mod experiment_design;
pub mod nuclera_config;
pub mod protocol_generator;
pub mod data_viewer;

use bevy_egui::egui;
use crate::ApplicationState;
use crate::data_models::CardDesign;

pub fn render_header(ui: &mut egui::Ui, title: &str) {
    ui.heading(title);
    ui.separator();
}

pub fn render_button_enabled(ui: &mut egui::Ui, text: &str, enabled: bool) -> bool {
    ui.add_enabled(enabled, egui::Button::new(text)).clicked()
}
