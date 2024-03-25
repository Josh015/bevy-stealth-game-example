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

        !mover.is_moving()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity_commands = world.entity_mut(agent);
        let Some(mut mover) = entity_commands.get_mut::<Mover>() else {
            return true;
        };

        mover.set_move_to(self.move_to.clone());
        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        let mut entity_commands = world.entity_mut(agent);
        let Some(mut mover) = entity_commands.get_mut::<Mover>() else {
            return;
        };
        mover.cancel_move_to();
    }
}
