use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::character_controller::CharacterControllerPlugin;

#[derive(Component, Serialize, Deserialize, Default)]
pub struct NetworkCharacterController {
    pub translation: Vec3,
    pub yaw: f32
}

pub struct NetworkCharacterControllerPlugin;

impl Plugin for NetworkCharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterControllerPlugin)
        .replicate::<NetworkCharacterController>();
    }
}
