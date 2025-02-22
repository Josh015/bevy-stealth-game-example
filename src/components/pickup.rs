use bevy::{ecs::prelude::*, math::prelude::*, prelude::*};

pub(super) struct PickupPlugin;

impl Plugin for PickupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spinning_rise_and_fall_effect);
    }
}

/// Items that player can pick up by colliding with them.
#[derive(Clone, Component, Debug, Default)]
#[require(Transform)]
pub struct Pickup;

fn spinning_rise_and_fall_effect(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Pickup>>,
) {
    for mut transform in &mut query {
        // Rotate in place at a fixed speed.
        transform.rotation = (transform.rotation
            * Quat::from_axis_angle(
                Vec3::Y,
                std::f32::consts::FRAC_PI_2 * time.delta_secs(),
            ))
        .normalize();

        // TODO: Need to optimize this by storing Rotation as a component!
        // Up and Down hover effect (assumes Up=+Y).
        let rotation_angle = transform.rotation.to_euler(EulerRot::YZX).0;
        transform.translation.y = (rotation_angle).sin() * 0.1 + 0.2
    }
}
