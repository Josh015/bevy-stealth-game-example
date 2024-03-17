use crate::components::movement::{moving_to::MovingTo, turning_to::TurningTo};
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
        let entity = world.entity(agent);

        !entity.contains::<MovingTo>() && !entity.contains::<TurningTo>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity = world.entity_mut(agent);
        let Some(transform) = entity.get::<Transform>() else {
            return true;
        };
        let new_direction = (self.position - transform.translation).normalize();

        entity.insert((
            MovingTo::new(self.position),
            TurningTo::new(Direction3d::new_unchecked(new_direction)),
        ));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<(MovingTo, TurningTo)>();
    }
}
