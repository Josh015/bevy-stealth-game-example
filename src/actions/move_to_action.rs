use bevy::app::prelude::*;
use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

use crate::common::MOVEMENT_TOLERANCE;
use crate::{MovingSpeed, Translating};

use super::TurnTo;

pub(super) struct MoveToActionPlugin;

impl Plugin for MoveToActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_move_to, move_to, clean_up_move_to).chain(),
        );
    }
}

/// Move the entity in a straight line to a given point.
///
/// **WARNING**: Can't be run in parallel with
/// [`TurnToAction`](crate::actions::TurnToAction).
#[derive(new)]
pub struct MoveToAction {
    position: Vec3,
}

impl Action for MoveToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<MoveTo>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert((MoveTo::new(self.position),));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<MoveTo>();
    }
}

/// Moves a [`MovingSpeed`] entity to a new position before removing itself.
#[derive(Clone, Component, Debug, new)]
pub(super) struct MoveTo {
    position: Vec3,

    #[new(default)]
    is_finished: bool,
}

fn start_move_to(
    mut commands: Commands,
    query: Query<(Entity, &MovingSpeed, &MoveTo, &Transform), Added<MoveTo>>,
) {
    for (entity, moving_speed, move_to, transform) in &query {
        let moving_direction =
            (move_to.position - transform.translation).normalize();

        commands.entity(entity).insert((
            Translating {
                translation: moving_direction * moving_speed.0,
            },
            TurnTo::new(Direction3d::new_unchecked(moving_direction)),
        ));
    }
}

fn move_to(
    mut commands: Commands,
    mut query: Query<(Entity, &mut MoveTo, &mut Transform)>,
) {
    for (entity, mut move_to, mut transform) in &mut query {
        // Delay removal by one update to prevent visual snapping to final position.
        if move_to.is_finished {
            commands.entity(entity).remove::<MoveTo>();
            transform.translation = move_to.position;
        } else {
            move_to.is_finished =
                move_to.position.distance(transform.translation)
                    <= MOVEMENT_TOLERANCE;
        }
    }
}

fn clean_up_move_to(
    mut commands: Commands,
    mut removed: RemovedComponents<MoveTo>,
    query: Query<Entity, Or<(With<Translating>, With<TurnTo>)>>,
) {
    // Clean up associated components if this one is removed early.
    for entity in removed.read() {
        if query.contains(entity) {
            commands.entity(entity).remove::<(Translating, TurnTo)>();
        }
    }
}
