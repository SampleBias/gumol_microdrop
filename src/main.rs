use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod data_models;
mod modules;
mod ui;
mod visualization;

use data_models::{CurrentPanel, ApplicationState};
use visualization::droplet_grid::DropletGridPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Gumol MicroDrop Design Studio".to_string(),
                resolution: (1400., 900.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(DropletGridPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .insert_resource(CurrentPanel::default())
        .insert_resource(ApplicationState::default())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn ui_system(
    mut contexts: EguiContexts,
    mut current_panel: ResMut<CurrentPanel>,
    mut app_state: ResMut<ApplicationState>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("nav_panel")
        .exact_width(160.0)
        .show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("MicroDrop");
            ui.separator();
            ui.add_space(4.0);

            let panels = [
                (CurrentPanel::SimulationImport, "1. Import"),
                (CurrentPanel::CardEditor, "2. Card Editor"),
                (CurrentPanel::ParameterMapping, "3. Param Map"),
                (CurrentPanel::ExperimentDesign, "4. Experiment"),
                (CurrentPanel::NucleraConfig, "5. Nuclera"),
                (CurrentPanel::ProtocolGenerator, "6. Protocol"),
                (CurrentPanel::DataViewer, "7. Data"),
            ];

            for (panel, label) in panels {
                ui.selectable_value(&mut *current_panel, panel, label);
            }

            ui.separator();
            ui.add_space(4.0);

            if app_state.simulation.is_some() {
                ui.colored_label(egui::Color32::from_rgb(100, 200, 100), "Sim: loaded");
            } else {
                ui.colored_label(egui::Color32::GRAY, "Sim: none");
            }

            if app_state.droplet_matrix.is_some() {
                let n = app_state.droplet_matrix.as_ref().unwrap().droplets.len();
                ui.colored_label(egui::Color32::from_rgb(100, 200, 100), format!("Matrix: {n}"));
            } else {
                ui.colored_label(egui::Color32::GRAY, "Matrix: none");
            }
        });

    egui::TopBottomPanel::bottom("status_bar")
        .exact_height(24.0)
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.label(&app_state.status_message);
            });
        });

    let panel = *current_panel;
    match panel {
        CurrentPanel::SimulationImport => {
            ui::simulation_importer::render(ctx, &mut app_state, &mut current_panel);
        }
        CurrentPanel::CardEditor => {
            ui::card_editor::render(ctx, &mut app_state);
        }
        CurrentPanel::ParameterMapping => {
            ui::parameter_mapping::render(ctx, &mut app_state, &mut current_panel);
        }
        CurrentPanel::ExperimentDesign => {
            ui::experiment_design::render(ctx, &mut app_state, &mut current_panel);
        }
        CurrentPanel::NucleraConfig => {
            ui::nuclera_config::render(ctx, &mut app_state, &mut current_panel);
        }
        CurrentPanel::ProtocolGenerator => {
            ui::protocol_generator::render(ctx, &mut app_state);
        }
        CurrentPanel::DataViewer => {
            ui::data_viewer::render(ctx, &mut app_state);
        }
    }
}
