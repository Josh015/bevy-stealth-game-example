use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    system_params::Animations, GameplaySet, Movement, StoredAnimation,
    MOVING_ANIMATION,
};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, control_player.in_set(GameplaySet));
    }
}

/// Required components for a [`Player`] entity.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub input_manager_bundle: InputManagerBundle<PlayerAction>,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            input_manager_bundle: InputManagerBundle::<PlayerAction> {
                action_state: ActionState::default(),
                input_map: PlayerAction::default_input_map(),
            },
        }
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

#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum PlayerAction {
    Move,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        use PlayerAction::*;

        let mut input_map = InputMap::default();

        input_map.insert(Move, DualAxis::left_stick());
        input_map.insert(Move, VirtualDPad::dpad());
        input_map.insert(Move, VirtualDPad::wasd());
        input_map.insert(Move, VirtualDPad::arrow_keys());
        input_map
    }
}

fn control_player(
    time: Res<Time>,
    mut commands: Commands,
    mut animations: Animations,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            &Movement,
            &ActionState<PlayerAction>,
            Option<&StoredAnimation>,
        ),
        With<Player>,
    >,
) {
    let (
        entity,
        mut player_transform,
        movement,
        action_state,
        stored_animation,
    ) = query.single_mut();

    if action_state.pressed(&PlayerAction::Move) {
        if stored_animation.is_none() {
            if let Some(current_animation) = animations.get_current_clip(entity)
            {
                commands
                    .entity(entity)
                    .insert(StoredAnimation(current_animation));
            }

            animations.play_clip(entity, MOVING_ANIMATION);
        }

        let clamped_axis = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .unwrap()
            .xy();
        let move_direction = clamped_axis.extend(0.0).xzy()
            * Vec3::new(1.0, 1.0, -1.0).normalize_or_zero();

        player_transform.translation +=
            move_direction * movement.linear_speed * time.delta_seconds();
    } else if let Some(stored_animation) = stored_animation {
        animations.play_clip_handle(entity, stored_animation.0.clone_weak());
        commands.entity(entity).remove::<StoredAnimation>();
    }
}
