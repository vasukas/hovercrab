//! Utility plugins

use bevy::prelude::*;

pub mod scene_utils;

pub struct UtilPlugins;

impl Plugin for UtilPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((scene_utils::SceneUtilsPlugin,));
    }
}
