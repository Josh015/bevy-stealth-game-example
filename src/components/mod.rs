// TODO: These all need to act as physical attributes, status conditions,
// senses, event emitters, and data storage. Don’t have them perform AI logic!

use bevy::app::{App, Plugin};

pub mod agents;
pub mod level;
pub mod movement;
pub mod physics;
pub mod states;
pub mod status_effects;
pub mod trigger;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((movement::MovementPlugin,));
    }
}
