mod actions;
mod assets;
mod components;
mod events;
mod game_state;
mod system_params;
mod ui;
mod util;

mod prelude {
    pub use crate::{
        actions::*, assets::*, components::*, events::*, game_state::*,
        system_params::*, ui::*, util::*,
    };
}

pub use bevy::prelude::*;
pub use prelude::*;

pub struct BevyStealthGameExamplePlugin;

impl Plugin for BevyStealthGameExamplePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ActionsPlugin,
            AssetsPlugin,
            ComponentsPlugin,
            GameStatePlugin,
            EventsPlugin,
            UiPlugin,
        ));
    }
}
