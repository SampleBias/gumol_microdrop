pub mod eprotein;
pub mod simulation_importer;
pub mod card_editor;
pub mod parameter_mapping;
pub mod experiment_design;
pub mod nuclera_config;
pub mod protocol_generator;
pub mod data_viewer;

use bevy_egui::egui;
use crate::data_models::{DropletCondition, ColorMode};

pub fn render_header(ui: &mut egui::Ui, title: &str) {
    ui.heading(title);
    ui.separator();
}

pub fn droplet_color(condition: &DropletCondition, mode: ColorMode, max_conc: f64) -> egui::Color32 {
    match mode {
        ColorMode::Oxidant => {
            let t = (condition.oxidant_concentration / max_conc.max(1.0)).min(1.0) as f32;
            let g = (255.0 * (1.0 - t * 0.65)) as u8;
            let b = (255.0 * (1.0 - t * 0.65)) as u8;
            egui::Color32::from_rgb(255, g, b)
        }
        ColorMode::Antioxidant => {
            if condition.antioxidant_concentration == 0.0 {
                egui::Color32::from_rgb(230, 230, 230)
            } else {
                let t = (condition.antioxidant_concentration / 100.0).min(1.0) as f32;
                let r = (255.0 * (1.0 - t * 0.45)) as u8;
                let b = (255.0 * (1.0 - t * 0.45)) as u8;
                egui::Color32::from_rgb(r, 255, b)
            }
        }
        ColorMode::ExposureTime => {
            let t = (condition.exposure_time / 60.0).min(1.0) as f32;
            let r = (220.0 * (1.0 - t)) as u8;
            let g = (220.0 * (1.0 - t)) as u8;
            egui::Color32::from_rgb(r, g, 255)
        }
        ColorMode::Treatment => {
            match condition.antioxidant.to_lowercase().as_str() {
                "control" => egui::Color32::from_rgb(210, 210, 230),
                "sod3" => egui::Color32::from_rgb(180, 240, 180),
                "catalase" => egui::Color32::from_rgb(240, 210, 180),
                "gpx" => egui::Color32::from_rgb(180, 210, 240),
                _ => egui::Color32::from_rgb(230, 230, 230),
            }
        }
    }
}

pub fn empty_cell_color() -> egui::Color32 {
    egui::Color32::from_rgb(220, 220, 220)
}
