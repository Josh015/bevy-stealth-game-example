use bevy::app::prelude::*;

mod action_sequence;
mod animation_action;
mod dialog_bubble_action;
mod face_direction_action;
mod move_to_action;
mod parallel_actions;
mod repeat_action;
mod repeat_sequence;
mod sound_action;
mod state_done_action;
mod wait_action;

pub use action_sequence::*;
pub use animation_action::*;
pub use dialog_bubble_action::*;
pub use face_direction_action::*;
pub use move_to_action::*;
pub use parallel_actions::*;
pub use repeat_action::*;
pub use repeat_sequence::*;
pub use sound_action::*;
pub use state_done_action::*;
pub use wait_action::*;

pub(super) struct ActionsPlugin;

impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FaceDirectionPlugin,
            MoveToActionPlugin,
            WaitActionPlugin,
        ));
    }
}
