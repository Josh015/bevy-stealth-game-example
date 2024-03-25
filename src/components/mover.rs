use crate::{system_params::*, system_sets::*};
use bevy::prelude::*;

const MOVING_ANIMATION: &str = "moving";
const DESTINATION_MARGIN_OF_ERROR: f32 = 0.001;
const HEADING_MARGIN_OF_ERROR: f32 = 0.001;

pub(super) struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        // The order is important for correct rotations, so don't mess with it!
        app.add_systems(Update, move_to.in_set(StoppedWhenPausedSet));
    }
}

/// Required components for a [`Mover`] entity.
#[derive(Bundle, Default)]
pub struct MoverBundle {
    pub mover: Mover,
    pub linear_speed: LinearSpeed,
    pub angular_speed: AngularSpeed,
}

/// Moves the entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Mover {
    /// Where the entity needs to move.
    pub move_to: Option<MoveTo>,

    /// Previously running animation from before the movement started.
    pub stored_animation: Option<Handle<AnimationClip>>,
}

/// Makes an entity transform in a specified way.
#[derive(Clone, Component, Debug)]
pub enum MoveTo {
    /// A point this entity is trying to reach.
    Destination(Vec3),

    /// A direction this entity wants to face.
    Heading(Direction3d),
}

/// Linear speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct LinearSpeed(pub f32);

impl Default for LinearSpeed {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug)]
pub struct AngularSpeed(pub f32);

impl Default for AngularSpeed {
    fn default() -> Self {
        Self(std::f32::consts::TAU)
    }
}

fn move_to(
    time: Res<Time>,
    mut animations: Animations,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Mover,
        &LinearSpeed,
        &AngularSpeed,
    )>,
) {
    for (entity, mut transform, mut mover, linear_speed, angular_speed) in
        &mut query
    {
        match (&mover.move_to, &mover.stored_animation) {
            (Some(_), None) => {
                if let Some(current_animation) =
                    animations.get_current_clip(entity)
                {
                    mover.stored_animation = Some(current_animation);
                }

                animations.play_clip(entity, MOVING_ANIMATION);
            },
            (None, Some(stored_animation)) => {
                animations
                    .play_clip_handle(entity, stored_animation.clone_weak());
                mover.stored_animation = None;
            },
            _ => {},
        }

        let Some(move_to) = &mover.move_to else {
            continue;
        };
        let (heading, end_translation) = match move_to {
            MoveTo::Destination(destination) => {
                let diff = *destination - transform.translation;
                let heading = diff.normalize_or_zero();
                let distance_squared = diff.length_squared();
                let end_translation =
                    distance_squared <= DESTINATION_MARGIN_OF_ERROR;

                if end_translation {
                    transform.translation = *destination;
                } else {
                    transform.translation +=
                        heading * linear_speed.0 * time.delta_seconds();
                }

                (heading, end_translation)
            },
            MoveTo::Heading(heading) => (**heading, true),
        };

        // Negate forward() because glTF models typically face +Z axis.
        let forward = -*transform.forward();
        let end_rotation =
            forward.dot(heading).abs() >= 1.0 - HEADING_MARGIN_OF_ERROR;

        if !end_rotation {
            transform.rotation = (transform.rotation
                * Quat::from_axis_angle(
                    forward.cross(heading).normalize_or_zero(),
                    angular_speed.0 * time.delta_seconds(),
                ))
            .normalize();
        }

        // let forward_angle = forward.x.atan2(forward.z);
        // let heading_angle = heading.x.atan2(heading.z);
        // let diff = heading_angle - forward_angle;
        // let dir = diff.signum();
        // let delta = dir * angular_speed.0 * time.delta_seconds();
        // let end_rotation = diff.abs() < delta.abs();

        // println!(
        //     "{} {} {} {}",
        //     forward_angle + delta,
        //     forward_angle,
        //     diff.abs(),
        //     delta.abs()
        // );
        // if end_rotation {
        //     transform.rotation =
        //         Quat::from_axis_angle(Vec3::Y, heading_angle).normalize();
        // } else {
        //     transform.rotation =
        //         Quat::from_axis_angle(Vec3::Y, forward_angle + delta)
        //             .normalize();
        // }

        if end_translation && end_rotation {
            mover.move_to = None;
        }
    }
}
