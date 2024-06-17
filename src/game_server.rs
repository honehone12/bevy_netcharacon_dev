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
            .in_set(TnuaUserControlsSystemSet)
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
                    RigidBody::Dynamic,
                    Collider::capsule_y(CHARACTER_HALF_HIGHT, CHARACTER_RADIUS),
                    TnuaControllerBundle::default(),
                    TnuaRapier3dIOBundle::default(),
                    TnuaRapier3dSensorShape(
                        Collider::cylinder(0.0, CHARACTER_RADIUS - CHARACTER_OFFSET)
                    ),
                    LockedAxes::from(
                        LockedAxes::ROTATION_LOCKED_X 
                        | LockedAxes::ROTATION_LOCKED_Z
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
        &mut TnuaController,
    )>,
    mut actions: EventReader<FromClient<NetworkAction>>,
) {
    for FromClient { client_id, event } in actions.read() {
        for (net_id, mut cc) in query.iter_mut() {
            if *client_id == net_id.client_id() {
                update_character(&mut cc, event);
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
