use crate::components::*;
use bevy::{
    ecs::{prelude::*, system::SystemParam},
    prelude::*,
};
use std::time::Duration;

const ANIMATION_TRANSITION_DELAY_MILLIS: u64 = 500;

/// Allows animations to easily be played on entities that support them.
#[derive(SystemParam)]
pub struct Animations<'w, 's> {
    query:
        Query<'w, 's, (&'static AnimationClips, &'static AnimationEntityLink)>,
    animation_players: Query<'w, 's, &'static mut AnimationPlayer>,
}

impl<'w, 's> Animations<'w, 's> {
    /// Looks up an animation clip by name and plays it on an entity.
    pub fn play_clip(&mut self, entity: Entity, clip_name: &str) {
        if let Ok((animation_clips, animation_entity_link)) =
            self.query.get_mut(entity)
        {
            if let Some(animation_clip) = animation_clips.0.get(clip_name) {
                if let Ok(mut animation_player) =
                    self.animation_players.get_mut(animation_entity_link.0)
                {
                    animation_player
                        .play_with_transition(
                            animation_clip.clone_weak(),
                            Duration::from_millis(
                                ANIMATION_TRANSITION_DELAY_MILLIS,
                            ),
                        )
                        .repeat();
                }
            }
        }
    }

    /// Plays an animation clip on an entity.
    pub fn play_clip_handle(
        &mut self,
        entity: Entity,
        clip_handle: Handle<AnimationClip>,
    ) {
        if let Ok((_, animation_entity_link)) = self.query.get_mut(entity) {
            if let Ok(mut animation_player) =
                self.animation_players.get_mut(animation_entity_link.0)
            {
                animation_player
                    .play_with_transition(
                        clip_handle.clone_weak(),
                        Duration::from_millis(
                            ANIMATION_TRANSITION_DELAY_MILLIS,
                        ),
                    )
                    .repeat();
            }
        }
    }

    /// Gets the handle for this entity's currently playing animation.
    pub fn get_current_clip(
        &self,
        entity: Entity,
    ) -> Option<Handle<AnimationClip>> {
        let Ok((_, animation_entity_link)) = self.query.get(entity) else {
            return None;
        };
        let Ok(animation_player) =
            self.animation_players.get(animation_entity_link.0)
        else {
            return None;
        };

        Some(animation_player.animation_clip().clone())
    }
}
