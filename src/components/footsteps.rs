use bevy::prelude::*;

use crate::SoundWaveConfig;

pub(super) struct FootstepsPlugin;

impl Plugin for FootstepsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Update, (ping, pong).chain().in_set(StopWhenPausedSet));
    }
}

// Causes entity to emit sound-waves while translating, but not rotating.
// Specify globally defined sound-wave type.
#[derive(Clone, Component, Debug)]
pub struct Footsteps {
    pub sound_wave: Handle<SoundWaveConfig>,
}
