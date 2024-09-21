use bevy::prelude::*;

mod actors;
mod sound_waves;

pub use actors::*;
pub use sound_waves::*;

pub(super) struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ActorsPlugin, SoundWavesPlugin));
    }
}
