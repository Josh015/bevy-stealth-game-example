use bevy::prelude::*;

/// Required components for a [`Player`] entity.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        // TODO: Set up player input mappings here.

        Self { player: Player }
    }
}

/// Entity that can be targeted by enemy units.
#[derive(Clone, Component, Debug, Default)]
pub struct Player;

/// Blocks the Player from being seen by Vision.
#[derive(Clone, Component, Debug, Default)]
pub struct Invisibility; // timer: Timer

/// Greatly reduces the emission radius of footstep sound waves.
#[derive(Clone, Component, Debug, Default)]
pub struct QuietFootsteps; // timer: Timer

/// Allows the player to control their currently equipped firearm child entity.
#[derive(Clone, Component, Debug)]
pub struct EquippedFirearm(Entity);
