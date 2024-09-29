use bevy::prelude::*;

mod blueprints;
mod emotes;
mod sound_waves;

pub use blueprints::*;
pub use emotes::*;
pub use sound_waves::*;

pub(super) struct SpawnersPlugin;

impl Plugin for SpawnersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BlueprintsPlugin, EmotesPlugin, SoundWavesPlugin));
    }
}
