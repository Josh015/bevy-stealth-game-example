use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::util::Repeat;

/// Takes a single action and repeats it.
///
/// **WARNING**: Doesn't work with
/// [`ActionSequence`](super::ActionSequence). Use
/// [`RepeatSequence`](super::RepeatSequence) instead.
#[derive(new)]
pub struct RepeatAction<A: Action> {
    repeat: Repeat,
    action: A,
}

impl<A: Action> Action for RepeatAction<A> {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        self.action.is_finished(agent, world)
    }

    fn on_add(&mut self, agent: Entity, world: &mut World) {
        self.action.on_add(agent, world);
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        self.action.on_start(agent, world)
    }

    fn on_stop(
        &mut self,
        agent: Option<Entity>,
        world: &mut World,
        reason: StopReason,
    ) {
        self.action.on_stop(agent, world, reason);
    }

    fn on_drop(
        mut self: Box<Self>,
        agent: Option<Entity>,
        world: &mut World,
        reason: DropReason,
    ) {
        if self.repeat.is_finished() || reason != DropReason::Done {
            return;
        }

        let Some(agent) = agent else { return };

        self.repeat.advance();
        world.actions(agent).start(false).add(self as BoxedAction);
    }
}
