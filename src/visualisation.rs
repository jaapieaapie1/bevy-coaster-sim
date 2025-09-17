use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use crate::track::PathComponent;
use crate::utils::evaluate_bezier;

pub struct VisualisationPlugin;

impl Plugin for VisualisationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, visualise_tracks);
    }
}
pub fn visualise_tracks(
    mut commands: Commands,
    paths: Query<(Entity, &PathComponent), Changed<PathComponent>>,
) {
    for (entity, path) in paths.iter() {
        commands.get_entity(entity).unwrap().log_components();
    }
}

pub fn generate_track_mesh(path: &PathComponent, subdivisions: u32, width: f32) -> Mesh
{
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=subdivisions as usize {
        let progress = i as f32 / subdivisions as f32;
        let (position, forward) = evaluate_bezier(path, progress);

        let up = if forward.dot(Vec3::Y).abs() < 0.99 {
            forward.cross(Vec3::Y).cross(forward).normalize_or_zero()
        } else {
            forward.cross(Vec3::Z).cross(forward).normalize_or_zero()
        };

        let right = forward.cross(up).normalize_or_zero();

        let half_width = width / 2.0;

        positions.push((position - right * half_width).to_array());
        positions.push((position + right * half_width).to_array());

        normals.push(up.to_array());
        normals.push(up.to_array());

        uvs.push([0.0, progress]);
        uvs.push([1.0, progress]);
    }

    for i in 0..subdivisions {
        let current = i * 2;
        let next = (i + 1) * 2;

        indices.push(current);
        indices.push(next + 1);
        indices.push(next);

        indices.push(current);
        indices.push(current + 1);
        indices.push(next + 1);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}