use bevy::{
    ecs::prelude::*,
    utils::{petgraph::adj::NodeIndex, HashMap},
};

/// Stores animations for a given glTF scene.
#[derive(Clone, Component, Debug, Default)]
pub struct AnimationClips(pub HashMap<String, NodeIndex>);
