use bevy::input::mouse::MouseMotion;
use crate::{
    *,
    character_controller::*,
    instant_event_buffer::*, 
    level::*,
    client_builder::Client,
    network_character_controller::*,
};

const FORWARD: KeyCode = KeyCode::KeyW;
const LEFT: KeyCode = KeyCode::KeyA;
const BACK: KeyCode = KeyCode::KeyS;
const RIGHT: KeyCode = KeyCode::KeyD;
const JUMP: KeyCode = KeyCode::Space;

pub struct GameClientPlugin;

impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsDebugPlugin::default())
        .add_systems(Startup, (
            setup_light,
            setup_fixed_camera,
            client_setup_floor,
            client_setup_box
        ))
        .add_systems(PreUpdate, (
            monitor_connection_system,
            handle_player_spawn,
            draw_net_cc_gizmos_system
        ).chain(
        ).after(ClientSet::Receive))
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

fn handle_player_spawn(
    mut commands: Commands,
    query: Query<(
        Entity, 
        &NetworkId,
        &NetworkCharacterController
    ), 
        Added<NetworkId>
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    client: Res<Client>
) {
    for (e, net_id, net_cc) in query.iter() {
        commands.entity(e)
        .insert(
            PbrBundle{
                mesh: meshes.add(Mesh::from(
                    Capsule3d::new(CHARACTER_RADIUS, CHARACTER_HIGHT * 0.5))
                ),
                material: materials.add(CHARACTER_COLOR),
                transform: Transform{
                    translation: net_cc.translation,
                    rotation: yaw_to_quat(net_cc.yaw),
                    ..default()
                },
                ..default()
            }
        );

        if client.id() == net_id.client_id().get() {
            commands.entity(e)
            .insert((
                InstantEventBuffer::<NetworkAction>::new(),
                LockedAxes::new().lock_rotation_x().lock_rotation_z(),
                CharacterControllerBundle::new(
                    Collider::capsule(CHARACTER_HIGHT, CHARACTER_RADIUS),
                    GRAVITY
                )
            ));
        } else {
            commands.entity(e)
            .insert((
                RigidBody::Kinematic,
                Collider::capsule(CHARACTER_HIGHT, CHARACTER_RADIUS)
            ));
        }

        info!("player: {:?} spawned", net_id.client_id())
    }
}

fn handle_input(
    mut query: Query<&mut InstantEventBuffer<NetworkAction>>, 
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    mut net_actions: EventWriter<NetworkAction>
) {
    let Ok(ref mut actions) = query.get_single_mut() else {
        return;
    };

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
    if keyboard.just_pressed(JUMP) {
        action.jump = true;
    }

    for e in mouse.read() {
        action.angular += e.delta;
    }

    actions.send(action.clone());
    net_actions.send(action);
}

fn handle_action(
    mut query: Query<(
        &mut InstantEventBuffer<NetworkAction>,
        &mut InstantEventBuffer<ControllerAction>
    )>
) {
    let Ok((
        ref mut actions, 
        ref mut controls
    )) = query.get_single_mut() else {
        return;
    };

    for a in actions.read() {
        if a.linear != Vec2::ZERO {
            controls.send(ControllerAction::Move(a.linear.normalize()));
        }
        if a.jump {
            controls.send(ControllerAction::Jump);
        }
    }
}

fn draw_net_cc_gizmos_system(
    query: Query<&NetworkCharacterController>,
    mut gizmos: Gizmos
) {
    for net_cc in query.iter() {
        gizmos.cuboid(Transform{
            translation: net_cc.translation,
            rotation: yaw_to_quat(net_cc.yaw),
            ..default()
        }, Color::GREEN);

        // info!(
        //     "trans: {} : rot: {}", 
        //     net_cc.translation, 
        //     net_cc.yaw,
        // );
    }
}
