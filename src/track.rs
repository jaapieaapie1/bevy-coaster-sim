use bevy::math::Vec3;
use bevy::prelude::{Component, Entity};
use crate::utils::{build_arc_length_table, estimate_curve_length};

#[derive(Component)]
pub struct PathComponent {
    pub start_anchor: Vec3,
    pub end_anchor: Vec3,
    pub start_control_point: Vec3,
    pub end_control_point: Vec3,
    pub length_meters: f32,
    pub arc_length_table: Vec<(f32, f32)>,
}

impl PathComponent {
    pub fn new(start_anchor: Vec3, end_anchor: Vec3, start_control_point: Vec3, end_control_point: Vec3) -> Self
    {
        let mut path = Self {
            start_anchor,
            end_anchor,
            start_control_point,
            end_control_point,
            length_meters: 0.0,
            arc_length_table: Vec::new(),
        };

        path.length_meters = estimate_curve_length(&path, 100);
        path.arc_length_table = build_arc_length_table(&path, 100);

        path
    }

    pub fn new_straight(start: Vec3, end: Vec3) -> Self
    {
        let mut path = Self {
            start_anchor: start,
            end_anchor: end,
            start_control_point: start,
            end_control_point: end,
            length_meters: 0.0,
            arc_length_table: Vec::new(),
        };

        path.length_meters = estimate_curve_length(&path, 100);
        path.arc_length_table = build_arc_length_table(&path, 100);

        path
    }

    pub fn get_t_for_distance(&self, distance: f32) -> f32
    {
        if self.arc_length_table.is_empty() {
            return 0.0;
        }

        match self.arc_length_table.binary_search_by(|(d, _)| d.partial_cmp(&distance).unwrap()) {
            Ok(index) => self.arc_length_table[index].1,
            Err(index) => {
                if index == 0 {
                    0.0
                } else if index >= self.arc_length_table.len() {
                    return 1.0;
                } else {
                    let (dist_before, t_before) = self.arc_length_table[index - 1];
                    let (dist_after, t_after) = self.arc_length_table[index];

                    let segment_length = dist_after - dist_before;
                    if segment_length.abs() < f32::EPSILON {
                        return t_before;
                    }

                    let progress_in_segment = (distance - dist_before) / segment_length;

                    t_before + (t_after - t_before) * progress_in_segment
                }
            }
        }
    }
}

#[derive(Component)]
pub struct TrackConnection {
    pub previous: Option<Entity>,
    pub next: Option<Entity>,
}

impl TrackConnection {
    pub fn new_closed(previous: Entity, next: Entity) -> Self {
        Self { previous: Some(previous), next: Some(next) }
    }
}