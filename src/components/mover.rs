use crate::{system_params::*, system_sets::*, util::*};
use bevy::prelude::*;

const MOVING_ANIMATION: &str = "moving";
const DESTINATION_MARGIN_OF_ERROR: f32 = 0.001;

pub(super) struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        // NOTE: Systems will malfunction if order isn't enforced!
        app.add_systems(
            Update,
            (move_to_setup, move_to, translation, rotation)
                .chain()
                .in_set(StopWhenPausedSet),
        );
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
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum MoveTo {
    /// A point this entity is trying to reach.
    Destination(Vec3),

    /// A direction this entity wants to face.
    FaceDirection(Direction3d),

    /// An angle in radians that this entity wants to face.
    Heading(f32),
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

#[derive(Clone, Component, Debug)]
struct Translation {
    destination: Vec3,
}

#[derive(Clone, Component, Debug)]
struct Rotation {
    yaw: f32,
    heading: f32,
}

fn move_to_setup(
    mut commands: Commands,
    mut animations: Animations,
    mut query: Query<(Entity, &mut Mover, &Transform), Changed<Mover>>,
) {
    for (entity, mut mover, transform) in &mut query {
        let mut entity_commands = commands.entity(entity);
        let Some(move_to) = &mover.move_to else {
            entity_commands.remove::<(Translation, Rotation)>();
            continue;
        };

        let heading = match move_to {
            MoveTo::Destination(destination) => {
                let diff = *destination - transform.translation;

                // Translation.
                entity_commands.insert(Translation {
                    destination: *destination,
                });
                diff.x.atan2(diff.z)
            },
            MoveTo::FaceDirection(direction) => direction.x.atan2(direction.z),
            MoveTo::Heading(heading) => wrap_angle(*heading),
        };

        // Rotation.
        entity_commands.insert(Rotation {
            heading,
            yaw: transform.rotation.to_euler(EulerRot::YXZ).0,
        });

        // Save the currently playing animation for later.
        if mover.stored_animation.is_none() {
            if let Some(current_animation) = animations.get_current_clip(entity)
            {
                mover.stored_animation = Some(current_animation);
            }

            animations.play_clip(entity, MOVING_ANIMATION);
        }
    }
}

fn move_to(
    mut animations: Animations,
    mut query: Query<
        (Entity, &mut Mover),
        (Without<Translation>, Without<Rotation>),
    >,
) {
    for (entity, mut mover) in &mut query {
        // Clean up when everything is complete.
        if mover.move_to.is_some() {
            mover.move_to = None;

            // Restore the saved animation.
            if let Some(stored_animation) = &mover.stored_animation {
                animations
                    .play_clip_handle(entity, stored_animation.clone_weak());
                mover.stored_animation = None;
            }
        }
    }
}

fn translation(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &Translation, &mut Transform, &LinearSpeed)>,
) {
    for (entity, translation, mut transform, linear_speed) in &mut query {
        let diff = translation.destination - transform.translation;
        let dir = diff.normalize_or_zero();
        let distance_squared = diff.length_squared();
        let finished = distance_squared <= DESTINATION_MARGIN_OF_ERROR;

        transform.translation = if finished {
            commands.entity(entity).remove::<Translation>();
            translation.destination
        } else {
            transform.translation + dir * linear_speed.0 * time.delta_seconds()
        };
    }
}

fn rotation(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Rotation, &mut Transform, &AngularSpeed)>,
) {
    for (entity, mut rotation, mut transform, angular_speed) in &mut query {
        let diff = wrap_angle(rotation.heading - rotation.yaw);
        let dir = diff.signum();
        let delta = dir * angular_speed.0 * time.delta_seconds();
        let finished = diff.abs() < delta.abs();

        rotation.yaw = if finished {
            commands.entity(entity).remove::<Rotation>();
            rotation.heading
        } else {
            wrap_angle(rotation.yaw + delta)
        };

        transform.rotation = Quat::from_rotation_y(rotation.yaw).normalize();
    }
}
