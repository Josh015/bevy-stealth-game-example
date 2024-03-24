use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{components::*, system_sets::*};

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
    mut query: Query<
        (&Transform, &mut Mover, &ActionState<PlayerAction>),
        With<Player>,
    >,
) {
    let (player_transform, mut mover, action_state) = query.single_mut();

    if action_state.pressed(&PlayerAction::Move) {
        let clamped_axis = action_state
            .clamped_axis_pair(&PlayerAction::Move)
            .unwrap()
            .xy();
        let move_direction = clamped_axis.extend(0.0).xzy()
            * Vec3::new(1.0, 1.0, -1.0).normalize_or_zero();

        mover.move_to = Some(crate::MoveTo::Destination(
            player_transform.translation
                + move_direction * mover.linear_speed * time.delta_seconds(),
        ));
    }
}
