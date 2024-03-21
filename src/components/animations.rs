use std::time::Duration;

use bevy::{ecs::prelude::*, prelude::*, utils::HashMap};

use crate::game::LoadedSet;

const ANIMATION_TRANSITION_DELAY_MILLIS: u64 = 500;

pub(super) struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                link_animations,
                start_default_animation,
                start_animating,
                stop_animating,
            )
                .in_set(LoadedSet),
        );
    }
}

/// Stores [`AnimationClip`] references for a given glTF [`Scene`].
#[derive(Clone, Component, Debug, Default)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

/// Stores the name of the default animation to run when others have completed.
#[derive(Clone, Component, Debug, Default)]
pub struct DefaultAnimation(pub String);

/// An entity that is playing an animation.
#[derive(Clone, Component, Debug, Default)]
pub struct Animating {
    pub animation_name: String,
}

/// Allows a parent entity to access the [`AnimationPlayer`] entity buried
/// within its [`Scene`] hierarchy.
#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

fn start_default_animation(
    query: Query<
        (&DefaultAnimation, &Animations, &AnimationEntityLink),
        Added<DefaultAnimation>,
    >,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for (default_animation, animations, animation_entity_link) in &query {
        if let Some(animation) = animations.0.get(&default_animation.0) {
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

fn start_animating(
    query: Query<
        (&Animating, &Animations, &AnimationEntityLink),
        Added<Animating>,
    >,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for (animating, animations, animation_entity_link) in &query {
        if let Some(animation) = animations.0.get(&animating.animation_name) {
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

fn stop_animating(
    mut removed: RemovedComponents<Animating>,
    query: Query<(&DefaultAnimation, &Animations, &AnimationEntityLink)>,
    mut animation_players: Query<&mut AnimationPlayer>,
) {
    for entity in removed.read() {
        if let Ok((default_animation, animations, animation_entity_link)) =
            query.get(entity)
        {
            if let Ok(mut animation_player) =
                animation_players.get_mut(animation_entity_link.0)
            {
                if let Some(animation) = animations.0.get(&default_animation.0)
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
}

fn link_animations(
    animation_players_query: Query<Entity, Added<AnimationPlayer>>,
    all_entities_with_parents_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
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
