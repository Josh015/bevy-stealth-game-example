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

/// Specify what type of movement is required.
#[derive(Clone, Copy, Debug)]
pub enum MoveTo {
    Destination(Vec3),
    Direction(Direction3d),
}

/// Moves the entity and then removes itself.
#[derive(Clone, Component, Debug, new)]
pub struct Mover {
    move_to: MoveTo,

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
        mover.heading = match mover.move_to {
            MoveTo::Destination(destination) => {
                let heading = (destination - transform.translation).normalize();

                commands.entity(entity).insert(Velocity(heading * speed.0));
                heading
            },
            MoveTo::Direction(heading) => *heading,
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
        if let MoveTo::Destination(destination) = mover.move_to {
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
            // transform.rotation =
            //     Quat::from_rotation_arc(FORWARD_DIRECTION, mover.heading);
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
