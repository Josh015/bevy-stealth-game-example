use std::time::Duration;

use bevy::{ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use bevy_tweening::{
    lens::{TransformPositionLens, TransformRotationLens},
    Animator, *,
};
use derive_new::new;

use crate::common::constants::FORWARD_DIRECTION;

// Move the entity in a straight line to a given point while playing a
// corresponding animation.
///
/// WARNING: Can't be used in parallel with [`FaceDirectionAction`](super::turn_to_face_direction_action::FaceDirectionAction).
#[derive(new)]
pub struct MoveToAction {
    new_position: Vec3,
}

impl Action for MoveToAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        let Some(animator) = world.entity(agent).get::<Animator<Transform>>()
        else {
            return true;
        };
        animator.tweenable().progress() == 1.0
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity = world.entity_mut(agent);
        let Some(transform) = entity.get::<Transform>() else {
            return true;
        };
        let new_direction =
            (self.new_position - transform.translation).normalize();
        let tween_translate = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_secs(1),
            TransformPositionLens {
                start: transform.translation,
                end: self.new_position,
            },
        );
        let tween_rotate = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(700),
            TransformRotationLens {
                start: transform.rotation,
                end: Quat::from_rotation_arc(FORWARD_DIRECTION, new_direction),
            },
        );
        let tracks = Tracks::new([tween_rotate, tween_translate]);

        entity.insert(Animator::new(tracks));

        // TODO: Play skeletal walking animation.
        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Animator<Transform>>();
        // TODO: Pause skeletal walking animation.
    }
}
