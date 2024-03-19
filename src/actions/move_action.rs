use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::{MoveTo, Mover};

/// Move the entity in a straight line to a given point.
///
/// **WARNING**: Can't be run in parallel with
/// [`TurnToAction`](crate::actions::TurnToAction).
#[derive(new)]
pub struct MoveAction {
    move_to: MoveTo,
}

impl Action for MoveAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<Mover>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world.entity_mut(agent).insert(Mover::new(self.move_to));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Mover>();
    }
}
