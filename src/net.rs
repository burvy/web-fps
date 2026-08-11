//! Allow the client to directly connect to the server, receive replicated
//! information, and predict the world client side for instant response rather
//! than waiting for the server.

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bevy::prelude::*;
use game_protocol::{
    protocol::{self, PlayerMarker},
    shared,
};
use lightyear::{
    netcode::{client_plugin::NetcodeConfig, Key, NetcodeClient},
    prelude::{client::ClientPlugins, *},
    webtransport::client::WebTransportClientIo,
};

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClientPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
        app.add_systems(Startup, connect);
        app.add_systems(
            Update,
            (
                add_physics,  // prediction
                draw_players, // draw visually
            ),
        );
    }
}

fn connect(mut cmds: Commands) -> Result {
    let digest = std::fs::read_to_string("digest.txt")?.trim().to_string();
    let auth = Authentication::Manual {
        server_addr: protocol::SERVER_ADDR,
        // TODO: use a more secure id so hackers dont spoof other peoples' ids
        client_id: SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() as u64,
        private_key: Key::default(),
        protocol_id: 0,
    };

    let client = cmds
        .spawn((
            Client::default(),
            LocalAddr(CLIENT_ADDR),
            PeerAddr(protocol::SERVER_ADDR),
            Link::default(),
            ReplicationReceiver,
            NetcodeClient::new(auth, NetcodeConfig::default())?,
            WebTransportClientIo {
                certificate_digest: digest,
            },
            PredictionManager::default(),
        ))
        .id();

    cmds.trigger(Connect { entity: client });
    Ok(())
}

fn add_physics(
    mut cmds: Commands,
    players: Query<Entity, (Added<Predicted>, With<protocol::PlayerMarker>)>,
) {
    players.iter().for_each(|player| {
        cmds.entity(player).insert(shared::PlayerBody::default());
    });
}

fn draw_players(
    mut cmds: Commands,
    players: Query<Entity, (Added<Predicted>, With<PlayerMarker>)>,
) {
    players.iter().for_each(|player| {
        cmds.entity(player).queue_apply_scene(bsn! {
            Mesh3d(asset_value(Capsule3d {
                radius: shared::PLAYER_RADIUS,
                half_length: shared::PLAYER_LENGTH,
            }))
            MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
        });
    });
}
