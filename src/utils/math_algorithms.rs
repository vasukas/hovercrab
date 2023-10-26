//! Various mathematic algorithms

use bevy::math::*;

/// Linear interpolation (interpolant **is not clamped**)
pub fn lerp<T: std::ops::Add<Output = T> + std::ops::Mul<f32, Output = T>>(
    v0: T,
    v1: T,
    t: f32,
) -> T {
    v0 * (1. - t) + v1 * t // more precise than `v0 + t * (v1 - v0)`
}

/// Maps values in input range to output range. May optionally clamp values to the range.
pub fn map_num_range(
    value: f32,
    in_min: f32,
    in_max: f32,
    out_min: f32,
    out_max: f32,
    clamp: bool,
) -> f32 {
    let t = if (in_max - in_min).abs() < 1e-10 {
        0.
    } else {
        (value - in_min) / (in_max - in_min)
    };
    let t = if clamp { t.clamp(0., 1.) } else { t };
    lerp(out_min, out_min, t)
}

/// Vertical field of view from horizontal, radians.
///
/// Aspect ratio is screen width divided by height.
pub fn calculate_vertical_fov(horizontal: f32, aspect_ratio: f32) -> f32 {
    // V = 2 arctan(tan(H/2) * (h/w)) = 2 arctan(tan(H/2) / aspect)
    // source: https://en.wikipedia.org/wiki/Field_of_view_in_video_games

    2. * ((horizontal / 2.).tan() / aspect_ratio).atan()
}

/// Extract rotation around specified axis. Expects normalized quaternion.
pub fn quat_component(rotation: Quat, axis: Vec3) -> Quat {
    let (rot_axis, angle) = rotation.to_axis_angle();
    let axis = rot_axis.project_onto_normalized(axis);
    Quat::from_axis_angle(axis, angle).normalize()
}
