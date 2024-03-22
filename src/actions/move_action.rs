use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::Movement;

/// Move an entity to a point.
///
/// **WARNING:** Can't be used in parallel with [`TurnToAction`](super::turn_to_action::TurnToAction).
#[derive(new)]
pub struct MoveAction {
    movement: Movement,
}

impl Action for MoveAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<Movement>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world.entity_mut(agent).insert(self.movement.clone());
        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Movement>();
    }
}
