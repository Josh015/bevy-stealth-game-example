use bevy::{
    ecs::{prelude::*, system::SystemParam},
    prelude::*,
};
use std::time::Duration;

use crate::prelude::*;

const ANIMATION_TRANSITION_DELAY: Duration = Duration::from_millis(500);

/// Allows animations to easily be played on entities that support them.
#[derive(SystemParam)]
pub struct Animations<'w, 's> {
    linked_entities_query:
        Query<'w, 's, (&'static AnimationClips, &'static AnimationEntityLink)>,
    animation_players_query: Query<
        'w,
        's,
        (
            &'static mut AnimationPlayer,
            &'static mut AnimationTransitions,
        ),
    >,
}

impl<'w, 's> Animations<'w, 's> {
    /// Looks up an animation clip by name and plays it on an entity.
    pub fn play_clip_name(&mut self, entity: Entity, clip_name: &str) {
        let Ok((animation_clips, animation_entity_link)) =
            self.linked_entities_query.get_mut(entity)
        else {
            return;
        };

        let Some(animation_clip) = animation_clips.0.get(clip_name) else {
            return;
        };

        if let Ok((mut player, mut transitions)) = self
            .animation_players_query
            .get_mut(animation_entity_link.0)
        {
            transitions
                .play(&mut player, *animation_clip, ANIMATION_TRANSITION_DELAY)
                .repeat();
        }
    }

    /// Plays an animation clip on an entity.
    pub fn play_clip(
        &mut self,
        entity: Entity,
        animation_clip: AnimationNodeIndex,
    ) {
        let Ok((_, animation_entity_link)) =
            self.linked_entities_query.get_mut(entity)
        else {
            return;
        };

        if let Ok((mut player, mut transitions)) = self
            .animation_players_query
            .get_mut(animation_entity_link.0)
        {
            transitions
                .play(&mut player, animation_clip, ANIMATION_TRANSITION_DELAY)
                .repeat();
        }
    }

    /// Gets the index for this entity's currently playing animation.
    pub fn get_current_animation(
        &self,
        entity: Entity,
    ) -> Option<AnimationNodeIndex> {
        let Ok((_, animation_entity_link)) =
            self.linked_entities_query.get(entity)
        else {
            return None;
        };
        let Ok((_, transitions)) =
            self.animation_players_query.get(animation_entity_link.0)
        else {
            return None;
        };

        transitions.get_main_animation()
    }
}
