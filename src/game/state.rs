use bevy::prelude::*;
use spew::prelude::SpewSystemSet;
use strum::{EnumIter, IntoEnumIterator};

pub(super) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .configure_sets(
                Update,
                LoadedSet.run_if(not(in_state(GameState::Loading))),
            )
            .configure_sets(
                Update,
                ActiveWhenPausedSet
                    .in_set(LoadedSet)
                    .run_if(in_state(GameState::Paused)),
            )
            .configure_sets(
                Update,
                StoppedWhenPausedSet
                    .in_set(LoadedSet)
                    .run_if(not(in_state(GameState::Paused))),
            )
            .configure_sets(
                Update,
                PlayableSet
                    .in_set(LoadedSet)
                    .after(StoppedWhenPausedSet)
                    .before(SpewSystemSet)
                    .run_if(in_state(GameState::Playing)),
            )
            .configure_sets(
                PostUpdate,
                LoadedSet.run_if(not(in_state(GameState::Loading))),
            )
            .configure_sets(
                PostUpdate,
                StoppedWhenPausedSet
                    .in_set(LoadedSet)
                    .run_if(not(in_state(GameState::Paused))),
            )
            .configure_sets(
                PostUpdate,
                PlayableSet
                    .in_set(LoadedSet)
                    .after(StoppedWhenPausedSet)
                    .run_if(in_state(GameState::Playing)),
            );

        for state in GameState::iter() {
            app.add_systems(
                OnEnter(state),
                despawn_invalid_entities_for_state::<GameState>,
            );
        }
    }
}

// All the app's possible states.
#[derive(
    Clone, Copy, Debug, Default, EnumIter, Eq, Hash, PartialEq, States,
)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    Playing,
    Paused,
}

/// Systems that are always running after everything is loaded.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct LoadedSet;

/// Systems that stop when the game is paused.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct StoppedWhenPausedSet;

/// Systems that are active when the game is paused.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct ActiveWhenPausedSet;

/// Systems that only run during gameplay.
#[derive(Clone, Debug, Eq, Hash, PartialEq, SystemSet)]
pub struct PlayableSet;

/// Tags an entity to only exist in its associated game states.
#[derive(Clone, Component, Debug)]
pub struct ForStates<S: States>(pub Vec<S>);

fn despawn_invalid_entities_for_state<S: States>(
    mut commands: Commands,
    game_state: Res<State<S>>,
    query: Query<(Entity, &ForStates<S>)>,
) {
    for (entity, for_states) in &query {
        if !for_states.0.contains(game_state.get()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
