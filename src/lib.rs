mod actions;
mod assets;
mod components;
mod events;
mod game_state;
mod spawners;
mod system_params;
mod ui;
mod util;

mod prelude {
    pub use crate::actions::*;
    pub use crate::assets::*;
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::game_state::*;
    pub use crate::spawners::*;
    pub use crate::system_params::*;
    pub use crate::ui::*;
    pub use crate::util::*;
}

pub use bevy::prelude::*;
pub use prelude::*;

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
