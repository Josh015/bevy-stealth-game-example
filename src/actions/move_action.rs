use bevy::prelude::*;
use bevy_sequential_actions::*;
use derive_new::new;

use crate::{Destination, Heading};

/// Specifies the desired movement type.
#[derive(Clone, Copy, Debug)]
pub enum MoveTo {
    Destination(Vec3),
    Heading(Direction3d),
}

/// Move an entity.
#[derive(new)]
pub struct MoveAction {
    move_to: MoveTo,
}

impl Action for MoveAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        let entity = world.entity(agent);

        !entity.contains::<Destination>() && !entity.contains::<Heading>()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let mut entity = world.entity_mut(agent);

        match self.move_to {
            MoveTo::Destination(destination) => {
                entity.insert(Destination(destination));
            },
            MoveTo::Heading(direction) => {
                entity.insert(Heading(direction));
            },
        }

        false
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        _reason: StopReason,
    ) {
        world.entity_mut(agent).remove::<(Destination, Heading)>();
    }
}
