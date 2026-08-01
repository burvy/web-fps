use std::time::Duration;

use avian3d::physics_transform::Position;
use bevy::prelude::*;
use game_protocol::{protocol, shared};
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
    let digest_hex = digest.to_string().replace(":", ""); // human to machine readable
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

    cmds.trigger(Start { entity: server });
    Ok(())
}

fn on_connect(
    trigger: On<Add, Connected>, // triggers when someone connected
    query: Query<&RemoteId, With<ClientOf>>, // query `RemoteId`s that connected to us
    mut cmds: Commands,
) {
    let Ok(remote_id) = query.get(trigger.entity) else {
        info!("Couldn't get remote id of player!");
        return;
    };

    // `ControlledBy` searches for `With<ReplicationSender>`, so this is required
    cmds.entity(trigger.entity).insert(ReplicationSender);
    cmds.spawn((
        // for player querying/identification
        protocol::PlayerMarker,
        // spawnpoint
        Position(Vec3::new((remote_id.to_bits() % 10) as f32, 0.0, 0.0)), // avian pos
        // agreed-upon player body shape between client and server
        shared::PlayerBody::default(),
        // replicate for everyone
        Replicate::to_clients(NetworkTarget::All),
    ));
}
