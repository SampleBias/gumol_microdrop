use bevy_egui::egui;
use crate::data_models::{ApplicationState, EproteinNavPanel};

pub fn render_export_protocol(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::super::render_header(ui, "eProtein — 3. Export & express/purify protocol");
        ui.horizontal(|ui| {
            if ui.button("← Screen matrix").clicked() {
                state.eprotein_panel = EproteinNavPanel::ScreenMatrix;
            }
            if ui.button("Next: Results →").clicked() {
                state.eprotein_panel = EproteinNavPanel::Results;
            }
        });
        ui.add_space(10.0);
        ui.label("Phase 3: Nuclera-aligned YAML + Markdown protocol (eGene prep → cartridge → expression → purification ranking).");
        ui.label("Will reuse / extend existing YAML generators once schema is validated against Nuclera exports.");
    });
}

pub fn render_results(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::super::render_header(ui, "eProtein — 4. Results import & ranking");
        ui.horizontal(|ui| {
            if ui.button("← Export").clicked() {
                state.eprotein_panel = EproteinNavPanel::ExportProtocol;
            }
            if ui.button("Next: Scale-up →").clicked() {
                state.eprotein_panel = EproteinNavPanel::ScaleUp;
            }
        });
        ui.add_space(10.0);
        ui.label("Phase 4: Import expression / purification CSV or JSON from Nuclera cloud export; rank top expressors.");
    });
}

pub fn render_scale_up(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::super::render_header(ui, "eProtein — 5. Scale-up");
        ui.horizontal(|ui| {
            if ui.button("← Results").clicked() {
                state.eprotein_panel = EproteinNavPanel::Results;
            }
        });
        ui.add_space(10.0);
        ui.label("Phase 4: Select winning construct + blend; µL–mL scale-up checklist and notes.");
    });
}
