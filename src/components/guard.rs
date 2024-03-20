use bevy::{app::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use seldom_state::prelude::*;
use std::time::Duration;

use crate::{
    actions::{MoveAction, RepeatSequence, StateDoneAction, WaitAction},
    util::Repeat,
    MoveTo,
};

pub(super) struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (ping, pong));
    }
}

/// Entities that can see the player.
#[derive(Clone, Component, Debug, Default)]
pub struct Eyes {
    pub distance: f32,
    pub fov: f32,
}

/// Entities that can hear and respond to sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct Ears {
    pub radius: f32,
}

/// A guard that's able to be stunned by the Stun Gun.
#[derive(Clone, Component, Debug, Default)]
pub struct Stunnable;

/// Designates a guard entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Guard;

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
        // TODO: Check for special components in the transition checks for those states.
        Self {
            guard: Guard,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default()
            // Whenever the player presses jump, jump
            .trans::<Ping, _>(
                done(None),
                Pong,
            )
            .trans::<Pong, _>(
                done(None),
                Ping,
            ),
            ping: Ping,
        }
    }
}

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
                    MoveAction::new(MoveTo::Direction(Direction3d::X)),
                    WaitAction::new(Duration::from_millis(400)),
                    MoveAction::new(MoveTo::Direction(Direction3d::Z)),
                    WaitAction::new(Duration::from_millis(400)),
                    MoveAction::new(MoveTo::Direction(Direction3d::NEG_X)),
                    WaitAction::new(Duration::from_millis(400)),
                    MoveAction::new(MoveTo::Direction(Direction3d::NEG_Z)),
                    WaitAction::new(Duration::from_millis(400)),
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
            StateDoneAction::new(Done::Success)
        ]);
    }
}

// TODO: Retrieve starting location position and direction from marked entity.
// TODO: All nodes of a specific patrol route children of one encompassing
// entity?
