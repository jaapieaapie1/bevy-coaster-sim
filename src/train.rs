use bevy::prelude::*;

#[derive(Component)]
pub struct TrackPosition {
    track: Entity,
    progress: f32,
}

#[derive(Component)]
pub struct LinearVelocity {
    pub speed: f32,
}