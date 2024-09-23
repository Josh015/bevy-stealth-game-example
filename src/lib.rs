mod actions;
mod assets;
mod components;
mod events;
mod game_state;
mod spawners;
mod system_params;
mod ui;
mod util;

pub use actions::*;
pub use assets::*;
pub use bevy::prelude::*;
pub use components::*;
pub use events::*;
pub use game_state::*;
pub use spawners::*;
pub use ui::*;
pub use util::*;

pub struct BevyStealthSoundGamePlugin;

impl Plugin for BevyStealthSoundGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActionsPlugin,
            AssetsPlugin,
            ComponentsPlugin,
            GameStatePlugin,
            EventsPlugin,
            SpawnersPlugin,
            UiPlugin,
        ));
    }
}
