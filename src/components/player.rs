use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::system_sets::GameplaySet;

use super::{Destination, MoveSpeed};

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, control_player.in_set(GameplaySet));
    }
}

/// Entity that can be targeted by enemy units.
#[derive(Clone, Component, Debug, Default)]
#[require(Transform, MoveSpeed, ActionState<PlayerAction>, InputMap<PlayerAction>(player_input_map))]
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum PlayerAction {
    Move,
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Move => InputControlKind::DualAxis,
        }
    }
}

fn player_input_map() -> InputMap<PlayerAction> {
    use PlayerAction::*;

    InputMap::default()
        .with_dual_axis(Move, GamepadStick::LEFT)
        .with_dual_axis(Move, VirtualDPad::arrow_keys())
        .with_dual_axis(Move, VirtualDPad::wasd())
        .with_dual_axis(Move, VirtualDPad::numpad())
}

fn control_player(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<
        (Entity, &Transform, &MoveSpeed, &ActionState<PlayerAction>),
        With<Player>,
    >,
) {
    let (entity, player_transform, move_speed, action_state) = query.single();

    let clamped_axis = action_state.clamped_axis_pair(&PlayerAction::Move).xy();
    let move_direction =
        Vec3::new(clamped_axis.x, 0.0, -clamped_axis.y).normalize_or_zero();

    // Prevent snapping to facing forward on -Z when releasing stick.
    if move_direction != Vec3::ZERO {
        commands.entity(entity).insert(Destination(
            player_transform.translation
                + move_direction * move_speed.0 * time.delta_secs(),
        ));
    }
}
