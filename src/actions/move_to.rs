use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;

use crate::{
    common::constants::FORWARD_DIRECTION,
    components::movement::{moving::Moving, turning::Turning},
};

/// Move the entity in a straight line to a given point.
///
/// **WARNING**: Can't be run in parallel with [`FaceDirectionAction`](super::face_direction_action::FaceDirectionAction).
pub struct MoveTo(pub Vec3);

impl Action for MoveTo {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        let entity = world.entity(agent);

        !entity.contains::<Moving>() && !entity.contains::<Turning>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity = world.entity_mut(agent);
        let Some(transform) = entity.get::<Transform>() else {
            return true;
        };
        let new_direction = (self.0 - transform.translation).normalize();

        entity.insert((
            Moving::new(transform.translation, self.0),
            Turning::new(
                transform.rotation,
                Quat::from_rotation_arc(FORWARD_DIRECTION, new_direction),
            ),
        ));

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<(Moving, Turning)>();
    }
}
