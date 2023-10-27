use crate::utils::{for_crate::bevy::FallibleCommands, math_algorithms};
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct Hovercrab {
    camera_entity: Option<Entity>,

    input_dir: Vec3, // normalized by XZ, but not by Y
    input_accel: bool,
    input_stop: bool,
    target_rotation: Vec3,
}

impl Hovercrab {
    pub fn new(camera_entity: Entity) -> Self {
        Self {
            camera_entity: Some(camera_entity),
            ..default()
        }
    }
}

//

pub struct HovercrabPlugin;

impl Plugin for HovercrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_hovercrab, (hovercrab_input, update_hovercrab).chain()),
        );
    }
}

const CRAB_HALF_SIZE: Vec3 = Vec3::new(4., 1., 3.);

fn spawn_hovercrab(mut commands: Commands, entities: Query<Entity, Added<Hovercrab>>) {
    let half_size = CRAB_HALF_SIZE;
    let mass = 800.;

    for entity in entities.iter() {
        commands.try_insert(
            entity,
            (
                RigidBody::Dynamic,
                // TODO: use round cuboid or convex hull?
                Collider::cuboid(half_size.x, half_size.y, half_size.z),
                ColliderMassProperties::Mass(mass),
                ReadMassProperties::default(),
                Velocity::default(),
                ExternalForce::default(),
            ),
        );
    }
}

// TODO: move this to presentation? untie from keys, tie to local player
fn hovercrab_input(
    mut crab: Query<&mut Hovercrab>,
    cameras: Query<&Transform>,
    keys: Res<Input<KeyCode>>,
    // buttons: Res<Input<MouseButton>>,
) {
    let Ok(mut crab) = crab.get_single_mut() else {
        return
    };

    let rotation = crab
        .camera_entity
        .and_then(|e| cameras.get(e).ok())
        .map(|t| t.rotation)
        .unwrap_or_default();

    let mut mov = Vec3::ZERO;
    if keys.pressed(KeyCode::W) {
        mov.z -= 1.
    }
    if keys.pressed(KeyCode::S) {
        mov.z += 1.
    }
    if keys.pressed(KeyCode::A) {
        mov.x -= 1.
    }
    if keys.pressed(KeyCode::D) {
        mov.x += 1.
    }
    mov = mov.normalize_or_zero();

    if keys.pressed(KeyCode::Z) {
        mov.y += 1.
    }
    if keys.pressed(KeyCode::X) {
        mov.y -= 1.
    }

    crab.input_dir = mov;

    crab.input_accel = keys.pressed(KeyCode::ShiftLeft);
    crab.input_stop = keys.pressed(KeyCode::ControlLeft);

    crab.target_rotation = rotation * Vec3::NEG_Z;
}

fn update_hovercrab(
    mut crabs: Query<(
        Entity,
        &Hovercrab,
        &Transform,
        &Velocity,
        &ReadMassProperties,
        &mut ExternalForce,
    )>,
    phy_ctx: Res<RapierContext>,
    phy_config: Res<RapierConfiguration>,
    time: Res<Time>,
) {
    let rotation_speed = 180_f32.to_radians();
    let body_height = CRAB_HALF_SIZE;
    let ray_max_offset = CRAB_HALF_SIZE * 0.8;
    let ray_margin = 0.1;
    let ray_length = 10.;

    let delta_seconds = time.delta_seconds();
    if delta_seconds < 0.001 {
        return;
    }

    for (body_entity, crab, transform, velocity, mass, mut ext_force) in crabs.iter_mut() {
        let mass = mass.0.mass;
        let center_of_mass = transform.translation;
        let body_rotation = transform.rotation;

        // reset forces
        *ext_force = default();

        // rotation magic
        {
            let current_dir = (body_rotation * Vec3::NEG_Z).xz();
            let target_dir = crab.target_rotation.xz();

            let current_velocity = -velocity.angvel.y;

            let next_angle = Vec2::Y.angle_between(current_dir) + current_velocity * delta_seconds;
            let target_angle = Vec2::Y.angle_between(target_dir);
            let delta_angle = math_algorithms::shortest_angle(next_angle, target_angle);

            let target_velocity = delta_angle / delta_seconds;
            // let target_velocity = target_velocity
            //     .abs()
            //     .min(rotation_speed)
            //     .copysign(target_velocity);
            let torque = target_velocity / delta_seconds;

            ext_force.torque.y -= torque * mass; // * 0.5;
        }

        // rays
        let ray_count = 4.;
        for (x_dir, z_dir) in [
            (0., -1.), // forward
            (0., 1.),  // backward
            (-1., 0.), // left
            (1., 0.),  // right
        ] {
            let ray_pos =
                center_of_mass + body_rotation * (Vec3::new(x_dir, 0., z_dir) * ray_max_offset);

            // let ray_dir = Vec3::Y;
            let ray_dir = {
                // v.3

                let max_angle = 45f32.to_radians();
                let rotation = math_algorithms::quat_component(transform.rotation, Vec3::Y);

                let fwd_rotation = Quat::from_rotation_x(crab.input_dir.z * max_angle);

                let rotation = rotation * fwd_rotation;

                rotation * Vec3::Y

                // v.2

                // let rotation = math_algorithms::quat_component(transform.rotation, Vec3::Y);

                // let base = Vec3::Y;
                // let fwd = rotation * Vec3::NEG_Z;
                // let side = rotation * Vec3::X;

                // let t = 0.25; // TODO: magic
                // let t = if crab.input_dir.x.abs() + crab.input_dir.z.abs() > 1.9 {
                //     t / 2.
                // } else {
                //     t
                // };
                // (base + fwd * -crab.input_dir.z * t + side * t * crab.input_dir.x).normalize()

                // v.1

                // let base = math_algorithms::quat_from_direction(base);
                // let fwd = math_algorithms::quat_from_direction(fwd);
                // let side = math_algorithms::quat_from_direction(side);

                // let s = 0.5;
                // base.slerp(fwd, s).slerp(side, s) * Vec3::Y
            };

            let ray_offset = ray_margin * 0.5;
            let body_offset = -ray_dir * body_height;

            let ray_hit = phy_ctx.cast_ray(
                ray_pos + ray_dir * ray_offset + body_offset,
                -ray_dir,
                ray_length + ray_margin,
                true,
                QueryFilter::default().exclude_rigid_body(body_entity),
            );

            let gravity = phy_config.gravity.y.abs();
            let k_force = 50.; // magic
            let min_force = gravity;
            let max_hover_force = 2. * gravity; // magic

            let mut force = min_force;

            // hover magic
            if let Some((_hit_entity, hit_distance)) = ray_hit {
                let distance_factor = (ray_length - hit_distance).max(0.) / ray_length;

                let current_velocity = velocity
                    .linear_velocity_at_point(ray_pos, center_of_mass)
                    .project_onto(ray_dir)
                    .y
                    .min(0.);
                let target_velocity = 0.;
                let target_force =
                    (target_velocity - current_velocity).max(0.) * distance_factor * k_force
                        / delta_seconds;

                force += target_force.clamp(-max_hover_force, max_hover_force);
            }

            *ext_force += ExternalForce::at_point(
                ray_dir * force * mass / ray_count,
                ray_pos,
                center_of_mass,
            );
        }

        // air drag force (real formula)
        {
            let speed = velocity.linvel.length();
            let dir = velocity.linvel.normalize_or_zero();

            let air_density = 1.225;
            let area = CRAB_HALF_SIZE.x * CRAB_HALF_SIZE.y * 4.;
            let drag_coeff = 0.09; // depends on object shape
            let force = 0.5 * air_density * speed.powi(2) * area * drag_coeff;

            ext_force.force -= dir * force;
        }
    }
}
