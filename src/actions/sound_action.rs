use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Plays a sound from this entity.
#[derive(new)]
pub struct SoundAction;

impl Action for SoundAction {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool {
        true
    }

    fn on_start(&mut self, _agent: Entity, _world: &mut World) -> bool {
        // TODO: Implement this functionality later.
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
