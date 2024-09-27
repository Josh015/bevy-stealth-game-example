use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::prelude::*;

/// Rotate to face a given direction.
#[derive(new)]
pub struct FaceDirectionAction {
    direction: Dir3,
}

impl Action for FaceDirectionAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        !world.entity(agent).contains::<Heading>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(Heading::from_vector(self.direction.as_vec3()));
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
