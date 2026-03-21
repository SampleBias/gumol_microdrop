use bevy_egui::egui;
use crate::data_models::{ApplicationState, EproteinNavPanel};
use crate::modules::EproteinScreenDesigner;

pub fn render_screen_matrix(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::super::render_header(ui, "eProtein — 2. Expression screen (microdrop map)");

        ui.horizontal(|ui| {
            if ui.button("← Back to constructs").clicked() {
                state.eprotein_panel = EproteinNavPanel::ConstructDesign;
            }
            if ui.button("Regenerate 192/88 grid").clicked() {
                let matrix = EproteinScreenDesigner::build_screen(
                    state.eprotein.screen_kind,
                    &state.eprotein.constructs,
                    &state.eprotein.blends,
                );
                let n = matrix.slots.len();
                state.eprotein.screen = Some(matrix);
                state.status_message = format!("Screen: {n} expression slots");
            }
            if ui.button("Next: Export / protocol →").clicked() {
                state.eprotein_panel = EproteinNavPanel::ExportProtocol;
            }
        });

        ui.add_space(8.0);
        ui.label("Row-major: each row is one construct slot; columns are blends 1–8 (Nuclera-style 24×8 or 11×8).");

        if let Some(screen) = &state.eprotein.screen {
            let n_blends = 8usize;
            let n_rows = screen.slots.len() / n_blends;

            ui.label(format!(
                "{} — {} slots ({} constructs × 8 blends)",
                screen.kind.label(),
                screen.slots.len(),
                n_rows
            ));

            ui.add_space(6.0);
            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let cell = 22.0_f32;
                    ui.horizontal(|ui| {
                        ui.add_space(28.0);
                        for b in 1..=n_blends {
                            ui.add_sized([cell, 16.0], egui::Label::new(egui::RichText::new(format!("B{}", b)).small()));
                        }
                    });

                    for row in 0..n_rows {
                        ui.horizontal(|ui| {
                            ui.add_sized([26.0, cell], egui::Label::new(egui::RichText::new(format!("C{}", row + 1)).small().strong()));
                            for col in 0..n_blends {
                                let idx = row * n_blends + col;
                                if let Some(slot) = screen.slots.get(idx) {
                                    let color = if slot.blend_enabled {
                                        egui::Color32::from_rgb(80, 140, 200)
                                    } else {
                                        egui::Color32::from_rgb(120, 120, 120)
                                    };
                                    let tip = format!(
                                        "Slot {}\n{}\n{}",
                                        slot.slot_index, slot.construct_id, slot.blend_name
                                    );
                                    let resp = ui.add_sized(
                                        [cell, cell],
                                        egui::Button::new("")
                                            .fill(color)
                                            .stroke(egui::Stroke::new(1.0, egui::Color32::DARK_GRAY)),
                                    );
                                    resp.on_hover_text(tip);
                                }
                            }
                        });
                    }
                });
        } else {
            ui.add_space(20.0);
            ui.label("Click “Regenerate 192/88 grid” to build the matrix from constructs and blends.");
        }

        ui.add_space(12.0);
        ui.collapsing("Cell-free blends (8)", |ui| {
            for b in state.eprotein.blends.iter_mut() {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut b.enabled, "");
                    ui.label(format!("{}:", b.blend_index + 1));
                    ui.text_edit_singleline(&mut b.name);
                });
                ui.text_edit_multiline(&mut b.description);
            }
        });
    });
}
