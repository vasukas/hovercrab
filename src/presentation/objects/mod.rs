//! Graphics for specific objects

use bevy::prelude::*;

pub mod hovercrab;

pub struct ObjectsPlugin;

impl Plugin for ObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((hovercrab::HovercrabPlugin,));
    }
}
