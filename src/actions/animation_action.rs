use crate::system_params::*;
use bevy::ecs::{prelude::*, system::SystemState};
use bevy_sequential_actions::*;
use derive_new::new;

/// Plays an animation.
#[derive(new)]
pub struct AnimationAction {
    clip_name: String,
}

impl Action for AnimationAction {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool {
        true
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut system_state: SystemState<Animations> = SystemState::new(world);
        let mut animations = system_state.get_mut(world);

        animations.play_clip(agent, &self.clip_name);
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
