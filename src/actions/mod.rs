use bevy::app::prelude::*;

pub mod action_sequence;
pub mod animation_action;
pub mod dialog_bubble_action;
pub mod move_to_action;
pub mod parallel_actions;
pub mod repeat_action;
pub mod repeat_sequence;
pub mod sound_action;
pub mod state_done_action;
pub mod turn_to_action;
pub mod wait_action;

pub(super) struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(wait_action::WaitActionPlugin);
    }
}
