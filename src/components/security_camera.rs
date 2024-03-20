use bevy::ecs::prelude::*;

/// Designates a security camera entity.
#[derive(Clone, Component, Debug, Default)]
pub struct SecurityCamera;

// TODO: SecurityCamera needs its own distinct vision component since it
// behaves completely differently from guards.
