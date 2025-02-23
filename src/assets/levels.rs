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
    PlayAudio,    // (sound name, text), non-blocking.
    TickCount,    // (tick count), Pair with sync? Set how long actions take?
    Sync,         // (), blocking, Blocks until other actions complete?
    Reset,        // (self), Reset floor switches to their off state.
    SetDoorTimer, // (countdown time), Shows door timer UI, non-blocking.
}
