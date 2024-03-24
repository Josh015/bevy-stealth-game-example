use crate::configs::*;

use super::game_state::*;
use bevy::{asset::*, prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::StartMenu),
        )
        .configure_loading_state(
            LoadingStateConfig::new(GameState::AssetLoading)
                .load_collection::<GameAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "actors", collection(mapped, typed))]
    pub actors: HashMap<FileStem, Handle<ActorConfig>>,

    #[asset(path = "sound_waves", collection(mapped, typed))]
    pub sound_waves: HashMap<FileStem, Handle<SoundWaveConfig>>,
}
