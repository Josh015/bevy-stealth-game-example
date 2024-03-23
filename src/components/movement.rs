use bevy::prelude::*;

use crate::game::{Animations, StoppedWhenPausedSet};

const DESTINATION_MARGIN_OF_ERROR: f32 = 0.01;
const HEADING_MARGIN_OF_ERROR: f32 = 0.001;
const MOVING_ANIMATION: &str = "moving";

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // The order is important for correct rotations, so don't mess with it!
        app.add_systems(
            Update,
            (move_to_setup, move_to_update, move_to_cleanup)
                .chain()
                .in_set(StoppedWhenPausedSet),
        );
    }
}

/// All stats relevant to movement.
#[derive(Clone, Component, Debug)]
pub struct Movement {
    /// Linear speed in `meters/second`.
    pub linear_speed: f32,

    /// Angular speed in `radians/second`.
    pub angular_speed: f32,
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            linear_speed: 1.0,
            angular_speed: std::f32::consts::TAU,
        }
    }
}

/// Makes an entity transform in a specified way.
#[derive(Clone, Component, Debug)]
pub enum MoveTo {
    /// A point this entity is trying to reach.
    Destination(Vec3),

    /// A direction this entity wants to face.
    Heading(Direction3d),
}

/// Stores currently running animation for later restoration.
#[derive(Clone, Component, Debug, Default)]
pub struct StoredAnimation(pub Handle<AnimationClip>);

fn move_to_setup(
    mut commands: Commands,
    mut animations: Animations,
    mut query: Query<Entity, Added<MoveTo>>,
) {
    for entity in &mut query {
        let mut entity_commands = commands.entity(entity);

        if let Some(current_animation) = animations.get_current_clip(entity) {
            entity_commands.insert(StoredAnimation(current_animation));
        }

        animations.play_clip(entity, MOVING_ANIMATION);
    }
}

fn move_to_update(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &MoveTo, &Movement)>,
) {
    for (entity, mut transform, move_to, movement) in &mut query {
        let mut entity_commands = commands.entity(entity);
        let (heading, end_translation) = match move_to {
            MoveTo::Destination(destination) => {
                let direction_vector = *destination - transform.translation;
                let heading = direction_vector.normalize();
                let distance = direction_vector.dot(direction_vector).sqrt();
                let end_translation = distance <= DESTINATION_MARGIN_OF_ERROR;

                if end_translation {
                    transform.translation = *destination;
                } else {
                    transform.translation +=
                        heading * movement.linear_speed * time.delta_seconds();
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
                    forward.cross(heading).normalize(),
                    movement.angular_speed * time.delta_seconds(),
                ))
            .normalize();
        }

        if end_translation && end_rotation {
            entity_commands.remove::<MoveTo>();
        }
    }
}

fn move_to_cleanup(
    mut commands: Commands,
    mut animations: Animations,
    mut removed: RemovedComponents<MoveTo>,
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
