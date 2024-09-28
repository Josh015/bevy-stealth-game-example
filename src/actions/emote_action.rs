use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Pops up a temporary emote image sprite that hovers over the head of the
/// entity that spawned it.
#[derive(new)]
pub struct EmoteAction {
    #[new(into)]
    texture_name: String,
    blocking: bool,
    // image_handle: Handle<Image>
}

impl Action for EmoteAction {
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
