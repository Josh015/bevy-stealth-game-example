use bevy::prelude::*;

/// Required components for a [`Emote`] entity.
#[derive(Bundle)]
pub struct EmoteBundle {
    pub emote: Emote,
    // TODO: Probably sprite bundle?
}

/// An image that temporarily appears above and follows another entity.
#[derive(Clone, Component, Debug)]
pub struct Emote {
    source: Entity,
    // TODO: Some kind of dissipation timer, or rely on particle system?
}

// TODO: Need some kind of emote emitter or fixed height as a child entity?
