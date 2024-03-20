use bevy::{prelude::*, reflect::TypePath, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use serde::Deserialize;

use super::state::GameState;

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YamlAssetPlugin::<ActorConfig>::new(&["actor.yaml"]))
            .add_plugins(YamlAssetPlugin::<SoundWaveConfig>::new(&[
                "sound_wave.yaml",
            ]))
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::StartMenu),
            )
            .configure_loading_state(
                LoadingStateConfig::new(GameState::Loading)
                    .load_collection::<GameAssets>(),
            );
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "config/actors", collection(typed, mapped))]
    pub actors: HashMap<String, Handle<ActorConfig>>,

    #[asset(path = "config/sound_waves", collection(typed))]
    pub sound_waves: Vec<Handle<SoundWaveConfig>>,
}

/// Configs for spawnable entities.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct ActorConfig {
    pub name: String,
    pub components: Vec<ComponentConfig>,
}

/// Configs for entity components.
#[derive(Clone, Debug, Deserialize)]
pub enum ComponentConfig {
    Player,
    Guard,
    SecurityCamera,
    Pickup,
    Weapon,
    Powerup { file_name: String },
    //Trigger {} // Probably want to have a sub-enum with pre-allowed events?
    Door,
    Barrier,
    Mover, // TODO: Let them specify speeds?
    Physics { radius: f32 },
    Footsteps { sound_wave: String },
    DropShadow,
    Eyes,
    Ears,
    Stunnable,
    BlocksVision,
    DeflectsSounds,
    Shatterable,
    Scene(String),
    AnimationClips(HashMap<String, String>),
}

/// Configs for spawnable entities.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct SoundWaveConfig {
    pub name: String,
    pub color: String,
    // sound??
}
