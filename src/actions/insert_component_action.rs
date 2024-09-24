use bevy::ecs::prelude::*;
use bevy_sequential_actions::*;

/// Inserts an arbitrary component.
pub struct InsertComponentAction<T: Component> {
    component: Option<T>,
}

impl<T: Component> InsertComponentAction<T> {
    pub fn new(component: T) -> Self {
        Self {
            component: Some(component),
        }
    }
}

impl<T: Component> Action for InsertComponentAction<T> {
    fn is_finished(&self, _agent: Entity, _world: &World) -> bool {
        true
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        world
            .entity_mut(agent)
            .insert(self.component.take().unwrap());
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
