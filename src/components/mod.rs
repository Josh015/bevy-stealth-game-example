use bevy::app::prelude::*;

mod animation_clips;
mod barrier;
mod blocks_vision;
mod deflects_sounds;
mod door;
mod drop_shadow;
mod equipped_firearm;
mod footsteps;
mod guard;
mod hearing;
mod invisibility;
mod mover;
mod panning;
mod physics;
mod pickup;
mod player;
mod quiet_footsteps;
mod security_camera;
mod shatterable;
mod spawn_point;
mod start_location;
mod stunnable;
mod trigger;
mod vision;
mod waypoint;
mod weapon;

pub use animation_clips::*;
pub use barrier::*;
pub use blocks_vision::*;
pub use deflects_sounds::*;
pub use door::*;
pub use drop_shadow::*;
pub use equipped_firearm::*;
pub use footsteps::*;
pub use guard::*;
pub use hearing::*;
pub use invisibility::*;
pub use mover::*;
pub use panning::*;
pub use physics::*;
pub use pickup::*;
pub use player::*;
pub use quiet_footsteps::*;
pub use security_camera::*;
pub use shatterable::*;
pub use spawn_point::*;
pub use start_location::*;
pub use stunnable::*;
pub use trigger::*;
pub use vision::*;
pub use waypoint::*;
pub use weapon::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GuardPlugin,
            HearingPlugin,
            MoverPlugin,
            PhysicsPlugin,
            VisionPlugin,
        ));
    }
}
