use crate::{
    common::{FORWARD_DIRECTION, MOVEMENT_TOLERANCE},
    AngularSpeed, AngularVelocity,
};
use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

pub(super) struct FaceDirectionPlugin;

impl Plugin for FaceDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_face_direction,
                face_direction,
                clean_up_face_direction,
            )
                .chain(),
        );
    }
}

/// Turn an entity to face a specified direction.
///
/// **WARNING**: Can't be run in parallel with
/// [`MoveToAction`](crate::actions::MoveToAction).
#[derive(new)]
pub struct FaceDirectionAction {
    direction: Direction3d,
}

impl Action for FaceDirectionAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<FaceDirection>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(FaceDirection::new(self.direction));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<FaceDirection>();
    }
}

/// Rotates a [`TurningSpeed`] entity to a new rotation before removing itself.
#[derive(Clone, Component, Debug, new)]
pub(super) struct FaceDirection {
    direction: Direction3d,

    #[new(default)]
    is_finished: bool,
}

fn start_face_direction(
    mut commands: Commands,
    query: Query<
        (Entity, &AngularSpeed, &FaceDirection, &Transform),
        Added<FaceDirection>,
    >,
) {
    for (entity, angular_speed, face_direction, transform) in &query {
        commands.entity(entity).insert(AngularVelocity {
            axis: Direction3d::new_unchecked(
                (*transform.forward())
                    .cross(*face_direction.direction)
                    .normalize(),
            ),
            velocity: angular_speed.0,
        });
    }
}

fn face_direction(
    mut commands: Commands,
    mut query: Query<(Entity, &mut FaceDirection, &mut Transform)>,
) {
    for (entity, mut face_direction, mut transform) in &mut query {
        // Delay removal by one update to prevent visual snapping to final rotation.
        if face_direction.is_finished {
            commands.entity(entity).remove::<FaceDirection>();
            transform.rotation = Quat::from_rotation_arc(
                FORWARD_DIRECTION,
                *face_direction.direction,
            );
        } else {
            face_direction.is_finished =
                (*transform.forward()).dot(*face_direction.direction).abs()
                    >= 1.0 - MOVEMENT_TOLERANCE;
        }
    }
}

fn clean_up_face_direction(
    mut commands: Commands,
    mut removed: RemovedComponents<FaceDirection>,
    query: Query<Entity, With<AngularVelocity>>,
) {
    // Clean up associated components if this one is removed early.
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).remove::<AngularVelocity>();
        }
    }
}
