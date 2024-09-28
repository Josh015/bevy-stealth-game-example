use bevy::{ecs::prelude::*, prelude::*};
use derive_new::new;
use strum::IntoEnumIterator;

use crate::prelude::*;

pub(super) struct ForStatePlugin;

impl Plugin for ForStatePlugin {
    fn build(&self, app: &mut App) {
        for state in GameState::iter() {
            app.add_systems(
                OnEnter(state),
                despawn_invalid_entities_for_state::<GameState>,
            );
        }
    }
}

/// Tags an entity to only exist in the provided game states.
#[derive(Clone, Component, Debug, new)]
pub struct ForStates<S: States> {
    #[new(into_iter = "S")]
    states: Vec<S>,
}

fn despawn_invalid_entities_for_state<S: States>(
    mut commands: Commands,
    game_state: Res<State<S>>,
    query: Query<(Entity, &ForStates<S>)>,
) {
    for (entity, for_states) in &query {
        if !for_states.states.contains(game_state.get()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
