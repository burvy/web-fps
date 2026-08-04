use bevy::prelude::*;
use game_protocol::world::{
    self,
    definition::{BASEPLATE_LENGTH, BASEPLATE_THICKNESS},
};

pub fn build_baseplate(
    mut cmds: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut mats: ResMut<Assets<StandardMaterial>>,
) {
    // world building
    cmds.spawn((
        world::definition::Baseplate::default(),
        Mesh3d(mesh.add(Cuboid::new(
            BASEPLATE_LENGTH,
            BASEPLATE_THICKNESS,
            BASEPLATE_LENGTH,
        ))),
        MeshMaterial3d::<StandardMaterial>(mats.add(Color::WHITE)),
    ));
}
