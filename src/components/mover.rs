use bevy::prelude::*;

pub const FORWARD_DIRECTION: Vec3 = Vec3::NEG_Z;
pub const LINEAR_VELOCITY_MARGIN_OF_ERROR: f32 = 0.001;
pub const ANGULAR_VELOCITY_MARGIN_OF_ERROR: f32 = 0.0001;

pub(super) struct MoverPlugin;

impl Plugin for MoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (linear_velocity, angular_velocity, mover).chain(),
        );
    }
}

/// Required components for a [`Mover`] entity.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MoverBundle {
    pub mover: Mover,
    pub linear_speed: LinearSpeed,
    pub angular_speed: AngularSpeed,
}

/// Provides precise on-demand movement for a [`Transform`] entity.
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
        self.move_to.is_none() && self.heading.is_none()
    }
}

/// Specifies the desired movement type.
#[derive(Clone, Copy, Debug)]
pub enum MoveTo {
    Destination(Vec3),
    Direction(Direction3d),
}

/// Linear speed in `meters/second`.
#[derive(Clone, Component, Debug)]
pub struct LinearSpeed(pub f32);

impl Default for LinearSpeed {
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

/// Linear velocity that updates translation over time.
#[derive(Clone, Component, Debug)]
pub struct LinearVelocity(pub Vec3);

/// Angular velocity that updates rotation over time.
#[derive(Clone, Component, Debug)]
pub struct AngularVelocity {
    pub axis: Direction3d,
    pub velocity: f32,
}

fn linear_velocity(
    time: Res<Time>,
    mut query: Query<(&LinearVelocity, &mut Transform)>,
) {
    for (linear_velocity, mut transform) in &mut query {
        transform.translation += linear_velocity.0 * time.delta_seconds();
    }
}

fn angular_velocity(
    time: Res<Time>,
    mut query: Query<(&AngularVelocity, &mut Transform)>,
) {
    for (angular_velocity, mut transform) in &mut query {
        transform.rotation = (transform.rotation
            * Quat::from_axis_angle(
                *angular_velocity.axis,
                angular_velocity.velocity * time.delta_seconds(),
            ))
        .normalize();
    }
}

fn mover(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &mut Mover,
        &LinearSpeed,
        &AngularSpeed,
        Has<LinearVelocity>,
        Has<AngularVelocity>,
    )>,
) {
    for (
        entity,
        mut transform,
        mut mover,
        linear_speed,
        angular_speed,
        has_linear_velocity,
        has_angular_velocity,
    ) in &mut query
    {
        match (mover.move_to, mover.heading) {
            // Initialize and cache data before inserting working components.
            (Some(move_to), None) => {
                let mut entity = commands.entity(entity);
                let heading = match move_to {
                    MoveTo::Destination(destination) => {
                        let heading =
                            (destination - transform.translation).normalize();

                        entity.insert(LinearVelocity(heading * linear_speed.0));
                        heading
                    },
                    MoveTo::Direction(heading) => {
                        // Cleanup for when move_to mode is switched midway.
                        if has_linear_velocity {
                            entity.remove::<LinearVelocity>();
                        }

                        *heading
                    },
                };

                mover.heading = Some(heading);
                entity.insert(AngularVelocity {
                    axis: Direction3d::new_unchecked(
                        // Negate forward() because glTF models face POS_Z!
                        -(*transform.forward()).cross(heading).normalize(),
                    ),
                    velocity: angular_speed.0,
                });
            },

            // Check progress and eventually remove the working components.
            (Some(move_to), Some(heading)) => {
                if !has_linear_velocity && !has_angular_velocity {
                    mover.move_to = None;
                    mover.heading = None;
                } else {
                    let mut entity = commands.entity(entity);

                    if has_linear_velocity {
                        if let MoveTo::Destination(destination) = move_to {
                            if destination.distance(transform.translation)
                                <= LINEAR_VELOCITY_MARGIN_OF_ERROR
                            {
                                entity.remove::<LinearVelocity>();
                                transform.translation = destination;
                            }
                        }
                    }

                    // Negate forward() because glTF models face POS_Z!
                    if has_angular_velocity
                        && (-transform.forward()).dot(heading).abs()
                            >= 1.0 - ANGULAR_VELOCITY_MARGIN_OF_ERROR
                    {
                        entity.remove::<AngularVelocity>();
                    }
                }
            },

            // Reset to default state if mover was stopped externally.
            (None, Some(_)) => {
                mover.heading = None;

                if has_linear_velocity || has_angular_velocity {
                    commands
                        .entity(entity)
                        .remove::<(AngularVelocity, LinearVelocity)>();
                }
            },
            _ => {},
        }
    }
}
