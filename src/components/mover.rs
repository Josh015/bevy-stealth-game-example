use crate::{system_params::*, system_sets::*, util::*};
use bevy::prelude::*;

const MOVING_ANIMATION: &str = "moving";
const DESTINATION_MARGIN_OF_ERROR: f32 = 0.001;

pub(super) struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
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
    current_rotation: f32,
    move_to: Option<MoveTo>,
    stored_animation: Option<Handle<AnimationClip>>,
}

impl Mover {
    /// Set the type of movement to execute.
    pub fn set_move_to(&mut self, move_to: MoveTo) {
        self.move_to = Some(move_to);
    }

    /// Cancel the current movement.
    pub fn cancel_move_to(&mut self) {
        self.move_to = None;
    }

    /// Check if there is movement occurring.
    pub fn is_moving(&self) -> bool {
        self.move_to.is_some()
    }
}

/// Makes an entity transform in a specified way.
#[derive(Clone, Debug)]
pub enum MoveTo {
    /// A point this entity is trying to reach.
    Destination(Vec3),

    /// A direction this entity wants to face.
    FaceDirection(Direction3d),
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
                // Save the currently playing animation for later.
                if let Some(current_animation) =
                    animations.get_current_clip(entity)
                {
                    mover.stored_animation = Some(current_animation);
                }

                animations.play_clip(entity, MOVING_ANIMATION);

                // Extract and store current yaw.
                let (yaw, _, _) = transform.rotation.to_euler(EulerRot::YXZ);

                mover.current_rotation = yaw;
            },
            (None, Some(stored_animation)) => {
                // Restore the saved animation.
                animations
                    .play_clip_handle(entity, stored_animation.clone_weak());
                mover.stored_animation = None;
            },
            _ => {},
        }

        let Some(move_to) = &mover.move_to else {
            continue;
        };

        // Translations.
        let (direction, end_translation) = match move_to {
            MoveTo::Destination(destination) => {
                let diff = *destination - transform.translation;
                let dir = diff.normalize_or_zero();
                let distance_squared = diff.length_squared();
                let end_translation =
                    distance_squared <= DESTINATION_MARGIN_OF_ERROR;

                if end_translation {
                    transform.translation = *destination;
                } else {
                    transform.translation +=
                        dir * linear_speed.0 * time.delta_seconds();
                }

                //println!("dir {}", dir);

                (dir, end_translation)
            },
            MoveTo::FaceDirection(direction) => (**direction, true),
        };

        // Rotations.
        let end_rotation = if direction == Vec3::ZERO {
            true
        } else {
            let heading = direction.x.atan2(direction.z);
            let diff = wrap_angle(heading - mover.current_rotation);
            let dir = diff.signum();
            let delta = dir * angular_speed.0 * time.delta_seconds();
            let end_rotation = diff.abs() < delta.abs();

            //println!("heading {}", heading);

            mover.current_rotation = if end_rotation {
                heading
            } else {
                wrap_angle(mover.current_rotation + delta)
            };

            transform.rotation =
                Quat::from_rotation_y(mover.current_rotation).normalize();

            end_rotation
        };

        // Clean up when everything is complete.
        if end_translation && end_rotation {
            mover.move_to = None;
        }
    }
}
