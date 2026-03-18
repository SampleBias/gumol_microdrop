use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod data_models;
mod modules;
mod ui;
mod visualization;

use data_models::{CurrentPanel, ApplicationState};
use ui::{card_editor::CardEditorPanel, simulation_importer::SimulationImporterPanel};
use visualization::droplet_grid::DropletGridPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
    commands.spawn((
        TextBundle::from_section(
            "Gumol MicroDrop Design Studio",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        }),
    ));
}

fn ui_system(
    mut contexts: EguiContexts,
    mut current_panel: ResMut<CurrentPanel>,
    app_state: Res<ApplicationState>,
) {
    egui::SidePanel::left("panel_selector")
        .default_width(250.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Navigation");
            ui.separator();

            ui.selectable_value(&mut *current_panel, CurrentPanel::SimulationImport, "1. Simulation Import");
            ui.selectable_value(&mut *current_panel, CurrentPanel::CardEditor, "2. Card Editor");
            ui.selectable_value(&mut *current_panel, CurrentPanel::ParameterMapping, "3. Parameter Mapping");
            ui.selectable_value(&mut *current_panel, CurrentPanel::ExperimentDesign, "4. Experiment Design");
            ui.selectable_value(&mut *current_panel, CurrentPanel::NucleraConfig, "5. Nuclera Config");
            ui.selectable_value(&mut *current_panel, CurrentPanel::ProtocolGenerator, "6. Protocol Generator");
            ui.selectable_value(&mut *current_panel, CurrentPanel::DataViewer, "7. Data Viewer");

            ui.separator();
            ui.label(format!("Status: {}", if app_state.simulation_file.is_some() { "Simulation loaded" } else { "No simulation" }));
        });

    match *current_panel {
        CurrentPanel::SimulationImport => {
            SimulationImporterPanel::render(ui::simulation_importer::SimulationImporterContext, contexts, &mut *current_panel);
        }
        CurrentPanel::CardEditor => {
            CardEditorPanel::render(ui::card_editor::CardEditorContext, contexts, &app_state);
        }
        CurrentPanel::ParameterMapping => {
            ui::parameter_mapping::render_parameter_mapping(contexts);
        }
        CurrentPanel::ExperimentDesign => {
            ui::experiment_design::render_experiment_design(contexts);
        }
        CurrentPanel::NucleraConfig => {
            ui::nuclera_config::render_nuclera_config(contexts);
        }
        CurrentPanel::ProtocolGenerator => {
            ui::protocol_generator::render_protocol_generator(contexts);
        }
        CurrentPanel::DataViewer => {
            ui::data_viewer::render_data_viewer(contexts);
        }
    }
}
