use bevy::{app::AppExit, prelude::*};

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (exit_on_ctrl_q,));
    }
}

fn exit_on_ctrl_q(keys: Res<Input<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Q) && keys.pressed(KeyCode::ControlLeft) {
        exit.send_default()
    }
}
