use bevy::prelude::*;

pub fn player() -> impl Scene {
    let view = Transform::from_translation(Vec3::new(0.0, 10.0, 20.0));
    bsn! {
        #Camera
        Camera3d
        template_value(view.looking_at(Vec3::ZERO, Vec3::Y))
    }
}
