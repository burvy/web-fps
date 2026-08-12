use bevy::prelude::*;

pub mod definition;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // This only spawned a camera and we spawn a camera in `net.rs`
        // app.add_systems(Startup, definition::player.spawn());
    }
}
