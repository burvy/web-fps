use bevy::prelude::*;

/// Resource to store client-side player info
#[derive(Resource, Default)]
pub struct PlayerInfo {
    pub paused: bool,
}

// /// Spawn client-side things
// pub fn player() -> impl Scene {
//     let view = Transform::from_translation(Vec3::new(0.0, 10.0, 20.0));
//     bsn! {
//         #Camera
//         Camera3d
//         template_value(view.looking_at(Vec3::ZERO, Vec3::Y))
//     }
// }
