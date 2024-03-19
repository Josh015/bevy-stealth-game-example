use bevy::prelude::*;
use derive_new::new;

use crate::{AngularSpeed, AngularVelocity, Speed, Velocity};

pub const FORWARD_DIRECTION: Vec3 = Vec3::NEG_Z;
pub const MOTION_MARGIN_OF_ERROR: f32 = 0.01;

pub(super) struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (start_mover, mover, clean_up_mover).chain());
    }
}

/// Means of imparting motion.
#[derive(Clone, Debug)]
pub enum Motion {
    Destination(Vec3),
    Heading(Direction3d),
}

/// Component that translates and rotates the entity.
#[derive(Clone, Component, Debug, new)]
pub struct Mover {
    motion: Motion,

    #[new(default)]
    heading: Vec3,
}

fn start_mover(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Speed, &AngularSpeed, &mut Mover, &Transform),
        Added<Mover>,
    >,
) {
    for (entity, speed, angular_speed, mut mover, transform) in &mut query {
        mover.heading = match mover.motion {
            Motion::Destination(destination) => {
                let heading = (destination - transform.translation).normalize();

                commands.entity(entity).insert(Velocity(heading * speed.0));
                heading
            },
            Motion::Heading(heading) => *heading,
        };

        commands.entity(entity).insert(AngularVelocity {
            axis: Direction3d::new_unchecked(
                (*transform.forward()).cross(mover.heading).normalize(),
            ),
            velocity: angular_speed.0,
        });
    }
}

fn mover(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Mover,
        &mut Transform,
        Has<Velocity>,
        Has<AngularVelocity>,
    )>,
) {
    for (entity, mover, mut transform, has_velocity, has_angular_velocity) in
        &mut query
    {
        if let Motion::Destination(destination) = mover.motion {
            if has_velocity
                && destination.distance(transform.translation)
                    <= MOTION_MARGIN_OF_ERROR
            {
                commands.entity(entity).remove::<Velocity>();
                transform.translation = destination;
            }
        }

        if has_angular_velocity
            && transform.forward().dot(mover.heading).abs()
                >= 1.0 - MOTION_MARGIN_OF_ERROR
        {
            commands.entity(entity).remove::<AngularVelocity>();
            transform.rotation =
                Quat::from_rotation_arc(FORWARD_DIRECTION, mover.heading);
        }

        if !has_velocity && !has_angular_velocity {
            commands.entity(entity).remove::<Mover>();
            return;
        }
    }
}

fn clean_up_mover(
    mut commands: Commands,
    mut removed: RemovedComponents<Mover>,
) {
    for entity in removed.read() {
        commands
            .entity(entity)
            .remove::<(Velocity, AngularVelocity)>();
    }
}
