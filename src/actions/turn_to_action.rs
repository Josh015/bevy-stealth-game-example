use crate::{
    common::{FORWARD_DIRECTION, MOVEMENT_TOLERANCE},
    AngularSpeed, AngularVelocity,
};
use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

pub(super) struct TurnToPlugin;

impl Plugin for TurnToPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_turn_to, turn_to, clean_up_turn_to).chain(),
        );
    }
}

/// Turn an entity to face a specified direction.
///
/// **WARNING**: Can't be run in parallel with
/// [`MoveToAction`](crate::actions::MoveToAction).
#[derive(new)]
pub struct TurnToAction {
    direction: Direction3d,
}

impl Action for TurnToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<TurnTo>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world.entity_mut(agent).insert(TurnTo::new(self.direction));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<TurnTo>();
    }
}

/// Rotates a [`TurningSpeed`] entity to a new rotation before removing itself.
#[derive(Clone, Component, Debug, new)]
pub(super) struct TurnTo {
    direction: Direction3d,

    #[new(default)]
    is_finished: bool,
}

fn start_turn_to(
    mut commands: Commands,
    query: Query<(Entity, &AngularSpeed, &TurnTo, &Transform), Added<TurnTo>>,
) {
    for (entity, angular_speed, turn_to, transform) in &query {
        commands.entity(entity).insert(AngularVelocity {
            axis: Direction3d::new_unchecked(
                (*transform.forward()).cross(*turn_to.direction).normalize(),
            ),
            velocity: angular_speed.0,
        });
    }
}

fn turn_to(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TurnTo, &mut Transform)>,
) {
    for (entity, mut turn_to, mut transform) in &mut query {
        // Delay removal by one update to prevent visual snapping to final rotation.
        if turn_to.is_finished {
            commands.entity(entity).remove::<TurnTo>();
            transform.rotation =
                Quat::from_rotation_arc(FORWARD_DIRECTION, *turn_to.direction);
        } else {
            turn_to.is_finished =
                (*transform.forward()).dot(*turn_to.direction).abs()
                    >= 1.0 - MOVEMENT_TOLERANCE;
        }
    }
}

fn clean_up_turn_to(
    mut commands: Commands,
    mut removed: RemovedComponents<TurnTo>,
    query: Query<Entity, With<AngularVelocity>>,
) {
    // Clean up associated components if this one is removed early.
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).remove::<AngularVelocity>();
        }
    }
}
