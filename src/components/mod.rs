use bevy::app::prelude::*;

mod animation_clips;
mod barrier;
mod door;
mod drop_shadow;
mod floor_switch;
mod footsteps;
mod glass;
mod guard;
mod mover;
mod physics;
mod pickup;
mod player;
mod security_camera;
mod spawn_point;
mod start_location;
mod trigger;
mod waypoint;
mod weapon;

pub use animation_clips::*;
pub use barrier::*;
pub use door::*;
pub use drop_shadow::*;
pub use floor_switch::*;
pub use footsteps::*;
pub use glass::*;
pub use guard::*;
pub use mover::*;
pub use physics::*;
pub use pickup::*;
pub use player::*;
pub use security_camera::*;
pub use spawn_point::*;
pub use start_location::*;
pub use trigger::*;
pub use waypoint::*;
pub use weapon::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GuardPlugin, MoverPlugin, PhysicsPlugin));
    }
}
