use crate::{
    gameplay::objects::hovercrab::Hovercrab,
    presentation::player::{
        camera::WorldCamera,
        mouselook::{MouselookController, OrbitCamera},
    },
    utils::{for_crate::bevy::FallibleCommands, plugins::scene_utils::SceneStaticCollider},
};
use bevy::prelude::*;

pub struct TmpPlugin;

impl Plugin for TmpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (initial_spawn,))
            .add_systems(Update, (spawn_player,));
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
            commands.try_despawn_recursive(entity);
        }

        let player_entity = commands
            .spawn((
                GameplayObject,
                SpatialBundle::from_transform(Transform::from_xyz(0., 5., 0.)),
            ))
            .id();

        let camera_entity = commands
            .spawn((
                GameplayObject,
                WorldCamera,
                MouselookController { allow_flip: false },
                OrbitCamera {
                    target: player_entity.into(),
                    distance: 12.,
                    offset: Vec3::new(0., 3., 0.),
                },
            ))
            .id();

        commands.try_insert(player_entity, Hovercrab::new(camera_entity));
    }
}
