use bevy::{app::prelude::*, ecs::prelude::*, pbr::PbrBundle};

use crate::components::physics::Physics;

use super::{hearing::Hearing, vision::Vision};

pub(super) struct GuardPlugin;

impl Plugin for GuardPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((movement::MovementPlugin,));
    }
}

/// Assigns an entity an animation and gets it to start fading.
#[derive(Bundle, Clone, Default)]
pub struct GuardBundle {
    pub pbr_bundle: PbrBundle, // SceneBundle
    pub guard: Guard,
    pub physics: Physics,
    pub vision: Vision,
    pub hearing: Hearing,
    // Footsteps
    //     sound_wave: “Guard”
    // Trigger:
    //     Game Over.
    // GuardAI
    //     Stunned
    //     ChasingPlayer
    //     SearchingForPlayer
    //     InvestigatingSound
    //     GuardingLocation | Patrolling
}

/// Designates a guard entity.
#[derive(Clone, Component, Debug, Default)]
pub struct Guard;

// TODO: Retrieve starting location position and direction from marked entity.
// TODO: All nodes of a specific patrol route children of one encompassing
// entity?
