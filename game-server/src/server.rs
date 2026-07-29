use bevy::prelude::*;
use game_protocol::protocol;
use lightyear::{
    connection::server::Start,
    netcode::{server_plugin::NetcodeConfig, NetcodeServer},
    prelude::{Identity, LocalAddr},
    webtransport::server::WebTransportServerIo,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
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
