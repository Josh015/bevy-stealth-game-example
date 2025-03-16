mod actions;
mod assets;
mod components;
mod events;
mod game_state;
mod system_params;
mod ui;
mod util;

pub mod prelude {
    pub use crate::{
        actions::*, assets::*, components::*, events::*, game_state::*,
        system_params::*, ui::*, util::*,
    };
}

pub use bevy::prelude::*;

pub struct LibPlugin;

impl Plugin for LibPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            actions::ActionsPlugin,
            assets::AssetsPlugin,
            components::ComponentsPlugin,
            game_state::GameStatePlugin,
            events::EventsPlugin,
            ui::UiPlugin,
        ));
    }
}
