use std::time::Duration;

use bevy::{
    ecs::{prelude::*, system::SystemParam},
    prelude::*,
    utils::HashMap,
};

use crate::game::LoadedSet;

const ANIMATION_TRANSITION_DELAY_MILLIS: u64 = 500;
const DEFAULT_ANIMATION: &str = "idle";

pub(super) struct AnimationsPlugin;

impl Plugin for AnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (start_default_animation, link_animations)
                .chain()
                .in_set(LoadedSet),
        );
    }
}

/// Stores human-friendly names mapped to [`AnimationClip`] handles.
#[derive(Clone, Component, Debug, Default)]
pub struct Animations(pub HashMap<String, Handle<AnimationClip>>);

/// Allows a parent entity to access the [`AnimationPlayer`] entity buried
/// within its [`Scene`] hierarchy.
#[derive(Component, Debug)]
pub struct AnimationEntityLink(pub Entity);

/// Allows animations to easily be played on entities that support them.
#[derive(SystemParam)]
pub struct Animator<'w, 's> {
    query: Query<'w, 's, (&'static Animations, &'static AnimationEntityLink)>,
    animation_players: Query<'w, 's, &'static mut AnimationPlayer>,
}

impl<'w, 's> Animator<'w, 's> {
    pub fn play_animation_for_entity(
        &mut self,
        target: Entity,
        animation_name: &str,
    ) {
        if let Ok((animations, animation_entity_link)) =
            self.query.get_mut(target)
        {
            if let Some(animation) = animations.0.get(animation_name) {
                if let Ok(mut animation_player) =
                    self.animation_players.get_mut(animation_entity_link.0)
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

fn start_default_animation(
    mut animator: Animator,
    query: Query<Entity, (With<Animations>, Added<AnimationEntityLink>)>,
) {
    for entity in &query {
        animator.play_animation_for_entity(entity, DEFAULT_ANIMATION);
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
