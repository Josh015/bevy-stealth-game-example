use crate::components::movement::moving_to::MovingTo;
use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

/// Move the entity in a straight line to a given point.
///
/// **WARNING**: Can't be run in parallel with
/// [`TurnToAction`](super::turn_to_action::TurnToAction).
#[derive(new)]
pub struct MoveToAction {
    position: Vec3,
}

impl Action for MoveToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<MovingTo>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert((MovingTo::new(self.position),));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<MovingTo>();
    }
}
