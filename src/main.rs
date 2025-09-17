mod track;
mod train;
mod visualisation;
mod utils;

use crate::train::{car_movement_system, friction_system, gravity_system};
use crate::visualisation::VisualisationPlugin;
use bevy::prelude::*;
use crate::track::PathComponent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(VisualisationPlugin)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            ((gravity_system, friction_system), car_movement_system).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        PathComponent {
            start_anchor: Default::default(),
            end_anchor: Default::default(),
            start_control_point: Default::default(),
            end_control_point: Default::default(),
            length_meters: 0.0,
        }
    ));
}
