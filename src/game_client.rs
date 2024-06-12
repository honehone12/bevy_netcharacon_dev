use bevy::{
    prelude::*,
    input::mouse::MouseMotion 
};
use bevy_replicon::prelude::*;
use crate::{
    NetworkAction, 
    BEFORE_PHYSICS_SET,
    instant_event::InstantEvent
};

const FORWARD: KeyCode = KeyCode::KeyW;
const LEFT: KeyCode = KeyCode::KeyA;
const BACK: KeyCode = KeyCode::KeyS;
const RIGHT: KeyCode = KeyCode::KeyD;

pub struct GameClientPlugin;

impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InstantEvent::<NetworkAction>::new())
        .add_systems(PreUpdate, 
            monitor_connection_system
            .after(ClientSet::Receive)
        )
        .add_systems(FixedUpdate, (
            handle_input,
            handle_action
        ).chain(
        ).before(BEFORE_PHYSICS_SET));
    }
}

fn monitor_connection_system(
    client: Res<RepliconClient>
) {
    let status = client.status();
    if !matches!(status, RepliconClientStatus::Connected { client_id: _ }) {
        info!("client status: {:?}", status);
    }
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    mut net_actions: EventWriter<NetworkAction>,
    mut actions: ResMut<InstantEvent<NetworkAction>>
) {
    let mut action = NetworkAction::default();

    if keyboard.pressed(FORWARD) {
        action.linear.y += 1.0;
    }
    if keyboard.pressed(BACK) {
        action.linear.y -= 1.0;
    }
    if keyboard.pressed(RIGHT) {
        action.linear.x += 1.0;
    }
    if keyboard.pressed(LEFT) {
        action.linear.x -= 1.0;
    }
    
    for e in mouse.read() {
        action.angular += e.delta;
    }

    actions.enqueue(action.clone());
    net_actions.send(action);
}

fn handle_action(
    mut actions: ResMut<InstantEvent<NetworkAction>>
) {
    for action in actions.drain() {

    }
}