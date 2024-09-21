use crate::{ActorConfig, PreloadedActorAssets, SoundWaveConfig};

use super::game_state::*;
use bevy::{asset::*, prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Gameplay),
        )
        .configure_loading_state(
            LoadingStateConfig::new(GameState::AssetLoading)
                .load_collection::<GameAssets>()
                .init_resource::<PreloadedActorAssets>(),
        );
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "actors", collection(mapped, typed))]
    pub actors: HashMap<AssetFileStem, Handle<ActorConfig>>,

    #[asset(path = "sound_waves", collection(mapped, typed))]
    pub sound_waves: HashMap<AssetFileStem, Handle<SoundWaveConfig>>,
}
