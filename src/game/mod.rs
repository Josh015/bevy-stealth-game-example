use bevy::prelude::*;

mod assets;
mod events;
mod spawn;
mod state;
mod system_params;
mod util;

pub use assets::*;
pub use events::*;
pub use spawn::*;
pub use state::*;
pub use system_params::*;
pub use util::*;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AssetsPlugin, EventsPlugin, SpawnPlugin, StatePlugin));
    }
}
