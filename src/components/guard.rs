use bevy::{app::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;
use std::time::Duration;

use crate::{
    actions::{
        MoveAction, MoveTo, RepeatSequence, StateDoneAction, WaitAction,
    },
    util::Repeat,
};

pub(super) struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ping, pong));
    }
}

/// Required components for a [`Guard`] entity.
#[derive(Bundle)]
pub struct GuardBundle {
    pub guard: Guard,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub ping: Ping,
}

impl Default for GuardBundle {
    fn default() -> Self {
        // ChasePlayer
        // SearchForPlayer
        // InvestigateSound
        // Patrol

        // Guard location
        // Patrol
        // Chase player
        // Search for player
        // Investigate noise
        // Stun response
        // Camera panning
        // Alarm response

        // TODO: Check for special components in the transition checks for those states.
        Self {
            guard: Guard,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default()
                .trans::<Ping, _>(done(None), Pong)
                .trans::<Pong, _>(done(None), Ping),
            ping: Ping,
        }
    }
}

/// Designates a guard entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Guard;

/// A [`Guard`] that can see the player.
#[derive(Clone, Component, Debug, Default)]
pub struct Vision {
    pub distance: f32,
    pub fov: f32,
}

/// A [`Guard`] that can hear and respond to sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct Hearing {
    pub radius: f32,
}

/// A [`Guard`] that's able to be stunned.
#[derive(Clone, Component, Debug, Default)]
pub struct Stunnable;

const SPIN_DELAY_MILLIS: u64 = 400;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Ping;

#[derive(Clone, Component, Copy, Reflect)]
#[component(storage = "SparseSet")]
pub struct Pong;

fn ping(mut commands: Commands, query: Query<Entity, Added<Ping>>) {
    for entity in &query {
        commands.actions(entity).add_many(actions![
            RepeatSequence::new(
                Repeat::Times(2),
                actions![
                    MoveAction::new(MoveTo::Heading(Direction3d::NEG_X)),
                    WaitAction::new(Duration::from_millis(SPIN_DELAY_MILLIS)),
                    MoveAction::new(MoveTo::Heading(Direction3d::NEG_Z)),
                    WaitAction::new(Duration::from_millis(SPIN_DELAY_MILLIS)),
                    MoveAction::new(MoveTo::Heading(Direction3d::X)),
                    WaitAction::new(Duration::from_millis(SPIN_DELAY_MILLIS)),
                    MoveAction::new(MoveTo::Heading(Direction3d::Z)),
                    WaitAction::new(Duration::from_millis(SPIN_DELAY_MILLIS)),
                ]
            ),
            StateDoneAction::new(Done::Success)
        ]);
    }
}

fn pong(mut commands: Commands, query: Query<Entity, Added<Ping>>) {
    let movement_range = 0.5;

    for entity in &query {
        commands.actions(entity).add_many(actions![
            MoveAction::new(MoveTo::Destination(Vec3::new(
                movement_range,
                0.0,
                movement_range
            ))),
            MoveAction::new(MoveTo::Destination(Vec3::new(
                movement_range,
                0.0,
                -movement_range
            ))),
            MoveAction::new(MoveTo::Destination(Vec3::new(
                -movement_range,
                0.0,
                -movement_range
            ))),
            MoveAction::new(MoveTo::Destination(Vec3::new(
                -movement_range,
                0.0,
                movement_range
            ))),
            MoveAction::new(MoveTo::Destination(Vec3::new(0.0, 0.0, 0.0))),
            MoveAction::new(MoveTo::Heading(Direction3d::Z)),
            WaitAction::new(Duration::from_millis(SPIN_DELAY_MILLIS)),
            StateDoneAction::new(Done::Success)
        ]);
    }
}

// TODO: Retrieve starting location position and direction from marked entity.
// TODO: All nodes of a specific patrol route children of one encompassing
// entity?
