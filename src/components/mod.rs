use bevy::app::prelude::*;

mod animation_clips;
mod barrier;
mod drop_shadow;
mod ears;
mod equipped_firearm;
mod eyes;
mod footsteps;
mod guard;
mod invisibility;
mod mover;
mod panning;
mod physics;
mod pickup;
mod player;
mod quiet_footsteps;
mod security_camera;
mod spawn_point;
mod start_location;
mod stunnable;
mod trigger;
mod waypoint;
mod weapon;

pub use animation_clips::*;
pub use barrier::*;
pub use drop_shadow::*;
pub use ears::*;
pub use equipped_firearm::*;
pub use eyes::*;
pub use footsteps::*;
pub use guard::*;
pub use invisibility::*;
pub use mover::*;
pub use panning::*;
pub use physics::*;
pub use pickup::*;
pub use player::*;
pub use quiet_footsteps::*;
pub use security_camera::*;
pub use spawn_point::*;
pub use start_location::*;
pub use stunnable::*;
pub use trigger::*;
pub use waypoint::*;
pub use weapon::*;

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EarsPlugin,
            EyesPlugin,
            GuardPlugin,
            MoverPlugin,
            PhysicsPlugin,
        ));
    }
}
