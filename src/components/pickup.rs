use bevy::{ecs::prelude::*, math::prelude::*, prelude::*};

pub(super) struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spinning_pickup_item);
    }
}

/// Required components for a [`Pickup`] entity.
#[derive(Bundle, Clone, Debug)]
pub struct PickupBundle {
    pub pickup: Pickup,
}

impl Default for PickupBundle {
    fn default() -> Self {
        // TODO: Need states to represent it being visible, disappearing, disappeared, and
        // reappearing after a set delay.

        Self { pickup: Pickup }
    }
}

/// Items that player can pick up by colliding with them.
#[derive(Clone, Component, Debug, Default)]
pub struct Pickup;

fn spinning_pickup_item(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Pickup>>,
) {
    for mut transform in &mut query {
        transform.rotation = (transform.rotation
            * Quat::from_axis_angle(
                Vec3::Y,
                std::f32::consts::FRAC_PI_2 * time.delta_seconds(),
            ))
        .normalize();
    }
}
