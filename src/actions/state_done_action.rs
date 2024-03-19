use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;
use seldom_state::trigger::Done;

/// Triggers the `done()` condition for seldom_state `StateMachine` components.
#[derive(new)]
pub struct StateDoneAction {
    done: Done,
}

impl Action for StateDoneAction {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool {
        true
    }

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
