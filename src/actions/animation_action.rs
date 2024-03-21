use bevy::ecs::{prelude::*, system::SystemState};
use bevy_sequential_actions::*;
use derive_new::new;

use crate::components::Animator;

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
        let mut system_state: SystemState<Animator> = SystemState::new(world);
        let mut animator = system_state.get_mut(world);

        animator.play_animation_for_entity(agent, &self.animation_name);
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
