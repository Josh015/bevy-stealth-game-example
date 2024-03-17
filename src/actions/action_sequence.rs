use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

#[derive(new)]
struct ActionSequence<const N: usize> {
    actions: [BoxedAction; N],

    #[new(default)]
    index: usize,
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

        if self.index >= self.actions.len() || reason != DropReason::Done {
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
