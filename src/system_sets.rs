use crate::states::*;
use bevy::prelude::*;
use spew::prelude::SpewSystemSet;

pub(super) struct SystemSetsPlugin;

impl Plugin for SystemSetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            PostAssetLoadingSet.run_if(not(in_state(GameState::AssetLoading))),
        )
        .configure_sets(
            Update,
            ActiveWhenPausedSet
                .in_set(PostAssetLoadingSet)
                .run_if(in_state(GameState::Paused)),
        )
        .configure_sets(
            Update,
            StoppedWhenPausedSet
                .in_set(PostAssetLoadingSet)
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_sets(
            Update,
            GameplaySet
                .in_set(PostAssetLoadingSet)
                .after(StoppedWhenPausedSet)
                .before(SpewSystemSet)
                .run_if(in_state(GameState::Gameplay)),
        )
        .configure_sets(
            PostUpdate,
            PostAssetLoadingSet.run_if(not(in_state(GameState::AssetLoading))),
        )
        .configure_sets(
            PostUpdate,
            StoppedWhenPausedSet
                .in_set(PostAssetLoadingSet)
                .run_if(not(in_state(GameState::Paused))),
        )
        .configure_sets(
            PostUpdate,
            GameplaySet
                .in_set(PostAssetLoadingSet)
                .after(StoppedWhenPausedSet)
                .run_if(in_state(GameState::Gameplay)),
        );
    }
}

/// Systems that run outside the loading state.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PostAssetLoadingSet;

/// Systems that stop when the game is paused.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct StoppedWhenPausedSet;

/// Systems that are active when the game is paused.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct ActiveWhenPausedSet;

/// Systems that only run during gameplay.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct GameplaySet;
