use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;
use seldom_state::trigger::Done;

/// Integrates with `seldom_state` to trigger the `done()` condition.
#[derive(new)]
pub struct StateDone {
    done: Done,
}

impl Action for StateDone {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool { true }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world.entity_mut(agent).insert(self.done);

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
