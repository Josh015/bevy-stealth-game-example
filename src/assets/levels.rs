use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub(super) struct LevelsPlugin;

impl Plugin for LevelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<LevelConfig>::new(&["level.ron"]));
    }
}

#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct LevelConfig {
    title: String,
    //map
    entities: Vec<EntityLevelConfig>,
    on_start: ScriptCommands,
    on_alert: ScriptCommands,
    on_escape: ScriptCommands,
    on_end_level: ScriptCommands,
    //glass
    //grates
}

#[derive(Clone, Debug, Deserialize)]
pub struct EntityLevelConfig {
    id: String,
    blueprint: String,
    //transform,
    on_idle: ScriptCommands,
    on_trigger: ScriptCommands,
}

pub type ScriptCommands = Option<Vec<ScriptCommandConfig>>;

#[derive(Clone, Debug, Deserialize)]
pub enum ScriptCommandConfig {
    OpenDoor,     // (door ID), non-blocking.
    CloseDoor,    // (door ID), non-blocking.
    ToggleDoor,   // (door ID), non-blocking.
    MoveTo,       // (waypoint ID), blocking.
    LookAt,       // (waypoint ID), blocking.
    Wait,         // (delay), blocking.
    Repeat,       // (), non-blocking, Repeats the script indefinitely.
    PlayVideo,    // (file name), non-blocking. Interrupt via mouse click.
    PlayAudio,    // (file name, text), non-blocking.
    TickCount,    // (tick count), non-blocking, Set interval for syncs.
    Sync,         // (tick count), blocking, Blocks until tick count reached.
    Reset,        // (self), Reset floor switches to their off state.
    SetDoorTimer, // (countdown time), Shows door timer UI, non-blocking.
    SetGlobal,    // (string, string), Set a global variable to access later.
    Achievement,  // (string), Triggers an achievement.
    NextLevel,    // (), Go to next level.
}

// TODO: Sync system is based on current time in integer second intervals.
// It's needed for synchronizing enemy patrol scripts.
