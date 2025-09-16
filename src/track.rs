use bevy::math::Vec3;
use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct PathComponent {
    pub start_anchor: Vec3,
    pub end_anchor: Vec3,
    pub start_control_point: Vec3,
    pub end_control_point: Vec3,
}

#[derive(Component)]
pub struct TrackConnection {
    previous: Option<Entity>,
    next: Option<Entity>,
}