use bevy::prelude::*;
use derive_new::new;

mod angular_velocity;
mod velocity;

pub use angular_velocity::*;
pub use velocity::*;

pub const FORWARD_DIRECTION: Vec3 = Vec3::NEG_Z;
pub const MOTION_MARGIN_OF_ERROR: f32 = 0.01;

pub(super) struct MotionPlugin;

impl Plugin for MotionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AngularVelocityPlugin, VelocityPlugin))
            .add_systems(
                Update,
                (start_motion, motion, clean_up_motion).chain(),
            );
    }
}

/// Directional speed in `meters/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct Speed(pub f32);

/// Angular speed in `radians/second`.
#[derive(Clone, Component, Debug, Default)]
pub struct AngularSpeed(pub f32);

/// Required components for movement.
#[derive(Bundle, Clone, Debug, Default)]
pub struct MotionBundle {
    pub speed: Speed,
    pub angular_speed: AngularSpeed,
}

/// Means of imparting motion.
#[derive(Clone, Debug)]
pub enum Motivation {
    Destination(Vec3),
    Heading(Direction3d),
}

#[derive(Clone, Component, Debug, new)]
pub struct Motion {
    motivation: Motivation,

    #[new(default)]
    heading: Vec3,
}

fn start_motion(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Speed, &AngularSpeed, &mut Motion, &Transform),
        Added<Motion>,
    >,
) {
    for (entity, speed, angular_speed, mut motion, transform) in &mut query {
        motion.heading = match motion.motivation {
            Motivation::Destination(destination) => {
                let heading = (destination - transform.translation).normalize();

                commands.entity(entity).insert(Velocity(heading * speed.0));
                heading
            },
            Motivation::Heading(heading) => *heading,
        };

        commands.entity(entity).insert(AngularVelocity {
            axis: Direction3d::new_unchecked(
                (*transform.forward()).cross(motion.heading).normalize(),
            ),
            velocity: angular_speed.0,
        });
    }
}

fn motion(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Motion,
        &mut Transform,
        Has<Velocity>,
        Has<AngularVelocity>,
    )>,
) {
    for (entity, motion, mut transform, has_velocity, has_angular_velocity) in
        &mut query
    {
        if let Motivation::Destination(destination) = motion.motivation {
            if has_velocity
                && destination.distance(transform.translation)
                    <= MOTION_MARGIN_OF_ERROR
            {
                commands.entity(entity).remove::<Velocity>();
                transform.translation = destination;
            }
        }

        if has_angular_velocity
            && transform.forward().dot(motion.heading).abs()
                >= 1.0 - MOTION_MARGIN_OF_ERROR
        {
            commands.entity(entity).remove::<AngularVelocity>();
            transform.rotation =
                Quat::from_rotation_arc(FORWARD_DIRECTION, motion.heading);
        }

        if !has_velocity && !has_angular_velocity {
            commands.entity(entity).remove::<Motion>();
            return;
        }
    }
}

fn clean_up_motion(
    mut commands: Commands,
    mut removed: RemovedComponents<Motion>,
) {
    for entity in removed.read() {
        commands
            .entity(entity)
            .remove::<(Velocity, AngularVelocity)>();
    }
}
