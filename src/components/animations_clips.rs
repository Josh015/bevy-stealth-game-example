use bevy::{ecs::prelude::*, prelude::*, utils::HashMap};

use crate::prelude::*;

const DEFAULT_ANIMATION: &str = "idle";

pub(super) struct AnimationClipsPlugin;

impl Plugin for AnimationClipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Paused),
            pause_all_animation_players,
        )
        .add_systems(OnExit(GameState::Paused), resume_all_animation_players)
        .add_systems(
            Update,
            (
                pause_added_animation_players.in_set(ActiveWhenPausedSet),
                (start_default_animation, link_animation_players).chain(),
            ),
        );
    }
}

/// Stores human-friendly names mapped to [`AnimationClip`] handles.
#[derive(Clone, Component, Debug, Default)]
pub struct AnimationClips(pub HashMap<String, Handle<AnimationClip>>);

/// Allows a parent entity to access the [`AnimationPlayer`] entity buried
/// within its [`Scene`] hierarchy.
#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

fn pause_all_animation_players(
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for mut animation_player in &mut animation_players {
        animation_player.pause_all();
    }
}

fn resume_all_animation_players(
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for mut animation_player in &mut animation_players {
        animation_player.resume_all();
    }
}

fn pause_added_animation_players(
    mut animation_players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut animation_player in &mut animation_players {
        animation_player.pause_all();
    }
}

fn start_default_animation(
    mut animations: Animations,
    query: Query<Entity, (With<AnimationClips>, Added<AnimationEntityLink>)>,
) {
    for entity in &query {
        animations.play_clip(entity, DEFAULT_ANIMATION);
    }
}

fn link_animation_players(
    mut commands: Commands,
    animation_players_query: Query<Entity, Added<AnimationPlayer>>,
    all_entities_with_parents_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
) {
    // Get all the Animation players which can be deep and hidden in the
    // hierarchy.
    for entity_with_animation_player in animation_players_query.iter() {
        let top_entity = get_top_parent(
            entity_with_animation_player,
            &all_entities_with_parents_query,
        );

        // If the top parent has an animation config ref then link the player to
        // the config.
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!(
                "Problem with multiple animation players for the same top \
                 parent"
            );
        } else {
            commands.entity(top_entity).insert(AnimationEntityLink(
                entity_with_animation_player.clone(),
            ));
        }
    }
}

fn get_top_parent(
    mut current_entity: Entity,
    all_entities_with_parents_query: &Query<&Parent>,
) -> Entity {
    // Loop up all the way to the top parent.
    loop {
        if let Ok(ref_to_parent) =
            all_entities_with_parents_query.get(current_entity)
        {
            current_entity = ref_to_parent.get();
        } else {
            break;
        }
    }

    current_entity
}
