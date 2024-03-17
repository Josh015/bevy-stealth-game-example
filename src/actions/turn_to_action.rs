use crate::components::movement::turning_to::TurningTo;
use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

/// Turn an entity to face a specified direction.
///
/// **WARNING**: Can't be run in parallel with
/// [`MoveToAction`](super::move_to_action::MoveToAction).
#[derive(new)]
pub struct TurnToAction {
    direction: Direction3d,
}

impl Action for TurnToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<TurningTo>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(TurningTo::new(self.direction));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<TurningTo>();
    }
}
