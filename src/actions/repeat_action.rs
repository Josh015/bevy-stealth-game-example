use crate::common::Repeat;
use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Takes a single action and repeats it.
///
/// **WARNING**: Doesn't work with
/// [`ActionSequence`](crate::actions::ActionSequence). Use
/// [`RepeatSequence`](crate::actions::RepeatSequence) instead.
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
        agent: Entity,
        world: &mut World,
        reason: StopReason,
    ) {
        self.action.on_stop(agent, world, reason);
    }

    fn on_drop(
        mut self: Box<Self>,
        agent: Entity,
        world: &mut World,
        reason: DropReason,
    ) {
        if self.repeat.is_finished() || reason != DropReason::Done {
            self.action.on_remove(agent, world);
            return;
        }

        self.repeat.advance();
        world.get_mut::<ActionQueue>(agent).unwrap().push_back(self);
    }
}
