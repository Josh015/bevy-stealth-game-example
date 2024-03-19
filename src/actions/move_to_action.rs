use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

use crate::{Motion, Motivation};

/// Move the entity in a straight line to a given point.
///
/// **WARNING**: Can't be run in parallel with
/// [`TurnToAction`](crate::actions::TurnToAction).
#[derive(new)]
pub struct MoveToAction {
    destination: Vec3,
}

impl Action for MoveToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<Motion>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(Motion::new(Motivation::Destination(self.destination)));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Motion>();
    }
}
