use bevy::{animation::AnimationClip, ecs::prelude::*, utils::HashMap};

/// Stores animations for a given glTF scene.
#[derive(Clone, Component, Debug, Default)]
pub struct AnimationClips(pub HashMap<String, AnimationClip>);
