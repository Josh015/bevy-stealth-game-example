pub mod menu;

use bevy::prelude::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((menu::MenuPlugin,));
    }
}
