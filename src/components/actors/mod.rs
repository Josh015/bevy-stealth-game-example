use bevy::app::prelude::*;

mod animation_clips;
mod blocks_vision;
mod deflects_sounds;
mod door;
mod drop_shadow;
mod equipped_firearm;
mod footsteps;
mod guard;
mod hearing;
mod panning;
mod pickup;
mod player;
mod security_camera;
mod shatterable;
mod spawn_point;
mod start_location;
mod stunnable;
mod vision;
mod wall;
mod waypoint;
mod weapon;

pub use animation_clips::*;
pub use blocks_vision::*;
pub use deflects_sounds::*;
pub use door::*;
pub use drop_shadow::*;
pub use equipped_firearm::*;
pub use footsteps::*;
pub use guard::*;
pub use hearing::*;
pub use panning::*;
pub use pickup::*;
pub use player::*;
pub use security_camera::*;
pub use shatterable::*;
pub use spawn_point::*;
pub use start_location::*;
pub use stunnable::*;
pub use vision::*;
pub use wall::*;
pub use waypoint::*;
pub use weapon::*;

pub(super) struct AgentsPlugin;

impl Plugin for AgentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GuardPlugin, HearingPlugin, VisionPlugin));
    }
}
