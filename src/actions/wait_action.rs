use crate::system_sets::*;
use bevy::{app::prelude::*, ecs::prelude::*, prelude::*};
use bevy_sequential_actions::*;
use derive_new::new;
use std::time::Duration;

pub(super) struct WaitActionPlugin;

impl Plugin for WaitActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, wait_timer_update.in_set(StopWhenPausedSet));
    }
}

/// Delays the next action by a specified duration.
#[derive(new)]
pub struct WaitAction {
    duration: Duration,

    #[new(default)]
    current: Option<Timer>,
}

impl Action for WaitAction {
    fn is_finished(&self, agent: Entity, world: &World) -> bool {
        world.get::<WaitTimer>(agent).unwrap().0.finished()
    }

    fn on_start(&mut self, agent: Entity, world: &mut World) -> bool {
        let timer = self
            .current
            .take()
            .unwrap_or(Timer::new(self.duration, TimerMode::Once));

        world.entity_mut(agent).insert(WaitTimer(timer));
        self.is_finished(agent, world)
    }

    fn on_stop(
        &mut self,
        agent: Entity,
        world: &mut World,
        reason: StopReason,
    ) {
        let wait_timer = world.entity_mut(agent).take::<WaitTimer>();

        if reason == StopReason::Paused {
            self.current = Some(wait_timer.unwrap().0);
        }
    }
}

#[derive(Component)]
struct WaitTimer(Timer);

fn wait_timer_update(time: Res<Time>, mut query: Query<&mut WaitTimer>) {
    for mut wait_timer in &mut query {
        wait_timer.0.tick(time.delta());
    }
}
