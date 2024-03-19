use bevy::prelude::*;

use crate::{AngularVelocity, Velocity};

pub const FORWARD_DIRECTION: Vec3 = Vec3::NEG_Z;
pub const MOTION_MARGIN_OF_ERROR: f32 = 0.01;

pub(super) struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mover);
    }
}

/// Specify what type of movement is required.
#[derive(Clone, Copy, Debug)]
pub enum MoveTo {
    Destination(Vec3),
    Direction(Direction3d),
}

/// Moves the entity and then removes itself.
#[derive(Clone, Component, Debug, Default)]
pub struct Mover {
    move_to: Option<MoveTo>,
    heading: Option<Vec3>,
}

impl Mover {
    pub fn start(&mut self, move_to: MoveTo) {
        self.move_to = Some(move_to);
        self.heading = None;
    }

    pub fn stop(&mut self) {
        self.move_to = None;
    }

    pub fn is_finished(&self) -> bool {
        self.move_to.is_none()
    }
}

/// Directional speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct Speed(pub f32);

impl Default for Speed {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug)]
pub struct AngularSpeed(pub f32);

impl Default for AngularSpeed {
    fn default() -> Self {
        Self(std::f32::consts::TAU)
    }
}

/// Required components for Mover to work.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MoverBundle {
    pub mover: Mover,
    pub speed: Speed,
    pub angular_speed: AngularSpeed,
}

fn mover(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Mover,
        &Speed,
        &AngularSpeed,
        &mut Transform,
        Has<Velocity>,
        Has<AngularVelocity>,
    )>,
) {
    for (
        entity,
        mut mover,
        speed,
        angular_speed,
        mut transform,
        has_velocity,
        has_angular_velocity,
    ) in &mut query
    {
        match (mover.move_to, mover.heading) {
            // Initialize and cache data before inserting working components.
            (Some(move_to), None) => {
                let heading = match move_to {
                    MoveTo::Destination(destination) => {
                        let heading =
                            (destination - transform.translation).normalize();

                        commands
                            .entity(entity)
                            .insert(Velocity(heading * speed.0));
                        heading
                    },
                    MoveTo::Direction(heading) => *heading,
                };

                mover.heading = Some(heading);
                commands.entity(entity).insert(AngularVelocity {
                    axis: Direction3d::new_unchecked(
                        (*transform.forward()).cross(heading).normalize(),
                    ),
                    velocity: angular_speed.0,
                });
            },

            // Check progress and eventually remove the working components.
            (Some(move_to), Some(heading)) => {
                if let MoveTo::Destination(destination) = move_to {
                    if has_velocity
                        && destination.distance(transform.translation)
                            <= MOTION_MARGIN_OF_ERROR
                    {
                        commands.entity(entity).remove::<Velocity>();
                        transform.translation = destination;
                    }
                }

                if has_angular_velocity
                    && transform.forward().dot(heading).abs()
                        >= 1.0 - MOTION_MARGIN_OF_ERROR
                {
                    commands.entity(entity).remove::<AngularVelocity>();
                }

                if !has_velocity && !has_angular_velocity {
                    mover.move_to = None;
                    mover.heading = None;
                }
            },

            // Reset to default state if move_to was cleared.
            (None, Some(_)) => {
                mover.heading = None;
                commands
                    .entity(entity)
                    .remove::<(AngularVelocity, Velocity)>();
            },
            _ => {},
        }
    }
}
