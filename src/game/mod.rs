use bevy::prelude::*;

pub mod assets;
pub mod state;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((assets::AssetsPlugin, state::StatePlugin));
    }
}
