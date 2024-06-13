use bevy::{
    prelude::*,
    input::mouse::MouseMotion 
};
use bevy_rapier3d::prelude::*;
use bevy_replicon::prelude::*;
use network_character_controller::*;
use crate::{
    *,
    instant_event::*, 
    level::*,
    client_builder::Client
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
            client_setup_floor
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
        &NetworkCharacterController,
        &NetworkYaw
    ), 
        Added<NetworkId>
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    client: Res<Client>
) {
    for (e, net_id, net_cc, net_yaw) in query.iter() {
        commands.entity(e)
        .insert((
            PbrBundle{
                mesh: meshes.add(Mesh::from(
                    Capsule3d::new(CHARACTER_RADIUS, CHARACTER_HALF_HIGHT * 2.0))
                ),
                material: materials.add(CHARACTER_COLOR),
                transform: Transform{
                    translation: net_cc.translation,
                    rotation: yaw_to_quat(net_yaw.yaw),
                    ..default()
                },
                ..default()
            },
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(CHARACTER_HALF_HIGHT, CHARACTER_RADIUS)
        ));

        if client.id() == net_id.client_id().get() {
            commands.entity(e)
            .insert(KinematicCharacterController::default());
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

    if action.linear != Vec2::ZERO || action.angular != Vec2::ZERO {
        actions.enqueue(action.clone());
        net_actions.send(action);
    }
}

fn handle_action(
    mut query: Query<(
        &mut KinematicCharacterController,
        &mut Transform
    )>,
    mut actions: ResMut<InstantEvent<NetworkAction>>,
    time: Res<Time<Fixed>>,
) {
    for (mut cc, _) in query.iter_mut() {
        if let Some(_) = cc.translation {
            cc.translation = None;
        }
    }

    for a in actions.drain() {
        for (mut cc, mut transform) in query.iter_mut() {
            update_character(&mut cc, &mut transform, &a, &time);
        }
    }
}

fn draw_net_cc_gizmos_system(
    query: Query<(
        &NetworkCharacterController,
        &NetworkYaw
    )>,
    mut gizmos: Gizmos
) {
    for (net_cc, net_yaw) in query.iter() {
        gizmos.cuboid(Transform{
            translation: net_cc.translation,
            rotation: yaw_to_quat(net_yaw.yaw),
            ..default()
        }, Color::GREEN);
    }
}
