use bevy::math::Vec3;
use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct PathComponent {
    pub start_anchor: Vec3,
    pub end_anchor: Vec3,
    pub start_control_point: Vec3,
    pub end_control_point: Vec3,
    pub length_meters: f32,
}

#[derive(Component)]
pub struct TrackConnection {
    pub previous: Option<Entity>,
    pub next: Option<Entity>,
}