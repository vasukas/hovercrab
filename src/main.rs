use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_egui::EguiPlugin;
use bevy_mod_mipmap_generator::{generate_mipmaps, MipmapGeneratorPlugin, MipmapGeneratorSettings};
use bevy_rapier3d::{prelude::RapierPhysicsPlugin, render::RapierDebugRenderPlugin};

mod gameplay;
mod presentation;
mod tmp;
mod utils;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::Fifo,
                    mode: WindowMode::Windowed,
                    position: WindowPosition::At(IVec2::ZERO),
                    resolution: (1920., 1080.).into(),
                    title: "Hovercrab".to_string(),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            MipmapPlugin {
                anisotropic_filtering: 16,
            },
            RapierPhysicsPlugin::<()>::default(),
            RapierDebugRenderPlugin {
                enabled: true,
                ..default()
            },
            EguiPlugin,
            utils::plugins::UtilPlugins,
            tmp::TmpPlugin,
            gameplay::GameplayPlugin,
            presentation::PresentationPlugin,
        ))
        .insert_resource(GizmoConfig {
            // depth_bias: -1.,
            ..default()
        })
        .run()
}

struct MipmapPlugin {
    anisotropic_filtering: u16,
}

impl Plugin for MipmapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MipmapGeneratorPlugin)
            .insert_resource(MipmapGeneratorSettings {
                anisotropic_filtering: self.anisotropic_filtering,
                minimum_mip_resolution: 16,
                ..default()
            })
            .add_systems(Update, generate_mipmaps::<StandardMaterial>);
    }
}
