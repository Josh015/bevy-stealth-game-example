use bevy::app::prelude::*;

pub mod firearm;
pub mod footsteps;
pub mod guard;
pub mod hearing;
pub mod panning;
pub mod pickup;
pub mod player;
pub mod start_location;
pub mod vision;

pub(super) struct AgentsPlugin;

impl Plugin for AgentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            guard::GuardPlugin,
            hearing::HearingPlugin,
            vision::VisionPlugin,
        ));
    }
}
