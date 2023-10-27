//! Hovercrab

use crate::{gameplay::objects::hovercrab::Hovercrab, utils::for_crate::bevy::FallibleCommands};
use bevy::prelude::*;

pub struct HovercrabPlugin;

impl Plugin for HovercrabPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_hovercrab,));
    }
}

fn spawn_hovercrab(
    mut commands: Commands,
    entities: Query<Entity, Added<Hovercrab>>,
    asset_server: Res<AssetServer>,
) {
    for entity in entities.iter() {
        let scene = asset_server.load("models/hovercrab.glb#Scene0");
        commands.try_with_children(entity, |parent| {
            parent.spawn(SceneBundle { scene, ..default() });
        });
    }
}
