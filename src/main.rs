mod track;
mod train;

use bevy::prelude::*;
use crate::train::{car_movement_system, friction_system, gravity_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, (
            (
                gravity_system,
                friction_system,
            ),
            car_movement_system
        ).chain())
        .run();
}
