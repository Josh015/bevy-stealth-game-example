mod blueprints;
mod campaigns;
mod emotes;
mod levels;
mod sound_waves;

pub use blueprints::*;
pub use campaigns::*;
pub use emotes::*;
pub use levels::*;
pub use sound_waves::*;

use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

use crate::game_state::GameState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::StartMenu),
        )
        .configure_loading_state(
            LoadingStateConfig::new(GameState::AssetLoading)
                .load_collection::<GameAssets>()
                .init_resource::<PreloadedBlueprintAssets>(),
        )
        .add_plugins((
            BlueprintsPlugin,
            CampaignsPlugin,
            EmotesPlugin,
            LevelsPlugin,
            SoundWavesPlugin,
        ));
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "blueprints", collection(mapped, typed))]
    pub blueprints: HashMap<AssetFileStem, Handle<BlueprintConfig>>,

    #[asset(path = "emotes", collection(mapped, typed))]
    pub emotes: HashMap<AssetFileStem, Handle<EmoteConfig>>,

    #[asset(path = "campaigns", collection(mapped, typed))]
    pub campaigns: HashMap<AssetFileStem, Handle<CampaignConfig>>,

    #[asset(path = "levels", collection(mapped, typed))]
    pub levels: HashMap<AssetFileStem, Handle<LevelConfig>>,

    #[asset(path = "sound_waves", collection(mapped, typed))]
    pub sound_waves: HashMap<AssetFileStem, Handle<SoundWaveConfig>>,
}
