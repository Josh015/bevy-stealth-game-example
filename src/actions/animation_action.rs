use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::CurrentAnimation;

/// Plays an animation.
#[derive(new)]
pub struct AnimationAction {
    animation_name: String,
}

impl Action for AnimationAction {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool {
        true
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(CurrentAnimation(self.animation_name.to_owned()));
        true
    }

    fn on_stop(
        &mut self,
        _agent: Entity,
        _world: &mut World,
        _reason: StopReason,
    ) {
    }
}
