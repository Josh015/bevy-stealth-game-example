use bevy::prelude::*;

/// An image that temporarily appears above and follows another entity.
#[derive(Clone, Component, Debug)]
pub struct Emote {
    source: Entity,
    // TODO: Some kind of dissipation timer, or rely on particle system?
}

// TODO: Need some kind of emote emitter or fixed height as a child entity?
