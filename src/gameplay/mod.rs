use bevy::prelude::*;

pub mod objects;
pub mod spawn;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((objects::ObjectsPlugin, spawn::SpawnPlugin));
    }
}
