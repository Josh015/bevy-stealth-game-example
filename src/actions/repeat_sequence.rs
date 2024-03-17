use crate::common::repeat::Repeat;
use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Runs a collection of actions in order and repeats them.
///
/// **WARNING**: Doesn't work with
/// [`RepeatAction`](super::repeat_action::RepeatAction).
#[derive(new)]
pub struct RepeatSequence<const N: usize> {
    repeat: Repeat,
    actions: [BoxedAction; N],

    #[new(default)]
    index: usize,
}

impl<const N: usize> Action for RepeatSequence<N> {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        self.actions[self.index].is_finished(agent, world)
    }

    fn on_add(&mut self, agent: Entity, world: &mut World) {
        self.actions
            .iter_mut()
            .for_each(|action| action.on_add(agent, world));
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        self.actions[self.index].on_start(agent, world)
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        reason: StopReason,
    ) {
        self.actions[self.index].on_stop(agent, world, reason);

        if reason == StopReason::Canceled {
            self.index = self.actions.len();
        }
    }

    fn on_drop(
        mut self: Box<Self>,
        agent: Entity,
        world: &mut World,
        reason: DropReason,
    ) {
        self.index += 1;

        if self.index >= self.actions.len() {
            self.repeat.advance();
            self.index = 0;
        }

        if self.repeat.is_finished() || reason != DropReason::Done {
            self.actions
                .iter_mut()
                .for_each(|action| action.on_remove(agent, world));
        } else {
            world
                .get_mut::<ActionQueue>(agent)
                .unwrap()
                .push_front(self);
        }
    }
}
