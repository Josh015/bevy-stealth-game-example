use bevy::prelude::*;
use strum::EnumIter;

pub(super) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(
                Update,
                PostAssetLoadingSet
                    .run_if(not(in_state(GameState::AssetLoading))),
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
                    .run_if(in_state(GameState::Gameplay)),
            )
            .configure_sets(
                PostUpdate,
                PostAssetLoadingSet
                    .run_if(not(in_state(GameState::AssetLoading))),
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

// All the game's possible states.
#[derive(
    Clone, Copy, Debug, Default, EnumIter, Eq, Hash, PartialEq, States,
)]
pub enum GameState {
    #[default]
    AssetLoading,
    StartMenu,
    Paused,
    Gameplay,
    GameOver,
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
