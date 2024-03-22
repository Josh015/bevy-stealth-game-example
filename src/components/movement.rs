use bevy::prelude::*;

use super::Animations;

const DESTINATION_MARGIN_OF_ERROR: f32 = 0.01;
const HEADING_MARGIN_OF_ERROR: f32 = 0.001;
const MOVING_ANIMATION: &str = "moving";

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // The order is important for correct rotations, so don't mess with it!
        app.add_systems(
            Update,
            (movement_setup, movement_check_progress, destination_cleanup)
                .chain(),
        );
    }
}

#[derive(Clone, Component, Debug)]
pub enum Movement {
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

/// Stores currently running animation for later restoration.
#[derive(Clone, Component, Debug, Default)]
pub struct StoredAnimation(pub Handle<AnimationClip>);

fn movement_setup(
    mut commands: Commands,
    mut animations: Animations,
    mut query: Query<Entity, Added<Movement>>,
) {
    for entity in &mut query {
        let mut entity_commands = commands.entity(entity);

        if let Some(current_animation) = animations.get_current_clip(entity) {
            entity_commands.insert(StoredAnimation(current_animation));
        }

        animations.play_clip(entity, MOVING_ANIMATION);
    }
}

fn movement_check_progress(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &Movement,
        &LinearSpeed,
        &AngularSpeed,
    )>,
) {
    for (entity, mut transform, movement, linear_speed, angular_speed) in
        &mut query
    {
        let mut entity_commands = commands.entity(entity);
        let (heading, end_translation) = match movement {
            Movement::Destination(destination) => {
                let direction_vector = *destination - transform.translation;
                let heading = direction_vector.normalize();
                let distance = direction_vector.dot(direction_vector).sqrt();
                let end_translation = distance <= DESTINATION_MARGIN_OF_ERROR;

                if end_translation {
                    transform.translation = *destination;
                } else {
                    transform.translation +=
                        heading * linear_speed.0 * time.delta_seconds();
                }

                (heading, end_translation)
            },
            Movement::Heading(heading) => (**heading, true),
        };

        // Negate forward() because glTF models typically face +Z axis.
        let forward = -*transform.forward();
        let end_rotation =
            forward.dot(heading).abs() >= 1.0 - HEADING_MARGIN_OF_ERROR;

        if !end_rotation {
            transform.rotation = (transform.rotation
                * Quat::from_axis_angle(
                    forward.cross(heading).normalize(),
                    angular_speed.0 * time.delta_seconds(),
                ))
            .normalize();
        }

        if end_translation && end_rotation {
            entity_commands.remove::<Movement>();
        }
    }
}

fn destination_cleanup(
    mut commands: Commands,
    mut animations: Animations,
    mut removed: RemovedComponents<Movement>,
    query: Query<&StoredAnimation>,
) {
    for entity in removed.read() {
        if let Ok(stored_animation) = query.get(entity) {
            animations
                .play_clip_handle(entity, stored_animation.0.clone_weak());
            commands.entity(entity).remove::<StoredAnimation>();
        }
    }
}
