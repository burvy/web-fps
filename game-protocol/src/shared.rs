//! Shared
//! Shared game logic/configuration between client and server

use avian3d::prelude::*;
use bevy::prelude::*;

/// Shared walkspeed to ensure similar simulation between client and server
pub const WALKSPEED: f32 = 5.0;

/// Shared player body definition to ensure similar simulation between client and server
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
            collider: Collider::capsule(0.5, 2.0),
            // TODO: Unlock rotation once base is set up
            locked: LockedAxes::ROTATION_LOCKED.lock_rotation_y(),
        }
    }
}
