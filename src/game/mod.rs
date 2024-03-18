use bevy::prelude::*;

mod assets;
mod state;

pub use assets::*;
pub use state::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AssetsPlugin, StatePlugin));
    }
}
