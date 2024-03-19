use crate::{MoveTo, Mover};
use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

/// Turn an entity to face a specified direction.
///
/// **WARNING**: Can't be run in parallel with
/// [`MoveToAction`](crate::actions::MoveToAction).
#[derive(new)]
pub struct FaceDirectionAction {
    direction: Direction3d,
}

impl Action for FaceDirectionAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<Mover>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(Mover::new(MoveTo::Direction(self.direction)));

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
