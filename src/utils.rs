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

pub fn estimate_curve_length(path: &PathComponent, num_segments: u32) -> f32 {
    let mut total_length = 0.0;

    // Get the starting point of the curve (at t=0.0).
    let (mut previous_point, _) = evaluate_bezier(&path, 0.0);

    // Iterate through each segment of the curve. We start at 1 since we
    // already have the position for segment 0.
    for i in 1..=num_segments {
        // Calculate our progress 't' along the curve for the current segment.
        let progress = i as f32 / num_segments as f32;

        // Get the 3D position of the next point on the curve.
        let (current_point, _) = evaluate_bezier(&path, progress);

        // Calculate the straight-line distance between the previous point and the
        // current point, and add it to our total.
        total_length += previous_point.distance(current_point);

        // Update the previous point for the next iteration of the loop.
        previous_point = current_point;
    }

    total_length
}

pub fn build_arc_length_table(path: &PathComponent, num_segments: u32) -> Vec<(f32, f32)> {
    let mut table = Vec::with_capacity(num_segments as usize + 1);
    let mut cumulative_distance = 0.0;

    // Add the starting point of the curve.
    let (mut previous_point, _) = evaluate_bezier(path, 0.0);
    table.push((0.0, 0.0)); // At 0.0 meters, t is 0.0

    for i in 1..=num_segments {
        let progress = i as f32 / num_segments as f32;
        let (current_point, _) = evaluate_bezier(path, progress);

        // Add the distance of this segment to our running total.
        cumulative_distance += previous_point.distance(current_point);
        table.push((cumulative_distance, progress));

        previous_point = current_point;
    }
    table
}