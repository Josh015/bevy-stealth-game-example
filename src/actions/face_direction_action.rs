use bevy::{ecs::prelude::*, math::primitives::Direction3d, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

use crate::{
    common::constants::FORWARD_DIRECTION,
    components::movement::turning::Turning,
};

/// Rotates an entity to face a given direction and plays a corresponding turning animation for left/right.
#[derive(new)]
pub struct FaceDirectionAction {
    new_direction: Direction3d,
}

impl Action for FaceDirectionAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        world.entity(agent).get::<Turning>().is_none()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity = world.entity_mut(agent);
        let Some(transform) = entity.get::<Transform>() else {
            return true;
        };
        entity.insert(Turning::new(
            transform.rotation,
            Quat::from_rotation_arc(FORWARD_DIRECTION, *self.new_direction),
        ));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Turning>();
    }
}
