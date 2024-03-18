use bevy::{ecs::prelude::*, prelude::*};

pub(super) struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, velocity);
    }
}

/// Translates an entity over time.
#[derive(Clone, Component, Debug)]
pub struct Velocity(pub Vec3);

fn velocity(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}
