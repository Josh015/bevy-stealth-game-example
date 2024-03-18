use bevy::app::prelude::*;

mod firearm;
mod footsteps;
mod guard;
mod hearing;
mod panning;
mod pickup;
mod player;
mod start_location;
mod vision;

pub use firearm::*;
pub use footsteps::*;
pub use guard::*;
pub use hearing::*;
pub use panning::*;
pub use pickup::*;
pub use player::*;
pub use start_location::*;
pub use vision::*;

pub(super) struct AgentsPlugin;

impl Plugin for AgentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GuardPlugin, HearingPlugin, VisionPlugin));
    }
}
