use std::f32::consts::FRAC_PI_2;

use avian3d::prelude::*;
use bevy::prelude::*;

pub fn world() -> impl SceneList {
    bsn_list! [
        (
            #Ground
            Mesh3d(asset_value(Circle::new(10.0)))
            MeshMaterial3d<StandardMaterial>(asset_value(Color::WHITE))
            Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2))
            Collider::cylinder(10.0, 0.1)
        ),
    ]
}
