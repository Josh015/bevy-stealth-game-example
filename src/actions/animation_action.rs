use bevy::ecs::{prelude::*, system::SystemState};
use bevy_sequential_actions::*;
use derive_new::new;

use crate::prelude::*;

/// Plays an animation.
///
/// **WARNING**: Malfunctions when used in [`ParallelActions`] with [`MoveToAction`] or [`FaceDirectionAction`].
#[derive(new)]
pub struct AnimationAction {
    #[new(into)]
    clip_name: String,

    #[new(value = "true")]
    blocking: bool,
}

impl AnimationAction {
    pub fn non_blocking(clip_name: impl Into<String>) -> Self {
        Self {
            clip_name: clip_name.into(),
            blocking: false,
        }
    }
}

impl Action for AnimationAction {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool {
        true
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut system_state: SystemState<Animations> = SystemState::new(world);
        let mut animations = system_state.get_mut(world);

        if !self.blocking {
            animations.play_clip_name(agent, &self.clip_name);
            true
        } else {
            // TODO: Implement logic to play once and detect completion.
            false
        }
    }

    fn on_stop(
        &mut self,
        _agent: Entity,
        _world: &mut World,
        _reason: StopReason,
    ) {
    }
}
