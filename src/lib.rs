pub mod server_builder;
pub mod client_builder;
pub mod config;
pub mod level;
pub mod game_server;
pub mod game_client;
pub mod network_character_controller;
pub mod instant_event;

use config::PHYSICS_FIXED_TICK_DELTA;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_rapier3d::prelude::*;

pub const BEFORE_PHYSICS_SET: PhysicsSet = PhysicsSet::SyncBackend;
pub const AFTER_PHYSICS_SET: PhysicsSet = PhysicsSet::Writeback;

#[derive(Component, Serialize, Deserialize)]
pub struct NetworkId(ClientId);

impl NetworkId {
    #[inline]
    pub fn new(client_id: ClientId) -> Self {
        Self(client_id)
    }

    #[inline]
    pub fn client_id(&self) -> ClientId {
        self.0
    }
}

#[derive(Event, Serialize, Deserialize, Default, Clone)]
struct NetworkAction {
    linear: Vec2,
    angular: Vec2
}

pub struct GameCommonPlugin;

impl Plugin for GameCommonPlugin {
    fn build(&self, app: &mut App) {
        let mut physics_config = RapierConfiguration
        ::from_world(&mut app.world);
        physics_config.timestep_mode = TimestepMode::Fixed { 
            dt: PHYSICS_FIXED_TICK_DELTA, 
            substeps: 1
        };
        app.insert_resource(physics_config);
        app.add_plugins(
            RapierPhysicsPlugin::<()>::default()
            .in_fixed_schedule()
        )
        .replicate::<NetworkId>()
        .add_client_event::<NetworkAction>(ChannelKind::Ordered);
    }
}