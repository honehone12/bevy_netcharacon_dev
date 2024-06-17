use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_replicon::prelude::*;
use network_character_controller::NetworkYaw;
use crate::{
    *,
    level::*,
    network_character_controller::*
};

pub struct GameServerPlugin;

impl Plugin for GameServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            server_setup_floor,
            server_setup_box
        ))
        .add_systems(PreUpdate, 
            handle_server_event
            .after(ServerSet::Receive)
        )
        .add_systems(FixedUpdate, 
            handle_action
            .before(BEFORE_PHYSICS_SET)
        )
        .add_systems(PostUpdate, 
            handle_character_controller_output
            .before(ServerSet::Send)
        );
    }
}

fn handle_server_event(
    mut commands: Commands,
    mut events: EventReader<ServerEvent>
) {
    for e in events.read() {
        match e {
            ServerEvent::ClientConnected { client_id } => {
                commands.spawn((
                    Replicated,
                    NetworkId::new(*client_id),
                    TransformBundle::from_transform(
                        Transform::from_translation(CHARACTER_SPAWN_POSITION)
                    ),
                    RigidBody::KinematicPositionBased,
                    Collider::capsule_y(CHARACTER_HALF_HIGHT, CHARACTER_RADIUS),
                    KinematicCharacterController::default(),
                    NetworkCharacterController{
                        translation: CHARACTER_SPAWN_POSITION,
                        ..default()
                    },
                    NetworkYaw::default()
                ));

                info!("client: {client_id:?} connected");
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("client: {client_id:?} disconnected with reason: {reason}");
            }
        }
    }
}

fn handle_action(
    mut query: Query<(
        &NetworkId,
        &mut KinematicCharacterController,
        &mut Transform
    )>,
    mut actions: EventReader<FromClient<NetworkAction>>,
    time: Res<Time<Fixed>>
) {
    for (_, mut cc, _) in query.iter_mut() {
        if let Some(_) = cc.translation {
            cc.translation = None;
        }
    }

    for FromClient { client_id, event } in actions.read() {
        for (net_id, mut cc, mut transform) in query.iter_mut() {
            if net_id.client_id() == *client_id {
                update_character(&mut cc, &mut transform, event, &time);
            }
        }
    }
}

fn handle_character_controller_output(
    mut query: Query<(
        &KinematicCharacterControllerOutput,
        &Transform,
        &mut NetworkCharacterController,
        &mut NetworkYaw
    )>
) {
    for (out, transform, mut net_cc, mut net_yaw) in query.iter_mut() {
        let trans = transform.translation;
        let yaw = quat_to_yaw(transform.rotation);

        net_cc.translation = trans;
        net_cc.grounded = out.grounded;
        net_yaw.yaw = yaw;
    }
}
