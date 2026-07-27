//! Protocol
//! Shared networking structure between client and server.

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use bevy::{ecs::entity::MapEntities, prelude::*};
use serde::{Deserialize, Serialize};

pub const TIMESTEP: f64 = 1.0 / 64.0;
pub const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0);

/// Marker used to mark something as a player (no fields)
/// This struct is bolted onto players as components, and thus must derive Component.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Component)]
pub struct PlayerMarker;

/// Player input packets
/// This rides inside ActionState<PlayerInputs>, with ActionState being the
/// component. Thus, PlayerInputs does not need to derive Component.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Reflect, Default)]
pub struct PlayerInputs {
    pub look: Vec2,
    pub motion: Vec2,
}

/// `MapEntities` exists to satisfy the `InputPlugin`'s trait bound.
/// Even if `PlayerInputs` has no fields that contain entities,
/// `InputPlugin` still demands it.
/// Entity IDs are per world, and when they are sent over the network,
/// they must be remapped to point at the same thing the sender
/// originally had it pointing at.
impl MapEntities for PlayerInputs {
    fn map_entities<M: EntityMapper>(&mut self, _: &mut M) {}
}
