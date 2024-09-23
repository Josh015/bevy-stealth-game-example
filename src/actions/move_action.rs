use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::prelude::*;

/// Moves an entity.
#[derive(new)]
pub struct MoveAction {
    move_to: MoveTo,
}

impl Action for MoveAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<MoveTo>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world.entity_mut(agent).insert(self.move_to.clone());
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
