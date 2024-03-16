use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;

use crate::{
    common::constants::FORWARD_DIRECTION,
    components::movement::{moving::Moving, rotating::Rotating},
};

// Move the entity in a straight line to a given point while playing a
// corresponding animation.
///
/// WARNING: Can't be used in parallel with [`MoveToActionAction`](super::turn_to_face_direction_action::MoveToActionAction).
#[derive(new)]
pub struct MoveToAction {
    new_position: Vec3,
}

impl Action for MoveToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        let entity = world.entity(agent);

        entity.get::<Moving>().is_none() && entity.get::<Rotating>().is_none()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity = world.entity_mut(agent);
        let Some(transform) = entity.get::<Transform>() else {
            return true;
        };
        let new_direction =
            (self.new_position - transform.translation).normalize();

        entity.insert((
            Moving::new(transform.translation, self.new_position),
            Rotating::new(
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
        world.entity_mut(agent).remove::<Moving>();
        world.entity_mut(agent).remove::<Rotating>();
    }
}
