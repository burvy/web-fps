use std::time::Duration;

use avian3d::dynamics::rigid_body::LinearVelocity;
use avian3d::physics_transform::{Position, Rotation};
use bevy::prelude::*;
use game_protocol::{protocol, shared, world};
use lightyear::input::server::{authorize_controlled_targets, InputValidationAppExt};
use lightyear::prelude::input::native::{ActionState, NativeStateSequence};
use lightyear::prelude::server::ServerPlugins;
use lightyear::prelude::*;
use lightyear::{
    connection::{client::Connected, client_of::ClientOf, server::Start},
    core::id::RemoteId,
    netcode::{server_plugin::NetcodeConfig, NetcodeServer},
    webtransport::server::WebTransportServerIo,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_plugins(ServerPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
        app.add_observer(on_connect);

        app.add_systems(FixedUpdate, server_player_motion);

        // this is not default, but prevents clients from controlling
        // entities they shouldn't be on the server
        app.add_input_validator(
            authorize_controlled_targets::<NativeStateSequence<protocol::PlayerInputs>>,
        );
    }
}

fn startup(mut cmds: Commands) -> Result {
    let valid_addresses = vec![
        "localhost".to_string(),
        "127.0.0.1".to_string(),
        "::1".to_string(),
    ];
    let identity = Identity::self_signed(valid_addresses)?;
    let digest = identity.certificate_chain().as_slice()[0].hash();
    let digest_hex = digest.to_string().replace(":", "");
    std::fs::write("digest.txt", &digest_hex)?;
    info!("digest written: {}", digest);

    let server = cmds
        .spawn((
            NetcodeServer::new(NetcodeConfig::default()),
            LocalAddr(protocol::SERVER_ADDR),
            WebTransportServerIo {
                certificate: identity,
            },
        ))
        .id();

    // world building
    cmds.spawn(world::definition::Baseplate::default());

    cmds.trigger(Start { entity: server });
    Ok(())
}

fn on_connect(
    trigger: On<Add, Connected>, // triggers when someone connected
    query: Query<&RemoteId, With<ClientOf>>, // query `RemoteId`s that connected to us
    mut cmds: Commands,
) {
    let Ok(remote_id) = query.get(trigger.entity) else {
        // client connected that isn't a client of us
        return;
    };

    // `ControlledBy` searches for `With<ReplicationSender>`, so this is required
    cmds.entity(trigger.entity).insert(ReplicationSender);
    cmds.spawn((
        // for player querying/identification
        protocol::PlayerMarker,
        // TODO: replace this spawnpoint with actual set spawn later on
        Position(Vec3::new((remote_id.to_bits() % 10) as f32, 2.0, 0.0)),
        // agreed-upon player body shape between client and server
        shared::PlayerBody::default(),
        // replicate for everyone
        Replicate::to_clients(NetworkTarget::All),
        // make all players predict
        PredictionTarget::to_clients(NetworkTarget::All),
        // Server puts `ControlledBy` on player entity, which is *replicated*
        // to client, who gets a `Controlled` marker. The client then writes
        // `ActionState<PlayerInputs>` into the `Controlled` entity, ships it
        // to the server while tagged with the ID, which is mapped back to the
        // server entity's id where its own `ActionState` is written.
        // `movement` reads and acts on the written `ActionState`
        ControlledBy {
            owner: trigger.entity,        // connection that owns this entity
            lifetime: Default::default(), // despawn upon disconnect
        },
    ));
}

fn server_player_motion(
    mut players: Query<(
        &mut Rotation,
        &mut LinearVelocity,
        Entity, // TODO: Remove logging
        &ActionState<protocol::PlayerInputs>,
    )>,
) {
    players
        .iter_mut()
        .for_each(|(mut rot, mut vel, entity, action)| {
            if action.motion != Vec2::ZERO {
                // TODO: Remove logging
                print!("entity: {:?} is moving: {:?}\n", entity, action.motion);
            }
            shared::apply_input(&mut rot, &mut vel, &action.0);
        })
}
