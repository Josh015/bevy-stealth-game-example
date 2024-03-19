use bevy::{ecs::prelude::*, math::prelude::*};

use crate::AngularVelocity;

/// Items that player can pick up by colliding with them.
#[derive(Clone, Component, Debug, Default)]
pub struct Pickup;

/// Required components for a [`Pickup`] entity.
#[derive(Bundle, Clone, Debug)]
pub struct PickupBundle {
    pub pickup: Pickup,
    pub angular_velocity: AngularVelocity,
}

impl Default for PickupBundle {
    fn default() -> Self {
        Self {
            pickup: Pickup,
            angular_velocity: AngularVelocity {
                axis: Direction3d::Y,
                velocity: 90f32.to_radians(),
            },
        }
    }
}
