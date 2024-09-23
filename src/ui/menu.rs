use bevy::{prelude::*, window::WindowFocused};
pub use leafwing_input_manager::prelude::*;

use crate::prelude::*;

// List of user actions associated to menu/ui interaction
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum MenuAction {
    Exit,
}

impl MenuAction {
    fn make_input_map() -> InputMap<Self> {
        use MenuAction::*;

        let input_map = InputMap::new([(Exit, KeyCode::Escape)]);

        input_map
    }
}

pub(super) struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default())
            .init_resource::<ActionState<MenuAction>>()
            .insert_resource(MenuAction::make_input_map())
            .add_systems(Update, handle_menu_inputs.in_set(PostAssetLoadingSet))
            .add_systems(
                Update,
                pause_game_when_window_loses_focus.in_set(GameplaySet),
            );
    }
}

fn handle_menu_inputs(
    game_state: Res<State<GameState>>,
    menu_action_state: Res<ActionState<MenuAction>>,
    mut app_exit: EventWriter<AppExit>,
) {
    use MenuAction::*;

    match game_state.get() {
        _ if menu_action_state.just_pressed(&Exit) => {
            app_exit.send_default();
        },
        _ => {},
    }
}

fn pause_game_when_window_loses_focus(
    mut window_focused_events: EventReader<WindowFocused>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for event in window_focused_events.read() {
        if !event.focused {
            next_game_state.set(GameState::Paused);
            info!("Paused");
        }
    }
}
