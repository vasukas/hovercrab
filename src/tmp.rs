use crate::{
    presentation::player::{
        camera::WorldCamera,
        mouselook::{MouselookController, OrbitCamera},
    },
    utils::{
        for_crate::bevy::FallibleCommands, math_algorithms,
        plugins::scene_utils::SceneStaticCollider,
    },
};
use bevy::prelude::*;

pub struct TmpPlugin;

impl Plugin for TmpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initial_spawn,))
            .add_systems(Update, (spawn_player, player_input));
    }
}

fn initial_spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ground & sun
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/ground.gltf#Scene0"),
            ..default()
        },
        SceneStaticCollider,
    ));
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 50_000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default().looking_to(Vec3::new(0.1, -0.9, -0.2), Vec3::Y),
        ..default()
    });
}

#[derive(Component)]
struct GameplayObject;

fn spawn_player(
    mut commands: Commands,
    objects: Query<Entity, With<GameplayObject>>,
    keys: Res<Input<KeyCode>>,
    mut inited: Local<bool>,
) {
    if keys.just_pressed(KeyCode::R) || !*inited {
        *inited = true;

        for entity in objects.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let player_entity = commands
            .spawn((GameplayObject, SpatialBundle::default()))
            .id();

        let camera_entity = commands
            .spawn((
                GameplayObject,
                WorldCamera,
                MouselookController { allow_flip: false },
                OrbitCamera {
                    target: player_entity.into(),
                    distance: 12.,
                },
            ))
            .id();

        commands.try_insert(player_entity, Player { camera_entity });
    }
}

#[derive(Component)]
struct Player {
    camera_entity: Entity,
}

fn player_input(
    mut objects: Query<(&mut Transform, &Player)>,
    cameras: Query<&Transform, Without<Player>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut gizmos: Gizmos,
) {
    let speed_slow = 3.;
    let speed_normal = 15.;
    let speed_fast = 50.;

    if let Ok((mut transform, player)) = objects.get_single_mut() {
        let rotation = cameras
            .get(player.camera_entity)
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
        mov = math_algorithms::quat_component(rotation, Vec3::Y) * mov.normalize_or_zero();
        if keys.pressed(KeyCode::Z) {
            mov.y += 1.
        }
        if keys.pressed(KeyCode::X) {
            mov.y -= 1.
        }

        if keys.pressed(KeyCode::ShiftLeft) {
            mov *= speed_fast
        } else if keys.pressed(KeyCode::ControlLeft) {
            mov *= speed_slow
        } else {
            mov *= speed_normal
        }

        transform.translation += mov * time.delta_seconds();

        gizmos.cuboid(*transform, Color::RED);
    }
}
