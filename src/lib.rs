pub mod server_builder;
pub mod client_builder;
pub mod config;
pub mod level;
pub mod game_server;
pub mod game_client;
pub mod network_character_controller;
pub mod instant_event;

use config::PHYSICS_FIXED_TICK_DELTA;
use network_character_controller::NetworkCharacterControllerPlugin;
use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_tnua_rapier3d::*;
use bevy_tnua::prelude::*;

pub const BEFORE_PHYSICS_SET: PhysicsSet = PhysicsSet::SyncBackend;
pub const AFTER_PHYSICS_SET: PhysicsSet = PhysicsSet::Writeback;

pub const CHARACTER_HALF_HIGHT: f32 = 0.5;
pub const CHARACTER_OFFSET: f32 = 0.1;
pub const CHARACTER_RADIUS: f32 = 0.5;
pub const CHARACTER_SPAWN_POSITION: Vec3 = Vec3::new(0.0, 2.0, 0.0);
pub const CHARACTER_COLOR: Color = Color::RED;
pub const CHARACTER_LINEAR_SPEED: f32 = 10.0;
pub const CHARACTER_ANGULAR_SPEED: f32 = 0.2; 

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
    pub angular: Vec2
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
        app.add_plugins((
            RapierPhysicsPlugin::<()>::default().in_fixed_schedule(),
            TnuaRapier3dPlugin::new(FixedUpdate),
            TnuaControllerPlugin::new(FixedUpdate),
            NetworkCharacterControllerPlugin
        ))
        .replicate::<NetworkId>()
        .add_client_event::<NetworkAction>(ChannelKind::Unreliable);
    }
}

pub(crate) fn update_character(
    cc: &mut TnuaController,
    action: &NetworkAction
) {
    let linear = Vec3::new(action.linear.x, 0.0, -action.linear.y)
    .normalize_or_zero() * CHARACTER_LINEAR_SPEED;
    
    let basis = TnuaBuiltinWalk{
        desired_velocity: linear,
        float_height: (CHARACTER_HALF_HIGHT * 2.0) + CHARACTER_OFFSET,
        ..default()
    };
    cc.basis(basis);
}

#[inline]
pub fn yaw_to_quat(y: f32) -> Quat {
    Quat::from_rotation_y(y)
}

#[inline]
pub fn quat_to_yaw(q: Quat) -> f32 {
    q.to_euler(EulerRot::YXZ).0
}
