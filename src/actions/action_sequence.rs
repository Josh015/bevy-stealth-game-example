use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Runs a collection of actions in order.
///
/// **WARNING**: Doesn't work with
/// [`RepeatAction`](crate::actions::RepeatAction). Use
/// [`RepeatSequence`](crate::actions::RepeatSequence) instead.
#[derive(new)]
pub struct ActionSequence<const N: usize> {
    actions: [BoxedAction; N],

    #[new(default)]
    index: usize,
    #[new(default)]
    canceled: bool,
}

impl<const N: usize> Action for ActionSequence<N> {
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
        agent: Option<Entity>,
        world: &mut World,
        reason: StopReason,
    ) {
        self.actions[self.index].on_stop(agent, world, reason);
        self.canceled = reason == StopReason::Canceled;
    }

    fn on_remove(&mut self, agent: Option<Entity>, world: &mut World) {
        self.actions[self.index].on_remove(agent, world);
    }

    fn on_drop(
        mut self: Box<Self>,
        agent: Option<Entity>,
        world: &mut World,
        reason: DropReason,
    ) {
        self.index += 1;

        if self.index >= N {
            return;
        }

        if self.canceled || reason != DropReason::Done {
            for i in self.index..N {
                self.actions[i].on_remove(agent, world);
            }
            return;
        }

        let Some(agent) = agent else { return };

        world
            .actions(agent)
            .start(false)
            .order(AddOrder::Front)
            .add(self as BoxedAction);
    }
}
