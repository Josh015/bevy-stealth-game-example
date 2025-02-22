use bevy::{asset::*, prelude::*, reflect::TypePath};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub(super) struct EmotesPlugin;

impl Plugin for EmotesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<EmoteConfig>::new(&["emote.ron"]));
    }
}

/// Configs for spawnable entities.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct EmoteConfig {
    pub image: String,
    // sound??
}
