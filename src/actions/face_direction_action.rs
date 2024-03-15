use std::time::Duration;

use bevy::{ecs::prelude::*, math::primitives::Direction3d, prelude::*};
use bevy_sequential_actions::*;
use bevy_tweening::{lens::TransformRotationLens, Animator, *};
use derive_new::new;

use crate::common::constants::FORWARD_DIRECTION;

/// Rotates an entity to face a given direction and plays a corresponding turning animation for left/right.
///
/// WARNING: Can't be used in parallel with [`MoveToAction`](super::move_to_action::MoveToAction).
#[derive(new)]
pub struct FaceDirectionAction {
    new_direction: Direction3d,
}

impl Action for FaceDirectionAction {
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
        let tween_rotate = Tween::new(
            EaseFunction::QuadraticInOut,
            Duration::from_millis(700),
            TransformRotationLens {
                start: transform.rotation,
                end: Quat::from_rotation_arc(
                    FORWARD_DIRECTION,
                    *self.new_direction,
                ),
            },
        );
        entity.insert(Animator::new(tween_rotate));

        // TODO: Play skeletal turning animation.
        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<Animator<Transform>>();
        // TODO: Pause skeletal turning animation.
    }
}
