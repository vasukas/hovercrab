//! Various mathematic algorithms

use bevy::math::*;
use std::f32::consts::PI;

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

/// Vector rotated by angle (radians)
pub fn rotate_vec_2d(v: Vec2, angle: f32) -> Vec2 {
    let (cs, sn) = (angle.cos(), angle.sin());
    Vec2::new(v.x * cs - v.y * sn, v.x * sn + v.y * cs)
}

/// Clamps value in range `[start; end)`, but wraps it around instead of
/// saturating.
pub fn wrapping_clamp(value: f32, start: f32, end: f32) -> f32 {
    // source: https://stackoverflow.com/a/14416133
    // TODO: attribution

    (((value - start) % (end - start)) + (end - start)) % (end - start) + start
}

/// Shortest angle from `current` to `target`
pub fn shortest_angle(current: f32, target: f32) -> f32 {
    wrapping_clamp(target - current, -PI, PI)
}

/// Change current to target, limited by max speed. Returns new current angle.
pub fn change_angle_constant_speed(current: f32, target: f32, speed: f32) -> f32 {
    let delta = shortest_angle(current, target);
    let magnitude = delta.abs().min(speed);
    current + magnitude.copysign(delta)
}

#[cfg(test)]
mod test {
    use super::*;
    use approx::*;

    /// Numbers are considered equal if difference is less than that.
    ///
    /// Required due to floating-point rounding errors.
    const ERROR: f32 = 0.0001;

    // TODO: comments

    #[test]
    fn test_others() {
        // TODO: implement
        todo!()
    }

    #[test]
    fn third_party() {
        // just to check I understood the API

        assert_relative_eq!(0., ERROR * 0.99, epsilon = ERROR);
        assert_relative_ne!(0., ERROR * 1.01, epsilon = ERROR);

        assert_relative_eq!(Vec2::X.angle_between(Vec2::Y), PI / 2., epsilon = ERROR);
        assert_relative_eq!(Vec2::Y.angle_between(Vec2::X), -PI / 2., epsilon = ERROR);
    }

    #[test]
    fn rotate_vec_2d() {
        let fun = super::rotate_vec_2d;

        assert_relative_eq!(fun(Vec2::X, PI / 2.), Vec2::Y, epsilon = ERROR);
        assert_relative_eq!(fun(Vec2::Y, -PI / 2.), Vec2::X, epsilon = ERROR);
    }

    #[test]
    fn wrapping_clamp() {
        let fun = super::wrapping_clamp;

        // inside range
        assert_relative_eq!(fun(0., -100., 200.), 0., epsilon = ERROR);
        assert_relative_eq!(fun(50., -100., 200.), 50., epsilon = ERROR);
        assert_relative_eq!(fun(-10., -100., 200.), -10., epsilon = ERROR);

        // below range
        assert_relative_eq!(fun(-100., -100., 200.), -100., epsilon = ERROR); // border
        assert_relative_eq!(fun(-130., -100., 200.), 170., epsilon = ERROR); // by 1 time
        assert_relative_eq!(fun(-500., -100., 200.), 100., epsilon = ERROR); // by 2 times

        // above range
        assert_relative_eq!(fun(200., -100., 200.), -100., epsilon = ERROR); // border
        assert_relative_eq!(fun(230., -100., 200.), -70., epsilon = ERROR); // by 1 time
        assert_relative_eq!(fun(601., -100., 200.), 1., epsilon = ERROR); // by 2 times
    }

    #[test]
    fn shortest_angle() {
        let fun = super::shortest_angle;

        assert_relative_eq!(fun(0., 0.), 0., epsilon = ERROR);
        assert_relative_eq!(fun(0., 1.), 1., epsilon = ERROR);
        assert_relative_eq!(fun(1., 0.), -1., epsilon = ERROR);
        assert_relative_eq!(fun(PI * 6., PI * 7. - 0.01), PI - 0.01, epsilon = ERROR);
        assert_relative_eq!(fun(PI * -7., PI * -6. - 0.01), PI - 0.01, epsilon = ERROR);
    }

    #[test]
    fn change_angle_constant_speed() {
        let fun = super::change_angle_constant_speed;

        assert_relative_eq!(fun(0., 1., 0.), 0., epsilon = ERROR);
        assert_relative_eq!(fun(0., 1., 1.), 1., epsilon = ERROR);
        assert_relative_eq!(fun(1., 0., 1.), 0., epsilon = ERROR);
        assert_relative_eq!(fun(0., 1., 0.25), 0.25, epsilon = ERROR);
        assert_relative_eq!(fun(0., 0., 0.25), 0., epsilon = ERROR);
        assert_relative_eq!(fun(1., 0., 0.25), 0.75, epsilon = ERROR);
        assert_relative_eq!(fun(-PI / 2., PI / 3., PI * 2.), PI / 3., epsilon = ERROR);
        assert_relative_eq!(fun(PI / 2., -PI / 3., PI * 2.), -PI / 3., epsilon = ERROR);
    }
}
