use bevy::prelude::*;
use strum::EnumIter;

use crate::states::GameState;

pub struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ActiveAfterLoadingSet
                .run_if(not(in_state(GameState::AssetLoading))),
        )
        .configure_sets(
            Update,
            ActiveWhenPausedSet
                .in_set(ActiveAfterLoadingSet)
                .run_if(in_state(GameState::Paused)),
        )
        .configure_sets(
            Update,
            StopWhenPausedSet
                .in_set(ActiveAfterLoadingSet)
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_sets(
            Update,
            ActiveDuringGameplaySet
                .in_set(ActiveAfterLoadingSet)
                .after(StopWhenPausedSet)
                .run_if(in_state(GameState::Gameplay)),
        )
        .configure_sets(
            PostUpdate,
            ActiveAfterLoadingSet
                .run_if(not(in_state(GameState::AssetLoading))),
        )
        .configure_sets(
            PostUpdate,
            StopWhenPausedSet
                .in_set(ActiveAfterLoadingSet)
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_sets(
            PostUpdate,
            ActiveDuringGameplaySet
                .in_set(ActiveAfterLoadingSet)
                .after(StopWhenPausedSet)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

/// Systems that run outside the loading state.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct ActiveAfterLoadingSet;

/// Systems that stop when the game is paused.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct StopWhenPausedSet;

/// Systems that are active when the game is paused.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct ActiveWhenPausedSet;

/// Systems that only run during gameplay.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct ActiveDuringGameplaySet;
