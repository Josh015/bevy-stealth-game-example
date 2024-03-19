use bevy::{app::prelude::*, ecs::prelude::*, math::prelude::*};

use crate::AngularVelocity;

pub(super) struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_pickup);
    }
}

/// Items that player can pick up by colliding with them.
#[derive(Clone, Component, Debug, Default)]
pub struct Pickup;

fn animate_pickup(mut commands: Commands, query: Query<Entity, Added<Pickup>>) {
    for entity in &query {
        commands.entity(entity).insert(AngularVelocity {
            axis: Direction3d::Y,
            velocity: 90f32.to_radians(),
        });
    }
}
