use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Apply mouse movement in primary window to [`Transform::rotation`] of this
/// entity.
///
/// Active only when window is focused.
///
/// When active (even if no
/// [`MouselookController`] component exists), cursor is hidden and confined to
/// the window.
#[derive(Component)]
pub struct MouselookController {
    /// If true, camera isn't locked at min/max elevation.
    pub allow_flip: bool,
}

/// Settings used by all entities with [`MouselookController`].
#[derive(Resource, Serialize, Deserialize)]
#[serde(default)]
pub struct MouselookSettings {
    // By how much rotate camera if cursor moved horizontally from one edge of the screen to the
    // other.
    pub sensitivity_degrees: f32,
}

impl Default for MouselookSettings {
    fn default() -> Self {
        Self {
            sensitivity_degrees: 200.,
        }
    }
}

/// If target is set, follows it.
///
/// Sets XZ-translation based on rotation.
#[derive(Component)]
pub struct OrbitCamera {
    pub target: Option<Entity>,
    pub distance: f32,
}

/// Controls controls
#[derive(Resource)]
pub struct InputControl {
    pub mouselook_enabled: bool,
}

impl Default for InputControl {
    fn default() -> Self {
        Self {
            mouselook_enabled: true,
        }
    }
}

pub struct MouselookPlugin;

impl Plugin for MouselookPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MouselookSettings>()
            .init_resource::<MouseDelta>()
            .init_resource::<InputControl>()
            // First because mouse events are generated in winit runner
            .add_systems(First, mouse_update)
            .add_systems(PreUpdate, (mouselook_controller, orbit_camera).chain());
    }
}

#[derive(Resource, Default)]
struct MouseDelta {
    /// Total movement since last frame.
    ///
    /// Values are relative to window width.
    value: Vec2,
}

// grab doesn't work immediatly after window creation
const INITIAL_GRAB_DELAY: Duration = Duration::from_millis(500);

fn mouse_update(
    mut delta: ResMut<MouseDelta>,
    mut motion: EventReader<MouseMotion>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    time: Res<Time>,
    controls: Res<InputControl>,
) {
    let Ok(mut window) = windows.get_single_mut() else {
        return
    };

    let enabled =
        controls.mouselook_enabled && window.focused && time.elapsed() > INITIAL_GRAB_DELAY;

    if enabled {
        delta.value = motion
            .iter()
            .fold(Vec2::ZERO, |sum, event| sum + event.delta)
            / window.resolution.width();

        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    } else {
        delta.value = default();

        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn mouselook_controller(
    mut controllers: Query<(&mut Transform, &MouselookController)>,
    delta: Res<MouseDelta>,
    settings: Res<MouselookSettings>,
) {
    let sensitivity = settings.sensitivity_degrees.to_radians();

    for (mut transform, controller) in controllers.iter_mut() {
        transform.rotation = new_camera_rotation(
            delta.value * sensitivity,
            transform.rotation,
            controller.allow_flip,
        );
    }
}

/// Caclulate new camera rotation from changes in angle.
///
/// If `allow_flip` is true, camera isn't locked at max/min vertical angle.
fn new_camera_rotation(delta_angle: Vec2, old_rotation: Quat, allow_flip: bool) -> Quat {
    let stage1 = |delta_angle: Vec2| {
        let yaw = Quat::from_rotation_y(-delta_angle.x);
        let pitch = Quat::from_rotation_x(-delta_angle.y);

        let rotation = yaw * old_rotation; // rotate around global y axis
        let rotation = rotation * pitch; // rotate around local x axis
        rotation
    };

    let rotation = stage1(delta_angle);

    if (rotation * Vec3::Y).y > 0. {
        rotation
    } else {
        if allow_flip {
            stage1(Vec2::new(-delta_angle.x, delta_angle.y))
        } else {
            let yaw = Quat::from_rotation_y(-delta_angle.x);
            yaw * old_rotation
        }
    }
}

fn orbit_camera(
    mut entities: Query<(&mut Transform, &OrbitCamera)>,
    targets: Query<&GlobalTransform>,
) {
    for (mut transform, camera) in entities.iter_mut() {
        let offset = camera
            .target
            .and_then(|e| targets.get(e).ok())
            .map(|t| t.translation())
            .unwrap_or_default();

        let rotation = transform.rotation;
        transform.translation = offset + rotation * Vec3::Z * camera.distance;
    }
}
