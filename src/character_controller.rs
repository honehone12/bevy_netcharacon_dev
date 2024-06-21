// this is server supported version of
// https://github.com/Jondolf/bevy_xpbd/tree/main/crates/bevy_xpbd_3d/examples/kinematic_character_3d
// server have to operate multiple character controller

use std::f32::{consts::PI, EPSILON};
use bevy::prelude::*;
use bevy_xpbd_3d::{
    prelude::*, 
    SubstepSchedule, 
    SubstepSet
};
use crate::{
    config::PHYSICS_SUBSTEP, 
    instant_event_buffer::InstantEventBuffer
};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ControllerAction>()
        .add_systems(SubstepSchedule, (
            control_system,
            apply_gravity_system,
            apply_movement_damping_system
        ).chain(
        ).before(SubstepSet::Integrate))
        .add_systems(SubstepSchedule, 
            kinematic_collisions_system
            .in_set(SubstepSet::SolveUserConstraints)
        )
        .add_systems(SubstepSchedule,
            update_grounded_system
            .after(SubstepSet::ApplyTranslation)
        );
    }
}

#[derive(Event)]
pub enum ControllerAction {
    Move(Vec2),
    Jump
}

#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct Acceleration(f32);

#[derive(Component)]
pub struct DampingFactor(f32);

#[derive(Component)]
pub struct JumpImpulse(f32);

#[derive(Component)]
pub struct Gravity(Vec3);

#[derive(Component)]
pub struct MaxSlopeAngle(f32);

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    controller_action_buffer: InstantEventBuffer<ControllerAction>,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: Gravity,
    movement: MovementBundle
}

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: Acceleration,
    damping_factor: DampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle
}

const DEFAULT_ACCELERATION: Acceleration = Acceleration(60.0 * PHYSICS_SUBSTEP);
const DEFAULT_DAMPING_FACTOR: DampingFactor = DampingFactor(0.98);
const DEFAULT_JUMP_IMPULSE: JumpImpulse = JumpImpulse(9.0);
const DEFAULT_MAX_SLOPE_ANGLE: MaxSlopeAngle = MaxSlopeAngle(PI * 0.45);

impl Default for MovementBundle {
    fn default() -> Self {
        Self { 
            acceleration: DEFAULT_ACCELERATION, 
            damping_factor: DEFAULT_DAMPING_FACTOR, 
            jump_impulse: DEFAULT_JUMP_IMPULSE, 
            max_slope_angle: DEFAULT_MAX_SLOPE_ANGLE 
        }
    }
}

impl CharacterControllerBundle {
    #[inline]
    pub fn new(collider: Collider, gravity: Vec3) -> Self {
        const SCALE: f32 = 0.99;
        const SUBDIVISIONS: u32 = 10;
        const MAX_CAST_TIME: f32 = 0.2;

        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vec3::ONE * SCALE, SUBDIVISIONS);

        Self{
            character_controller: CharacterController,
            controller_action_buffer: InstantEventBuffer::<ControllerAction>::new(),
            rigid_body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape, 
                Vec3::ZERO, 
                Quat::IDENTITY, 
                Direction3d::NEG_Y
            ).with_max_time_of_impact(MAX_CAST_TIME),
            gravity: Gravity(gravity),
            movement: MovementBundle::default()
        }
    }

    #[inline]
    pub fn with_movement(
        mut self,
        acceleration: f32,
        damping_factor: f32,
        jump_impulse: f32,
        max_slope_angle: f32
    ) -> Self {
        self.movement = MovementBundle{
            acceleration: Acceleration(acceleration),
            damping_factor: DampingFactor(damping_factor),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle)
        };
        self
    }
}

fn update_grounded_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &ShapeHits,
        &Rotation,
        Option<&MaxSlopeAngle>
    ),
        With<CharacterController>
    >
) {
    for (e, hits, rot, slope_angle) in query.iter_mut() {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = slope_angle {
                rot.rotate(-hit.normal2)
                .angle_between(Vec3::Y)
                .abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(e)
            .insert(Grounded);
        } else {
            commands.entity(e)
            .remove::<Grounded>();
        }
    }
}

fn control_system(
    mut query: Query<(
        &mut InstantEventBuffer<ControllerAction>,
        &Acceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>
    )>,
    time: Res<Time>
) {
    if query.is_empty() {
        return;
    }

    let delta_time = time.delta_seconds();

    for (mut controls, accel, jump, mut vel, is_grounded) in query.iter_mut() {
        for control in controls.read() {
            match control {
                ControllerAction::Move(dir) => {
                    vel.x += dir.x * accel.0 * delta_time;
                    vel.z -= dir.y * accel.0 * delta_time;
                }
                ControllerAction::Jump => {
                    if is_grounded {
                        vel.y = jump.0;
                    }
                }
            }
        }

        info!("vel: {}, delta time: {}", vel.0, delta_time);    
    }
}

fn apply_gravity_system(
    mut query: Query<(&Gravity, &mut LinearVelocity)>,
    time: Res<Time>
) {
    if query.is_empty() {
        return;
    }

    let delta_time = time.delta_seconds();
    
    for (gravity, mut vel) in query.iter_mut() {
        vel.0 += gravity.0 * delta_time;
    }
}

fn apply_movement_damping_system(
    mut query: Query<(&DampingFactor, &mut LinearVelocity)>
) {
    for (damp, mut vel) in query.iter_mut() {
        vel.x *= damp.0;
        if vel.x.abs() <= EPSILON {
            vel.x = 0.0;
        }
        vel.z *= damp.0;
        if vel.z.abs() <= EPSILON {
            vel.z = 0.0;
        }
    }
}

fn kinematic_collisions_system(
    mut ccs: Query<(
        &RigidBody,
        &mut Position,
        &Rotation,
        &mut LinearVelocity,
        Option<&MaxSlopeAngle>
    ),
        With<CharacterController>
    >,
    col_parents: Query<&ColliderParent, Without<Sensor>>,
    collisions: Res<Collisions>
) {
    for contacts in collisions.iter() {
        if !contacts.during_current_substep {
            continue;
        }

        let Ok([col_parent_1, col_parent_2]) = col_parents.get_many(
            [contacts.entity1, contacts.entity2]
        ) else {
            continue;
        };

        let (
            is_first, 
            (rb, mut pos, rot, mut vel, slope_angle)
        ) = if let Ok(chara) = ccs.get_mut(col_parent_1.get()) {
            (true, chara)
        } else if let Ok(chara) = ccs.get_mut(col_parent_2.get()) {
            (false, chara)
        } else {
            continue;
        };

        if !rb.is_kinematic() {
            continue;
        }

        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rot)
            } else {
                -manifold.global_normal2(rot)
            };

            for contact in manifold.contacts.iter()
            .filter(|c| c.penetration > 0.0) {
                pos.0 += normal * contact.penetration;
            }

            if slope_angle.is_some_and(|angle| {
                normal.angle_between(Vec3::Y)
                .abs() <= angle.0
            }) && vel.y < 0.0 {
                vel.y = vel.y.max(0.0);
            }
        }
    }
}
