mod chase_player;
mod check_alarm;
mod check_noise;
mod patrol;
mod stunned;

use bevy::prelude::*;
use bevy_sequential_actions::*;
pub use chase_player::*;
pub use check_alarm::*;
pub use check_noise::*;
pub use patrol::*;
use seldom_state::prelude::{done, AnyState, StateMachine};
pub use stunned::*;

use crate::prelude::*;

pub(super) struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ChasePlayerPlugin,
            CheckAlarmPlugin,
            CheckNoisePlugin,
            PatrolPlugin,
            StunnedPlugin,
        ));
    }
}

/// Required components for a [`Guard`] entity.
#[derive(Bundle)]
pub struct GuardBundle {
    pub guard: Guard,
    pub actions_bundle: ActionsBundle,
    pub state_machine: StateMachine,
    pub patrol: Patrol,
}

impl Default for GuardBundle {
    fn default() -> Self {
        Self {
            guard: Guard,
            actions_bundle: ActionsBundle::new(),
            state_machine: StateMachine::default()
                // TODO: Remove once events wired up.
                .trans::<ChasePlayer, _>(done(None), Patrol)
                .trans::<Patrol, _>(done(None), ChasePlayer::default())

                // TODO: Implement events and test.
                .trans::<AnyState, _>(on_event::<StunnedEnemyEvent>(), Stunned)
                .trans::<AnyState, _>(done(None), Patrol)
                .trans::<Patrol, _>(
                    on_event::<SawPlayerEvent>(),
                    ChasePlayer::default(),
                )
                .trans::<CheckNoise, _>(
                    on_event::<SawPlayerEvent>(),
                    ChasePlayer::default(),
                )
                .trans::<CheckAlarm, _>(
                    on_event::<SawPlayerEvent>(),
                    ChasePlayer::default(),
                )
                .trans::<Patrol, _>(
                    on_event::<AlarmEvent>(),
                    CheckAlarm::default(),
                )
                .trans::<CheckNoise, _>(
                    on_event::<AlarmEvent>(),
                    CheckAlarm::default(),
                )
                .trans::<Patrol, _>(on_event::<HeardNoiseEvent>(), CheckNoise),
            patrol: Patrol,
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

// TODO: Use parent observer to bubble event up from child component.

/// A [`Guard`] that can hear and respond to sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct Hearing {
    pub radius: f32,
}
