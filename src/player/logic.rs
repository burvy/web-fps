use avian3d::prelude::*;
use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};
use game_protocol::shared;
use lightyear::prelude::*;

use crate::player::definition;

/// Toggles pausing depending on if Escape is pressed
pub fn toggle_pause(
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut player_info: ResMut<definition::PlayerInfo>,
    key: Res<ButtonInput<KeyCode>>,
) {
    if key.just_pressed(KeyCode::Escape) {
        // dont need to configure
        player_info.paused = !player_info.paused;
        match player_info.paused {
            true => {
                cursor_options.grab_mode = CursorGrabMode::None;
                cursor_options.visible = true;
            }
            false => {
                cursor_options.grab_mode = CursorGrabMode::Locked;
                cursor_options.visible = false;
            }
        }
    }
}

/// Stores the accumulated mouse motion in the PlayerInfo resource
pub fn rotate_player_resource(
    mut look_res: ResMut<definition::PlayerInfo>,
    mouse_mot_res: Res<AccumulatedMouseMotion>,
) {
    look_res.update_look(mouse_mot_res.delta);
}

pub fn camera_follow_player(
    mut camera: Single<&mut Transform, With<definition::MainPlayerCamera>>,
    player: Single<&Position, With<Controlled>>,
    look_res: Res<definition::PlayerInfo>,
) {
    camera.translation = player.0 + Vec3::Y * shared::EYE_HEIGHT;
    camera.rotation = Quat::from_euler(EulerRot::YXZ, look_res.look.x, look_res.look.y, 0.0);
}
