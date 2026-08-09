//! Allow the client to directly connect to the server, receive replicated
//! information, and predict the world client side for instant response rather
//! than waiting for the server.

use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use bevy::prelude::*;
use game_protocol::protocol;
use lightyear::{
    netcode::Key,
    prelude::{client::ClientPlugins, *},
};

const CLIENT_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);

pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ClientPlugins {
            tick_duration: Duration::from_secs_f64(protocol::TIMESTEP),
        });
        app.add_systems(Startup, connect);
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
    Ok(())
}
