use std::time::Duration;

use bevy::{ecs::prelude::*, prelude::*, utils::HashMap};

use crate::game::LoadedSet;

const ANIMATION_TRANSITION_DELAY_MILLIS: u64 = 500;

pub(super) struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (link_animations, start_animating).in_set(LoadedSet),
        );
    }
}

/// Required components for an [`Animations`] entity.
#[derive(Bundle)]
pub struct AnimationsBundle {
    pub animations: Animations,
    pub current_animation: CurrentAnimation,
}

impl Default for AnimationsBundle {
    fn default() -> Self {
        Self {
            animations: Animations::default(),
            current_animation: CurrentAnimation("idle".to_owned()),
        }
    }
}

/// Stores human-friendly names mapped to [`AnimationClip`] handles.
#[derive(Clone, Component, Debug, Default)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

/// An entity that is currently playing an animation.
#[derive(Clone, Component, Debug, Default)]
pub struct CurrentAnimation(pub String);

/// Allows a parent entity to access the [`AnimationPlayer`] entity buried
/// within its [`Scene`] hierarchy.
#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

fn start_animating(
    query: Query<
        (&CurrentAnimation, &Animations, &AnimationEntityLink),
        Added<CurrentAnimation>,
    >,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for (current_animation, animations, animation_entity_link) in &query {
        if let Some(animation) = animations.0.get(&current_animation.0) {
            if let Ok(mut animation_player) =
                animation_players.get_mut(animation_entity_link.0)
            {
                animation_player
                    .play_with_transition(
                        animation.clone_weak(),
                        Duration::from_millis(
                            ANIMATION_TRANSITION_DELAY_MILLIS,
                        ),
                    )
                    .repeat();
            }
        }
    }
}

fn link_animations(
    mut commands: Commands,
    animation_players_query: Query<Entity, Added<AnimationPlayer>>,
    all_entities_with_parents_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
) {
    // Get all the Animation players which can be deep and hidden in the hierarchy.
    for entity_with_animation_player in animation_players_query.iter() {
        let top_entity = get_top_parent(
            entity_with_animation_player,
            &all_entities_with_parents_query,
        );

        // If the top parent has an animation config ref then link the player to the config.
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animation players for the same top parent");
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
