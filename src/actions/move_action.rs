use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::{MoveTo, Mover};

/// Move an entity.
///
/// **NOTE:** Requires [`Mover`] component.
#[derive(new)]
pub struct MoveAction {
    move_to: MoveTo,
}

impl Action for MoveAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        let Some(mover) = world.entity(agent).get::<Mover>() else {
            return true;
        };

        mover.is_finished()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let Some(mut mover) = world.get_mut::<Mover>(agent) else {
            return true;
        };

        mover.start(self.move_to);
        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        let Some(mut mover) = world.get_mut::<Mover>(agent) else {
            return;
        };

        mover.stop();
    }
}
