use bevy::log::tracing_subscriber::fmt::init;
use bevy::prelude::*;
use crate::track::{PathComponent, TrackConnection};

const GRAVITY_ACCELERATION: f32 = -9.81;

#[derive(Component)]
pub struct TrackPosition {
    track: Entity,
    distance_on_piece: f32,
}

#[derive(Component)]
pub struct LinearVelocity {
    pub speed: f32,
}

#[derive(Component)]
pub struct TrainPhysics {
    pub drag_factor: f32,
    pub rolling_resistance: f32,
}

impl Default for TrainPhysics {
    fn default() -> Self {
        Self {
            drag_factor: 0.0025,
            rolling_resistance: 0.25,
        }
    }
}

pub fn gravity_system(
    time: Res<Time>,
    mut cars: Query<(&mut LinearVelocity, &TrackPosition,)>,
    track_pieces: Query<&PathComponent>,
) {
    for (mut linear_velocity, position,) in cars.iter_mut() {
        let Ok(track_path) = track_pieces.get(position.track) else {
            eprintln!("Could not get track piece for car");

            continue;
        };

        let progress = (position.distance_on_piece / track_path.length_meters).clamp(0.0, 1.0);

        let (_, forward_dir) = evaluate_bezier(track_path, progress);

        let tangent = forward_dir.normalize_or_zero();

        let acceleration_along_track = GRAVITY_ACCELERATION * tangent.y;

        linear_velocity.speed += acceleration_along_track * time.delta_secs();
    }
}

pub fn friction_system(
    time: Res<Time>,
    mut cars: Query<(&mut LinearVelocity, &TrainPhysics,)>,
) {
    for (mut linear_velocity, physics,) in cars.iter_mut() {
        let initial_speed = linear_velocity.speed;
        if initial_speed.abs() < 0.001 {
            continue;
        }

        let friction_speed_reduction = physics.rolling_resistance * time.delta_secs();

        let drag_deceleration = physics.drag_factor * initial_speed.powi(2);
        let drag_speed_reduction = drag_deceleration * time.delta_secs();

        let total_reduction = drag_speed_reduction + friction_speed_reduction;

        linear_velocity.speed += total_reduction * initial_speed.signum();

        if initial_speed.signum() != linear_velocity.speed.signum() {
            linear_velocity.speed = 0.0;
        }

    }
}

pub fn car_movement_system(
    time: Res<Time>,
    mut cars: Query<(&mut TrackPosition, &mut Transform, &mut LinearVelocity)>,
    track_pieces: Query<(&PathComponent, &TrackConnection)>,
) {
    for (mut position, mut transform, mut velocity) in cars.iter_mut() {
        let Ok((mut track_path, mut track_connection)) = track_pieces.get(position.track) else {
            continue;
        };

        position.distance_on_piece += velocity.speed * time.delta_secs();

        while position.distance_on_piece >= track_path.length_meters {
            if let Some(next_track_id) = track_connection.next {
                let leftover = position.distance_on_piece - track_path.length_meters;

                position.track = next_track_id;
                position.distance_on_piece = leftover;

                if let Ok((new_path, new_connection)) = track_pieces.get(next_track_id) {
                    track_path = new_path;
                    track_connection = new_connection;
                } else {
                    eprintln!("Track connection could not be found.");

                    break;
                }
            } else {
                // Force stop at end of track.
                position.distance_on_piece = track_path.length_meters;
                velocity.speed = 0.0;

                break;
            }
        }

        while position.distance_on_piece < 0.0 {
            if let Some(previous_track_id) = track_connection.previous {
                if let Ok((new_path, new_connection)) = track_pieces.get(previous_track_id) {
                    position.distance_on_piece += new_path.length_meters;
                    position.track = previous_track_id;

                    track_path = new_path;
                    track_connection = new_connection;
                } else {
                    eprintln!("Track connection could not be found.");

                    break;
                }
            } else {
                position.distance_on_piece = 0.0;
                velocity.speed = 0.0;
            }
        }

        let progress = position.distance_on_piece / track_path.length_meters;
        let (world_pos, forward_dir) = evaluate_bezier(track_path, progress);

        let facing_dir = if velocity.speed < 0.0 { -forward_dir } else { forward_dir };

        transform.translation = world_pos.into();
        if facing_dir.length_squared() > 0.0 {
            transform.look_to(facing_dir, Vec3::Y);
        }
    }
}

fn evaluate_bezier(path: &PathComponent, t: f32) -> (Vec3, Vec3) {
    let t = t.clamp(0.0, 1.0);

    let p0 = path.start_anchor;
    let p1 = path.start_control_point;
    let p2 = path.end_control_point;
    let p3 = path.end_anchor;

    let one_minus_t = 1.0 - t;
    let one_minus_t_sq = one_minus_t * one_minus_t;
    let t_sq = t * t;

    let term0 = p0 * (one_minus_t_sq * one_minus_t);
    let term1 = p1 * (3.0 * one_minus_t_sq * t);
    let term2 = p2 * (3.0 * one_minus_t * t_sq);
    let term3 = p3 * (t_sq * t);

    let position = term0 + term1 + term2 + term3;

    let tangent = (p1 - p0) * (3.0 * one_minus_t_sq)
        + (p2 - p1) * (6.0 * one_minus_t * t)
        + (p3 - p2) * (3.0 * t_sq);

    let direction = tangent.normalize_or_zero();

    (position, direction)
}