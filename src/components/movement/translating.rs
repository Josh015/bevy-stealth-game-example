use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;

pub(super) struct TranslatingPlugin;

impl Plugin for TranslatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, translating);
    }
}

/// Translates an entity.
#[derive(Clone, Component, Debug, new)]
pub struct Translating {
    translation: Vec3,
}

fn translating(
    time: Res<Time>,
    mut query: Query<(&Translating, &mut Transform)>,
) {
    for (translating, mut transform) in &mut query {
        transform.translation += translating.translation * time.delta_seconds();
    }
}
