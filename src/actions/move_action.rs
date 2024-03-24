use crate::components::*;
use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Moves an entity.
#[derive(new)]
pub struct MoveAction {
    move_to: MoveTo,
}

impl Action for MoveAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        let entity_commands = world.entity(agent);
        let Some(mover) = entity_commands.get::<Mover>() else {
            return true;
        };
        mover.move_to.is_none()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity_commands = world.entity_mut(agent);
        let Some(mut mover) = entity_commands.get_mut::<Mover>() else {
            return true;
        };

        mover.move_to = Some(self.move_to.clone());
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
