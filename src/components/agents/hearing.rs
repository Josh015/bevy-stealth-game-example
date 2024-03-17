use bevy::{app::prelude::*, ecs::prelude::*};

pub(super) struct HearingPlugin;

impl Plugin for HearingPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins((movement::MovementPlugin,));
    }
}

/// A hearing radius that can trigger a [`HeardSoundEvent`](crate::events::HeardSoundEvent).
#[derive(Clone, Component, Debug, Default)]
pub struct Hearing {
    pub radius: f32,
}
