use bevy::{ecs::prelude::*, math::primitives::Direction3d, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

use crate::{
    common::constants::FORWARD_DIRECTION,
    components::movement::turning::Turning,
};

/// Rotate an entity to face a given direction.
///
/// **WARNING**: Can't be run in parallel with [`MoveToAction`](super::move_to_action::MoveToAction).
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
