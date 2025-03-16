mod debug;
mod menu;

pub use debug::*;
pub use menu::*;

use bevy::prelude::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((DebugPlugin, MenuPlugin));
    }
}
