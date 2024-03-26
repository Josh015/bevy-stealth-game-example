use bevy::{asset::*, prelude::*, reflect::TypePath, utils::HashMap};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub(super) struct ConfigsPlugin;

impl Plugin for ConfigsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ActorConfig>::new(&["actor.ron"]))
            .add_plugins(RonAssetPlugin::<SoundWaveConfig>::new(&[
                "sound_wave.ron",
            ]));
    }
}

/// Configs for spawnable entities.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct ActorConfig(pub Vec<ComponentConfig>);

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
    pub color: String,
    // sound??
}
