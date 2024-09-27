use bevy::{color::palettes, prelude::*};
pub use leafwing_input_manager::prelude::*;
use vleue_navigator::{prelude::*, NavMeshDebug};

use crate::prelude::*;

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<DebugUiAction>::default())
            .init_resource::<ActionState<DebugUiAction>>()
            .insert_resource(DebugUiAction::make_input_map())
            .init_resource::<DebugUiToggles>()
            .add_systems(Update, handle_debug_inputs.in_set(GameplaySet))
            .add_systems(
                Update,
                debug_display_routes.in_set(GameplaySet).run_if(
                    |debug_ui_toggles: Res<DebugUiToggles>| {
                        debug_ui_toggles.route
                    },
                ),
            );
    }
}

// List of user actions associated to debug UI.
#[derive(Actionlike, Clone, Copy, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum DebugUiAction {
    Route,
    NavMesh,
}

impl DebugUiAction {
    fn make_input_map() -> InputMap<Self> {
        use DebugUiAction::*;

        let input_map =
            InputMap::new([(Route, KeyCode::F1), (NavMesh, KeyCode::F2)]);

        input_map
    }
}

#[derive(Default, Resource)]
pub struct DebugUiToggles {
    pub route: bool,
    pub nav_mesh: bool,
}

fn handle_debug_inputs(
    menu_action_state: Res<ActionState<DebugUiAction>>,
    mut debug_ui_toggles: ResMut<DebugUiToggles>,
    mut commands: Commands,
    nav_mesh_query: Query<Entity, With<NavMeshSettings>>,
) {
    use DebugUiAction::*;

    if menu_action_state.just_pressed(&Route) {
        debug_ui_toggles.route = !debug_ui_toggles.route;
    } else if menu_action_state.just_pressed(&NavMesh) {
        debug_ui_toggles.nav_mesh = !debug_ui_toggles.nav_mesh;

        for entity in &nav_mesh_query {
            if debug_ui_toggles.nav_mesh {
                commands
                    .entity(entity)
                    .insert(NavMeshDebug(palettes::tailwind::RED_400.into()));
            } else {
                commands.entity(entity).remove::<NavMeshDebug>();
            }
        }
    }
}

fn debug_display_routes(
    navigator: Query<(&Route, &Transform)>,
    mut gizmos: Gizmos,
) {
    for (path, transform) in &navigator {
        let mut to_display = path.remaining.clone();
        to_display.push(path.next);
        to_display.push(transform.translation.xz().extend(0.2).xzy());

        if !to_display.is_empty() {
            gizmos.linestrip(
                to_display.iter().map(|xz| Vec3::new(xz.x, 0.2, xz.z)),
                palettes::tailwind::TEAL_400,
            );
        }
    }
}
