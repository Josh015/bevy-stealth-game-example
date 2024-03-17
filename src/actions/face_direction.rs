use crate::components::movement::turning_to::TurningTo;
use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

/// Rotate an entity to face a given direction.
///
/// **WARNING**: Can't be run in parallel with
/// [`MoveTo`](super::move_to::MoveTo).
#[derive(new)]
pub struct FaceDirection {
    direction: Direction3d,
}

impl Action for FaceDirection {
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
