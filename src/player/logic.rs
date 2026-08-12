use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::player::definition::PlayerInfo;

/// Toggles pausing depending on if Escape is pressed
pub fn toggle_pause(
    mut cursor_options: Single<&mut CursorOptions, With<PrimaryWindow>>,
    mut player_info: ResMut<PlayerInfo>,
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
    mut look_res: PlayerInfo,
    mouse_mot_res: Res<AccumulatedMouseMotion>,
) {
    look_res.update_look(mouse_mot_res.delta);
}
