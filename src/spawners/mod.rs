use bevy::prelude::*;

mod actors;
mod emotes;
mod sound_waves;

pub use actors::*;
pub use emotes::*;
pub use sound_waves::*;

pub(super) struct SpawnersPlugin;

impl Plugin for SpawnersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ActorsPlugin, EmotesPlugin, SoundWavesPlugin));
    }
}
