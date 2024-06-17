use bevy::input::mouse::MouseMotion;
use crate::{
    *,
    instant_event::*, 
    level::*,
    client_builder::Client,
    network_character_controller::*,
};

const FORWARD: KeyCode = KeyCode::KeyW;
const LEFT: KeyCode = KeyCode::KeyA;
const BACK: KeyCode = KeyCode::KeyS;
const RIGHT: KeyCode = KeyCode::KeyD;

pub struct GameClientPlugin;

impl Plugin for GameClientPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InstantEvent::<NetworkAction>::new())
        .add_plugins(RapierDebugRenderPlugin::default())
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
        ).in_set(TnuaUserControlsSystemSet));
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
        .insert((
            PbrBundle{
                mesh: meshes.add(Mesh::from(
                    Capsule3d::new(CHARACTER_RADIUS, CHARACTER_HALF_HIGHT * 2.0))
                ),
                material: materials.add(CHARACTER_COLOR),
                transform: Transform{
                    translation: net_cc.translation,
                    rotation: yaw_to_quat(net_cc.yaw),
                    ..default()
                },
                ..default()
            },
            Collider::capsule_y(CHARACTER_HALF_HIGHT, CHARACTER_RADIUS)
        ));

        if client.id() == net_id.client_id().get() {
            commands.entity(e)
            .insert((
                RigidBody::Dynamic,
                TnuaControllerBundle::default(),
                TnuaRapier3dIOBundle::default(),
                TnuaRapier3dSensorShape(
                    Collider::cylinder(0.0, CHARACTER_RADIUS - CHARACTER_OFFSET)
                ),
                LockedAxes::from(
                    LockedAxes::ROTATION_LOCKED_X
                    | LockedAxes::ROTATION_LOCKED_Z
                )
            ));
        } else {
            commands.entity(e)
            .insert(RigidBody::KinematicPositionBased);
        }

        info!("player: {:?} spawned", net_id.client_id())
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
    mut query: Query<&mut TnuaController, With<NetworkCharacterController>>,
    mut actions: ResMut<InstantEvent<NetworkAction>>,
) {
    let Ok(mut cc) = query.get_single_mut() else {
        return;
    };

    for a in actions.drain() {
        update_character(&mut cc, &a);
    }
}

fn draw_net_cc_gizmos_system(
    query: Query<(&NetworkCharacterController, &Transform)>,
    mut gizmos: Gizmos
) {
    for (net_cc, transform) in query.iter() {
        gizmos.cuboid(Transform{
            translation: net_cc.translation,
            rotation: yaw_to_quat(net_cc.yaw),
            ..default()
        }, Color::GREEN);

        info!(
            "CC translation: {} : CC rotation: {}, TF translation: {} : TF rotation: {}", 
            net_cc.translation, 
            net_cc.yaw,
            transform.translation,
            transform.rotation
        );
    }
}
