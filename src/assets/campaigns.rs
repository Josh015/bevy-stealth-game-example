use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub(super) struct CampaignsPlugin;

impl Plugin for CampaignsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<CampaignConfig>::new(&[
            "campaign.ron",
        ]));
    }
}

#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct CampaignConfig {
    title: String,
}
