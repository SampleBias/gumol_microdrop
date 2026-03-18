use bevy::prelude::*;
use crate::data_models::ColorMode;

#[derive(Resource, Default)]
pub struct DropletGridState {
    pub selected_droplet: Option<usize>,
    pub color_mode: ColorMode,
}

pub struct DropletGridPlugin;

impl Plugin for DropletGridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DropletGridState::default());
    }
}
