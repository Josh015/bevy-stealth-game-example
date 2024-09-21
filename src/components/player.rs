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
            input_manager_bundle: InputManagerBundle::<PlayerAction>::with_map(
                PlayerAction::default_input_map(),
            ),
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum PlayerAction {
    Move,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        use PlayerAction::*;

        let input_map =
            InputMap::default().with_dual_axis(Move, GamepadStick::LEFT);
        // .with_dual_axis(Move, KeyboardVirtualDPad::ARROW_KEYS)
        // .with_dual_axis(Move, KeyboardVirtualDPad::WASD)
        // .with_dual_axis(Move, KeyboardVirtualDPad::NUMPAD);
        input_map
    }
}

impl Actionlike for PlayerAction {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            PlayerAction::Move => InputControlKind::DualAxis,
        }
    }
}

fn control_player(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<
        (Entity, &Transform, &LinearSpeed, &ActionState<PlayerAction>),
        With<Player>,
    >,
) {
    let (entity, player_transform, linear_speed, action_state) = query.single();

    let clamped_axis = action_state.clamped_axis_pair(&PlayerAction::Move).xy();
    let move_direction =
        Vec3::new(clamped_axis.x, 0.0, -clamped_axis.y).normalize_or_zero();

    // Prevent snapping to facing forward on -Z when releasing stick.
    if move_direction != Vec3::ZERO {
        commands.entity(entity).insert(MoveTo::Destination(
            player_transform.translation
                + move_direction * linear_speed.0 * time.delta_seconds(),
        ));
    }
}
