use bevy::{asset::*, prelude::*, reflect::TypePath};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub(super) struct SoundWavesPlugin;

impl Plugin for SoundWavesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<SoundWaveConfig>::new(&[
            "sound_wave.ron",
        ]));
    }
}

/// Configs for spawnable entities.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct SoundWaveConfig {
    pub color: String,
    // sound??
}
