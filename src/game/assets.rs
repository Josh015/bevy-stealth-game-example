use bevy::{prelude::*, reflect::TypePath};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use serde::Deserialize;

use super::state::GameState;

pub(super) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(YamlAssetPlugin::<ActorConfig>::new(&["actor.yaml"]))
            .add_plugins(YamlAssetPlugin::<SoundConfig>::new(&["sound.yaml"]))
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
        //         println!("total files: {}", game_assets.actors.len());
        //     },
        // );
    }
}

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "config/actors", collection(typed))]
    pub actors: Vec<Handle<ActorConfig>>,

    #[asset(path = "config/sounds", collection(typed))]
    pub sounds: Vec<Handle<SoundConfig>>,
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
    DropShadow,
    Powerup { file_name: String },
    Animation3D { scene: String }, // TODO: Determine required files
    Mesh3D { file_name: String },  // TODO: Determine required files
    Vision,
    Hearing,
    Physics { radius: f32 },
    Stunnable,
    Footsteps { sound: String }, // Look up and replace with asset handle.
    //Trigger {} // Probably want to have a sub-enum with pre-allowed events?
    Weapon,
    BlocksVision,
    DeflectsSounds,
    Shatterable,
}

/// Configs for spawnable entities.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct SoundConfig {
    pub name: String,
    pub color: String,
}
