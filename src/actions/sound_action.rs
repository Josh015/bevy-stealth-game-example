use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Plays a sound from this entity.
#[derive(new)]
pub struct SoundAction {
    #[new(into)]
    sound_name: String,

    #[new(value = "true")]
    blocking: bool,
}

impl SoundAction {
    pub fn non_blocking(sound_name: impl Into<String>) -> Self {
        Self {
            sound_name: sound_name.into(),
            blocking: false,
        }
    }
}

impl Action for SoundAction {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool {
        true
    }

    fn on_start(&mut self, _agent: Entity, _world: &mut World) -> bool {
        // TODO: Implement this functionality later.
        !self.blocking
    }

    fn on_stop(
        &mut self,
        _agent: Entity,
        _world: &mut World,
        _reason: StopReason,
    ) {
    }
}
