use crate::{
    common::constants::FORWARD_DIRECTION,
    components::movement::turning::Turning,
};
use bevy::{ecs::prelude::*, math::primitives::Direction3d, prelude::*};
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
        !world.entity(agent).contains::<Turning>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity = world.entity_mut(agent);
        let Some(transform) = entity.get::<Transform>() else {
            return true;
        };
        entity.insert(Turning::new(
            transform.rotation,
            Quat::from_rotation_arc(FORWARD_DIRECTION, *self.direction),
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
