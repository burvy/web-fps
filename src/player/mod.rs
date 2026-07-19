use bevy::prelude::*;

pub mod definition;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, definition::player.spawn());
    }
}
