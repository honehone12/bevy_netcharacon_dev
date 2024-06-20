use character_controller::CharacterControllerBundle;
use instant_event_buffer::InstantEventBuffer;

use crate::{
    *,
    level::*,
    network_character_controller::*,
    character_controller::*
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
                    LockedAxes::new().lock_rotation_x().lock_rotation_z(),
                    CharacterControllerBundle::new(
                        Collider::capsule(CHARACTER_HIGHT, CHARACTER_RADIUS),
                        GRAVITY
                    ),
                    NetworkCharacterController{
                        translation: CHARACTER_SPAWN_POSITION,
                        ..default()
                    }
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
        &mut InstantEventBuffer<ControllerAction>
    )>,
    mut actions: EventReader<FromClient<NetworkAction>>,
) {
    for FromClient { client_id, event: action } in actions.read() {
        for (net_id, mut controls) in query.iter_mut() {
            if *client_id == net_id.client_id() {
                if action.linear != Vec2::ZERO {
                    controls.send(ControllerAction::Move(action.linear.normalize()));
                }
                if action.jump {
                    controls.send(ControllerAction::Jump);
                }
            }
        }
    }
}

fn handle_character_controller_output(
    mut query: Query<(
        &Transform,
        &mut NetworkCharacterController
    )>
) {
    for (transform, mut net_cc) in query.iter_mut() {
        let trans = transform.translation;
        let yaw = quat_to_yaw(transform.rotation);

        net_cc.translation = trans;
        net_cc.yaw = yaw;
    }
}
