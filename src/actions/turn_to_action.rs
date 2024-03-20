use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::Heading;

/// Turns an entity to face a direction.
///
/// **WARNING:** Can't be used in parallel with [`MoveToAction`](super::move_to_action::MoveToAction).
#[derive(new)]
pub struct TurnToAction {
    direction: Direction3d,
}

impl Action for TurnToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<Heading>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world.entity_mut(agent).insert(Heading(self.direction));
        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Heading>();
    }
}
