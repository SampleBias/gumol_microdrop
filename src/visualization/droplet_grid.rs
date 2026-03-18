use bevy::prelude::*;
use crate::data_models::{DropletCondition, CardDesign};

#[derive(Component)]
pub struct Droplet {
    pub row: usize,
    pub col: usize,
    pub condition: Option<DropletCondition>,
}

#[derive(Component)]
pub struct DropletGrid {
    pub rows: usize,
    pub cols: usize,
}

#[derive(Resource)]
pub struct DropletGridState {
    pub selected_droplet: Option<(usize, usize)>,
    pub show_labels: bool,
    pub color_mode: ColorMode,
}

impl Default for DropletGridState {
    fn default() -> Self {
        DropletGridState {
            selected_droplet: None,
            show_labels: true,
            color_mode: ColorMode::default(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorMode {
    #[default]
    Oxidant,
    Antioxidant,
    ExposureTime,
    Treatment,
}

pub struct DropletGridPlugin;

impl Plugin for DropletGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DropletGridState::default())
            .add_systems(Startup, setup_droplet_grid)
            .add_systems(Update, (
                update_droplet_colors,
                handle_droplet_selection,
            ));
    }
}

fn setup_droplet_grid(mut commands: Commands) {
    let rows = 8;
    let cols = 12;
    let cell_size = 45.0;
    let spacing = 5.0;

    // Create grid container
    let grid_width = cols as f32 * (cell_size + spacing) - spacing;
    let grid_height = rows as f32 * (cell_size + spacing) - spacing;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(grid_width),
                    height: Val::Px(grid_height),
                    flex_direction: FlexDirection::Row,
                    flex_wrap: FlexWrap::Wrap,
                    row_gap: Val::Px(spacing),
                    column_gap: Val::Px(spacing),
                    padding: UiRect::all(Val::Px(20.0)),
                    ..default()
                },
                background_color: BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                ..default()
            },
            DropletGrid { rows, cols },
        ))
        .with_children(|parent| {
            for row in 0..rows {
                for col in 0..cols {
                    let index = row * cols + col;
                    let color = calculate_droplet_color(index, row, col, ColorMode::Oxidant);

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(cell_size),
                                height: Val::Px(cell_size),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: BackgroundColor(color),
                            border_color: BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                            ..default()
                        },
                        Droplet {
                            row,
                            col,
                            condition: None,
                        },
                        Interaction::default(),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            format!("{}{}", (b'A' + row as u8) as char, col + 1),
                            TextStyle {
                                font_size: 10.0,
                                color: Color::srgb(0.1, 0.1, 0.1),
                                ..default()
                            },
                        ));
                    });
                }
            }
        });

    // Create legend
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(grid_width + 40.0),
                top: Val::Px(20.0),
                width: Val::Px(200.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Legend",
                TextStyle {
                    font_size: 18.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            let legend_items = vec![
                ("Control", Color::srgb(0.94, 0.94, 0.94)),
                ("Low Oxidant", Color::srgb(1.0, 0.78, 0.78)),
                ("Medium Oxidant", Color::srgb(1.0, 0.59, 0.59)),
                ("High Oxidant", Color::srgb(1.0, 0.39, 0.39)),
                ("Low Antioxidant", Color::srgb(0.78, 1.0, 0.78)),
                ("Medium Antioxidant", Color::srgb(0.59, 1.0, 0.59)),
                ("High Antioxidant", Color::srgb(0.39, 1.0, 0.39)),
            ];

            for (label, color) in legend_items {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            column_gap: Val::Px(10.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(20.0),
                                    height: Val::Px(20.0),
                                    ..default()
                                },
                                background_color: BackgroundColor(color),
                                ..default()
                            },
                        ));
                        parent.spawn(TextBundle::from_section(
                            label,
                            TextStyle {
                                font_size: 14.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

fn update_droplet_colors(
    grid_state: Res<DropletGridState>,
    mut droplets: Query<(&Droplet, &mut BackgroundColor)>,
) {
    for (droplet, mut bg_color) in droplets.iter_mut() {
        let color = calculate_droplet_color(
            droplet.row * 12 + droplet.col,
            droplet.row,
            droplet.col,
            grid_state.color_mode,
        );
        bg_color.0 = color;
    }
}

fn handle_droplet_selection(
    mut grid_state: ResMut<DropletGridState>,
    mut droplets: Query<(&Droplet, &Interaction, &mut BorderColor)>,
) {
    for (droplet, interaction, mut border_color) in droplets.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                grid_state.selected_droplet = Some((droplet.row, droplet.col));
                border_color.0 = Color::srgb(1.0, 1.0, 0.0);
            }
            Interaction::Hovered => {
                if grid_state.selected_droplet != Some((droplet.row, droplet.col)) {
                    border_color.0 = Color::srgb(1.0, 0.8, 0.0);
                }
            }
            Interaction::None => {
                if grid_state.selected_droplet != Some((droplet.row, droplet.col)) {
                    border_color.0 = Color::srgb(0.3, 0.3, 0.3);
                }
            }
        }
    }
}

fn calculate_droplet_color(index: usize, row: usize, col: usize, mode: ColorMode) -> Color {
    match mode {
        ColorMode::Oxidant => {
            // Generate oxidant-based colors
            if index < 12 {
                Color::srgb(0.94, 0.94, 0.94) // Control
            } else if index < 36 {
                let factor = (index - 12) as f32 / 24.0;
                Color::srgb(1.0, 0.78 - 0.39 * factor, 0.78 - 0.39 * factor)
            } else if index < 60 {
                Color::srgb(0.78, 1.0, 0.78)
            } else if index < 84 {
                Color::srgb(0.78 + 0.08 * (col as f32 / 12.0), 1.0, 0.78)
            } else {
                Color::srgb(1.0, 1.0, 0.71)
            }
        }
        ColorMode::Antioxidant => {
            // Generate antioxidant-based colors
            if col < 4 {
                Color::srgb(0.94, 0.94, 0.94)
            } else if col < 8 {
                Color::srgb(0.78, 1.0, 0.78)
            } else {
                Color::srgb(0.59, 1.0, 0.59)
            }
        }
        ColorMode::ExposureTime => {
            // Generate time-based colors (blue gradient)
            let time_factor = row as f32 / 8.0;
            Color::srgb(0.59, 0.59, 1.0 - 0.41 * time_factor)
        }
        ColorMode::Treatment => {
            // Generate treatment-based colors
            match row % 4 {
                0 => Color::srgb(0.94, 0.94, 0.94),
                1 => Color::srgb(1.0, 0.78, 0.78),
                2 => Color::srgb(0.78, 1.0, 0.78),
                _ => Color::srgb(0.78, 0.78, 1.0),
            }
        }
    }
}
