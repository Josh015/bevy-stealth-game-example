use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

/// Takes a collection of actions and runs them in parallel.
#[derive(new)]
pub struct ParallelActions<const N: usize> {
    actions: [BoxedAction; N],
}

impl<const N: usize> Action for ParallelActions<N> {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        self.actions
            .iter()
            .all(|action| action.is_finished(agent, world))
    }

    fn on_add(&mut self, agent: Entity, world: &mut World) {
        self.actions
            .iter_mut()
            .for_each(|action| action.on_add(agent, world));
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        std::array::from_fn::<bool, N, _>(|i| {
            self.actions[i].on_start(agent, world)
        })
        .into_iter()
        .all(|b| b)
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        reason: StopReason,
    ) {
        self.actions
            .iter_mut()
            .for_each(|action| action.on_stop(agent, world, reason));
    }

    fn on_remove(&mut self, agent: Entity, world: &mut World) {
        self.actions
            .iter_mut()
            .for_each(|action| action.on_remove(agent, world));
    }
}
