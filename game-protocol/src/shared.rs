//! Shared
//! Shared game logic/configuration between client and server

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::protocol;

/// Shared walkspeed to ensure similar simulation between client and server
pub const WALKSPEED: f32 = 5.0;

/// Shared jump velocity to ensure client-server similarity
pub const JUMP_VEL: f32 = 10.0; // TODO: should be used when jumping is implemented

/// Shared player radius for simulation similarity
pub const PLAYER_RADIUS: f32 = 0.45;
/// Shared player length for simulation similarity
/// This describes the cylinder section of the player,
/// excluding the caps. The total height is:
/// PLAYER_LENGTH + (2.0 * PLAYER_RADIUS) which is 1.8m
/// Recalculate for the doc if anything changes with these two constants
pub const PLAYER_LENGTH: f32 = 0.9;

/// Shared player body definition to ensure similar simulation between
/// client and server
#[derive(Bundle)]
pub struct PlayerBody {
    body: RigidBody,
    collider: Collider,
    locked: LockedAxes,
}

impl Default for PlayerBody {
    fn default() -> Self {
        Self {
            body: RigidBody::Dynamic,
            collider: Collider::capsule(PLAYER_RADIUS, PLAYER_LENGTH),
            locked: LockedAxes::ROTATION_LOCKED,
        }
    }
}

/// Hosts agreed-upon changes between client and server about how motion inputs
/// are processed. It is essential that there are no disagreements between
/// motion as it will cause client-side rubberbanding.
pub fn apply_input(
    rotation: &mut Rotation,
    velocity: &mut LinearVelocity,
    input: &protocol::PlayerInputs,
) {
    // `PlayerInputs` is responsible for declaring which is forward
    let motion = input.motion.clamp_length_max(1.0) * WALKSPEED;

    // TODO: Allow the camera to look around, but only rotate the body on yaw
    // rotation.x = input.look.x;
    rotation.y = input.look.y;

    // TODO: make the player move in the direction they are looking
    velocity.0.x = motion.x;
    velocity.0.z = motion.y;
    // TODO: add jump input when we get to it
}
