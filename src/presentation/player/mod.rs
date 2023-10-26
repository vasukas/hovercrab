use bevy::prelude::*;

pub mod actions;
pub mod camera;
pub mod mouselook;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            mouselook::MouselookPlugin,
            actions::ActionsPlugin,
        ));
    }
}
