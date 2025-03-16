use bevy::prelude::*;
use strum::EnumIter;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
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
