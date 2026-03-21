use bevy_egui::egui;
use crate::data_models::{ApplicationState, EproteinNavPanel};
use crate::eprotein::models::{Construct, DnaTopology, EproteinScreenKind, NUCLERA_MEMBRANE_N_CONSTRUCTS, NUCLERA_SOLUBLE_N_CONSTRUCTS};

pub fn render_construct_design(ctx: &egui::Context, state: &mut ApplicationState) {
    egui::CentralPanel::default().show(ctx, |ui| {
        super::super::render_header(ui, "eProtein — 1. Construct design");

        ui.label("Define DNA/protein constructs for the expression screen. Prediction APIs will attach here in Phase 1.");
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.label("Project:");
            ui.text_edit_singleline(&mut state.eprotein.project_name);
        });

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            ui.label("Screen type:");
            egui::ComboBox::from_id_source("eprotein_screen_kind")
                .selected_text(state.eprotein.screen_kind.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut state.eprotein.screen_kind,
                        EproteinScreenKind::SolubleProtein,
                        EproteinScreenKind::SolubleProtein.label(),
                    );
                    ui.selectable_value(
                        &mut state.eprotein.screen_kind,
                        EproteinScreenKind::MembraneProtein,
                        EproteinScreenKind::MembraneProtein.label(),
                    );
                });
        });

        ui.add_space(10.0);
        ui.horizontal(|ui| {
            if ui.button("Add construct").clicked() {
                let n = state.eprotein.constructs.len() + 1;
                state.eprotein.constructs.push(Construct::new(format!("c{}", n), format!("Construct {}", n)));
                state.status_message = format!("Constructs: {}", state.eprotein.constructs.len());
            }
            if ui.button("Fill demo constructs").clicked() {
                let n = match state.eprotein.screen_kind {
                    EproteinScreenKind::SolubleProtein => NUCLERA_SOLUBLE_N_CONSTRUCTS,
                    EproteinScreenKind::MembraneProtein => NUCLERA_MEMBRANE_N_CONSTRUCTS,
                };
                state.eprotein.constructs = (0..n)
                    .map(|i| {
                        let mut c = Construct::new(format!("g{}", i + 1), format!("Gene {}", i + 1));
                        c.amino_acid_sequence = "MKWVTFISLLFLFSSAYSRGVFRR".to_string();
                        c
                    })
                    .collect();
                state.status_message = format!("Loaded {n} demo constructs");
            }
            if ui.button("Clear constructs").clicked() {
                state.eprotein.constructs.clear();
                state.status_message = "Constructs cleared".to_string();
            }
        });

        ui.add_space(10.0);
        egui::ScrollArea::vertical()
            .max_height(420.0)
            .show(ui, |ui| {
                let mut remove_idx: Option<usize> = None;
                for (i, c) in state.eprotein.constructs.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(format!("#{}", i + 1));
                            ui.label("ID:");
                            ui.text_edit_singleline(&mut c.id);
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut c.display_name);
                            if ui.button("Remove").clicked() {
                                remove_idx = Some(i);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Topology:");
                            ui.selectable_value(&mut c.topology, DnaTopology::Linear, "Linear");
                            ui.selectable_value(&mut c.topology, DnaTopology::Circular, "Circular");
                        });
                        ui.label("Amino acid sequence (single-letter):");
                        ui.text_edit_multiline(&mut c.amino_acid_sequence);
                        ui.horizontal(|ui| {
                            ui.label("Solubility tag note:");
                            ui.text_edit_singleline(&mut c.solubility_tag_note);
                        });
                        if let Some(score) = c.prediction_solubility_score {
                            ui.label(format!("Cached solubility score: {:.4}", score));
                        }
                    });
                    ui.add_space(6.0);
                }
                if let Some(i) = remove_idx {
                    state.eprotein.constructs.remove(i);
                }
            });

        ui.add_space(10.0);
        if ui.button("Next: Screen matrix →").clicked() {
            state.eprotein_panel = EproteinNavPanel::ScreenMatrix;
        }
    });
}
