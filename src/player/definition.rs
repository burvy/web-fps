use avian3d::prelude::*;
use bevy::prelude::*;
use game_protocol::shared;

pub fn player() -> impl Scene {
    bsn! {
        (
            #Player
            Camera3d
            Mesh3d(asset_value(Capsule3d::new(
                shared::PLAYER_RADIUS,
                shared::PLAYER_LENGTH
            )))
            MeshMaterial3d<StandardMaterial>(asset_value(Color::WHITE))
            Transform::from_translation(Vec3 { x: 0.0, y: 5.0, z: 0.0 })
            Collider::capsule(0.5, 2.0)
        )
    }
}
