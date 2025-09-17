use bevy::math::Vec3;
use crate::track::PathComponent;

pub fn evaluate_bezier(path: &PathComponent, t: f32) -> (Vec3, Vec3) {
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
