use crate::utils::{for_crate::bevy::FallibleCommands, math_algorithms};
use bevy::{
    core_pipeline::{bloom::BloomSettings, fxaa::Fxaa, tonemapping::Tonemapping, Skybox},
    prelude::*,
    render::render_resource::{TextureViewDescriptor, TextureViewDimension},
};

#[derive(Component)]
pub struct WorldCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SkyboxResource>()
            .add_systems(Startup, load_skybox)
            .add_systems(Update, (convert_skybox, set_skybox, spawn_world_camera));
    }
}

#[derive(Resource, Default)]
struct SkyboxResource {
    image: Handle<Image>,
    converted: bool,
}

fn load_skybox(mut skybox: ResMut<SkyboxResource>, asset_server: Res<AssetServer>) {
    let skybox_file = "skybox.png"; // TODO: use real one

    skybox.image = asset_server.load(skybox_file);
}

fn convert_skybox(mut skybox: ResMut<SkyboxResource>, mut images: ResMut<Assets<Image>>) {
    if !skybox.converted {
        if let Some(image) = images.get_mut(&skybox.image) {
            convert_2d_to_cubemap(image);
            skybox.converted = true;
        }
    }
}

fn convert_2d_to_cubemap(image: &mut Image) {
    image.reinterpret_stacked_2d_as_array(6);
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });
}

#[derive(Component)]
struct HasSkyboxSet;

fn set_skybox(
    skybox_resource: Res<SkyboxResource>,
    mut commands: Commands,
    mut cameras: Query<(Entity, &mut Skybox), Without<HasSkyboxSet>>,
) {
    if skybox_resource.converted {
        for (entity, mut skybox) in cameras.iter_mut() {
            skybox.0 = skybox_resource.image.clone();
            commands.try_insert(entity, HasSkyboxSet);
        }
    }
}

fn spawn_world_camera(cameras: Query<Entity, Added<WorldCamera>>, mut commands: Commands) {
    let field_of_view = 80_f32; // degrees; TODO: read from settings
    let aspect_ratio = 1920. / 1080.; // TODO: use real one

    for entity in cameras.iter() {
        commands.try_insert(
            entity,
            (
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        ..default()
                    },
                    projection: PerspectiveProjection {
                        fov: math_algorithms::calculate_vertical_fov(
                            field_of_view.to_radians(),
                            aspect_ratio,
                        ),
                        ..default()
                    }
                    .into(),
                    tonemapping: Tonemapping::TonyMcMapface,
                    ..default()
                },
                BloomSettings::NATURAL,
                Skybox(default()),
                Fxaa::default(),
            ),
        );
    }
}
