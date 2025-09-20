mod track;
mod train;
mod utils;

use crate::track::{PathComponent, TrackConnection};
use crate::train::{
    LinearVelocity, TrackPosition, car_movement_system, friction_system, gravity_system,
};
use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    #[cfg(feature = "visualization")]
    {
        mod visualisation;

        use crate::visualisation::VisualisationPlugin;
        use bevy_flycam::PlayerPlugin;

        app.add_plugins((DefaultPlugins, VisualisationPlugin, PlayerPlugin));
    }

    #[cfg(not(feature = "visualization"))]
    {
        use bevy::app::ScheduleRunnerPlugin;
        use std::time::Duration;

        app.add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(10))),
        );
    }

    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            ((gravity_system, friction_system), car_movement_system).chain(),
        );

    app.run();
}

#[derive(Component)]
struct First;

fn setup(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 100.0,
        affects_lightmapped_meshes: true,
    });

    // --- Spawn the 4 Track Pieces ---

    // --- Piece 1: The Station (Flat) ---
    let station_id = commands
        .spawn((
            PathComponent::new_straight(
                Vec3::new(0.0, 5.0, 0.0),  // Start
                Vec3::new(20.0, 5.0, 0.0), // End
            ),
            First,
        ))
        .id();

    // --- Piece 2: The Lift Hill (Straight incline) ---
    let lift_hill_id = commands
        .spawn(PathComponent::new_straight(
            Vec3::new(20.0, 5.0, 0.0),
            Vec3::new(70.0, 30.0, 0.0), // Goes up 25m over 50m
        ))
        .id();

    // --- Piece 3: The Crest and Drop (Curved) ---
    // A nice parabolic arc for the first drop.
    let drop_id = commands
        .spawn(PathComponent::new(
            Vec3::new(70.0, 30.0, 0.0),
            Vec3::new(100.0, 0.0, 20.0), // Drops 30m and turns slightly
            Vec3::new(85.0, 32.0, 0.0),  // Control point is high to create the crest
            Vec3::new(95.0, 10.0, 15.0), // Control point pulls the curve down
        ))
        .id();

    // --- Piece 4: The Banked Turn (Curved and banked) ---
    // This piece curves while staying low to the ground.
    let turn_id = commands
        .spawn(PathComponent::new(
            Vec3::new(100.0, 0.0, 20.0),
            Vec3::new(60.0, 1.0, 60.0), // Ends further back and to the right
            Vec3::new(105.0, -1.0, 35.0), // Control points are wide to make a sweeping turn
            Vec3::new(80.0, 0.0, 65.0),
        ))
        .id();

    // --- Piece 5: The Airtime Hill (Small bump) ---
    let airtime_hill_id = commands
        .spawn(PathComponent::new(
            Vec3::new(60.0, 1.0, 60.0),
            Vec3::new(20.0, 2.0, 20.0), // Comes back towards the start
            Vec3::new(50.0, 8.0, 50.0), // Control point creates a small, sharp hill
            Vec3::new(30.0, 8.0, 30.0),
        ))
        .id();

    // --- Piece 6: Final Brake Run (Flat) ---
    let brake_run_id = commands
        .spawn(PathComponent::new_straight(
            Vec3::new(20.0, 2.0, 20.0),
            Vec3::new(0.0, 5.0, 0.0), // Rises slightly to meet the station
        ))
        .id();

    // --- Connect All The Pieces ---
    commands
        .entity(station_id)
        .insert(TrackConnection::new_closed(brake_run_id, lift_hill_id));
    commands
        .entity(lift_hill_id)
        .insert(TrackConnection::new_closed(station_id, drop_id));
    commands
        .entity(drop_id)
        .insert(TrackConnection::new_closed(lift_hill_id, turn_id));
    commands
        .entity(turn_id)
        .insert(TrackConnection::new_closed(drop_id, airtime_hill_id));
    commands
        .entity(airtime_hill_id)
        .insert(TrackConnection::new_closed(turn_id, brake_run_id));
    commands
        .entity(brake_run_id)
        .insert(TrackConnection::new_closed(airtime_hill_id, station_id));

    // --- Spawn a Car on the Track ---

    commands.spawn((
        // Example components needed for a car
        LinearVelocity { speed: 15.0 },
        // Place the car on the first piece
        TrackPosition {
            track: station_id,
            distance_on_piece: 0.0,
        },
    ));
}

