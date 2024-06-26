use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub const FLOOR_SIZE: Vec3 = Vec3::new(100.0, 1.0, 100.0);
pub const FLOOR_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const FLOOR_POSITION: Vec3 = Vec3::new(0.0, -0.5, 0.0);
pub const LIGHT_POSITION: Vec3 = Vec3::new(0.0, 50.0, 0.0);
pub const LIGHT_ROTATION_X: f32 = -std::f32::consts::PI / 4.0;
pub const CAMERA_POSITION: Vec3 = Vec3::new(0.0, 25.0, 25.0);

pub const BOX_SIZE: Vec3 = Vec3::new(5.0, 5.0, 5.0);
pub const BOX_COLOR: Color = Color::BLUE;
pub const BOX_POSITION_1: Vec3 = Vec3::new(5.0, 2.5, 5.0);
pub const BOX_POSITION_2: Vec3 = Vec3::new(-5.0, 2.5, 5.0);
pub const BOX_POSITION_3: Vec3 = Vec3::new(5.0, 2.5, -5.0);
pub const BOX_POSITION_4: Vec3 = Vec3::new(-5.0, 2.5, -5.0);


pub fn client_setup_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Mesh::from(Cuboid::from_size(FLOOR_SIZE))),
            material: materials.add(FLOOR_COLOR),
            transform: Transform::from_translation(FLOOR_POSITION),
            ..default()
        },
        floor_collider()
    ));
}

pub fn server_setup_floor(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(
            Transform::from_translation(FLOOR_POSITION)
        ),
        floor_collider()
    ));
}

pub fn floor_collider() -> impl Bundle {
    (
        Collider::cuboid(FLOOR_SIZE.x, FLOOR_SIZE.y, FLOOR_SIZE.z),
        RigidBody::Static
    )
}

pub fn setup_light(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight{
            shadows_enabled: true,
            ..default()
        },
        transform: Transform{
            translation: LIGHT_POSITION,
            rotation: Quat::from_rotation_x(LIGHT_ROTATION_X),
            ..default()
        },
        ..default()
    });
}

pub fn setup_fixed_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle{
        transform: Transform::from_translation(CAMERA_POSITION)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn box_collider() -> impl Bundle {
    (
        Collider::cuboid(BOX_SIZE.x, BOX_SIZE.y, BOX_SIZE.z),
        RigidBody::Static
    )
}

pub fn client_setup_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Mesh::from(Cuboid::from_size(BOX_SIZE))),
            material: materials.add(BOX_COLOR),
            transform: Transform::from_translation(BOX_POSITION_1),
            ..default()
        },
        box_collider()
    ));
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Mesh::from(Cuboid::from_size(BOX_SIZE))),
            material: materials.add(BOX_COLOR),
            transform: Transform::from_translation(BOX_POSITION_2),
            ..default()
        },
        box_collider()
    ));
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Mesh::from(Cuboid::from_size(BOX_SIZE))),
            material: materials.add(BOX_COLOR),
            transform: Transform::from_translation(BOX_POSITION_3),
            ..default()
        },
        box_collider()
    ));
    commands.spawn((
        PbrBundle{
            mesh: meshes.add(Mesh::from(Cuboid::from_size(BOX_SIZE))),
            material: materials.add(BOX_COLOR),
            transform: Transform::from_translation(BOX_POSITION_4),
            ..default()
        },
        box_collider()
    ));
}

pub fn server_setup_box(mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(
            Transform::from_translation(BOX_POSITION_1)
        ),
        box_collider()
    ));
    commands.spawn((
        TransformBundle::from_transform(
            Transform::from_translation(BOX_POSITION_2)
        ),
        box_collider()
    ));
    commands.spawn((
        TransformBundle::from_transform(
            Transform::from_translation(BOX_POSITION_3)
        ),
        box_collider()
    ));
    commands.spawn((
        TransformBundle::from_transform(
            Transform::from_translation(BOX_POSITION_4)
        ),
        box_collider()
    ));
}
