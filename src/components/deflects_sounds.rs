use bevy::ecs::prelude::*;

/// A wall that can deflect sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct DeflectsSounds;

// Ricocheting sound waves:
// Treat DeflectSounds entities like secondary emitters.
// Emit sound at reflected angle from where source sound hit.
// For footstep waves emit wide waves. For sound gun emit small waves.
// Waves get weaker with each reflection.
// Angular diameter of sound wave affects what walls it can hit. Also affects
// what walls will block parts of the initial sound wave.
