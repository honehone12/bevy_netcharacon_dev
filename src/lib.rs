pub mod server_builder;
pub mod client_builder;
pub mod config;
pub mod level;
pub mod game_server;
pub mod game_client;
pub mod network_character_controller;
pub mod character_controller;
pub mod instant_event_buffer;

use config::PHYSICS_FIXED_TICK_RATE64;
use network_character_controller::NetworkCharacterControllerPlugin;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub const BEFORE_PHYSICS_SET: PhysicsSet = PhysicsSet::Prepare;
pub const AFTER_PHYSICS_SET: PhysicsSet = PhysicsSet::Sync;

pub const CHARACTER_HIGHT: f32 = 1.0;
pub const CHARACTER_OFFSET: f32 = 0.1;
pub const CHARACTER_RADIUS: f32 = 0.5;
pub const CHARACTER_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 2.0, 0.0);
pub const CHARACTER_COLOR: Color = Color::RED;
pub const CHARACTER_LINEAR_SPEED: f32 = 10.0;
pub const CHARACTER_ANGULAR_SPEED: f32 = 0.2; 

pub const GRAVITY: Vec3 = Vec3::new(
    0.0,
    -9.81,
    0.0
);

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
    pub linear: Vec2,
    pub angular: Vec2,
    pub jump: bool
}

pub struct GameCommonPlugin;

impl Plugin for GameCommonPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            Time::new_with(Physics::fixed_hz(PHYSICS_FIXED_TICK_RATE64))
        )
        .insert_resource(SubstepCount(1))
        .add_plugins((
            PhysicsPlugins::new(FixedUpdate),
            NetworkCharacterControllerPlugin
        ))
        .replicate::<NetworkId>()
        .add_client_event::<NetworkAction>(ChannelKind::Unreliable);
    }
}

#[inline]
pub fn yaw_to_quat(y: f32) -> Quat {
    Quat::from_rotation_y(y)
}

#[inline]
pub fn quat_to_yaw(q: Quat) -> f32 {
    q.to_euler(EulerRot::YXZ).0
}
