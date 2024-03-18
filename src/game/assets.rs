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
        // .add_systems(
        //     OnExit(GameState::Loading),
        //     |game_assets: Res<GameAssets>| {
        //         game_assets.sound_waves.iter().for_each(|s| {
        //             println!("sound_wave: {}", s.0);
        //         });
        //     },
        // );
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
    Powerup {
        file_name: String,
    },
    //Trigger {} // Probably want to have a sub-enum with pre-allowed events?
    Animation3D {
        scene: String,
        animations: HashMap<String, String>,
    },
    Mesh3D {
        scene: String,
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
    BlocksVision,
    DeflectsSounds,
    Shatterable,
}

/// Configs for spawnable entities.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct SoundWaveConfig {
    pub name: String,
    pub color: String,
}
