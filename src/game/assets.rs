use bevy::{prelude::*, reflect::TypePath, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use super::state::GameState;

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ActorConfig>::new(&["actor.ron"]))
            .add_plugins(RonAssetPlugin::<SoundWaveConfig>::new(&[
                "sound_wave.ron",
            ]))
            .add_loading_state(
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
    //Trigger {} // Probably want to have a sub-enum with pre-allowed events?
    FloorSwitch,
    Door,
    Glass,
    Movement {
        linear_speed: f32,
        angular_speed: f32,
    },
    Physics {
        radius: f32,
    },
    Footsteps {
        sound_wave: String,
    },
    DropShadow,
    Vision,
    Hearing,
    Stunnable,
    Barrier,
    BlocksVision,
    DeflectsSounds,
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
