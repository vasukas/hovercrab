//! Game presentation (graphics, sound, etc) and user interface

use bevy::prelude::*;

pub mod player;

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((player::PlayerPlugin,));
    }
}
