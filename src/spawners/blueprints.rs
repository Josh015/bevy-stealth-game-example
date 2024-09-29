use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap};
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

use crate::prelude::*;

pub(super) struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<BlueprintConfig>::new(&[
            "blueprint.ron",
        ]))
        .observe(spawn_blueprint_from_config_with_matrix);
    }
}

#[derive(Event)]
pub enum SpawnBlueprint {
    WithTransform(String, Mat4),
}

/// Blueprint entity configuration.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct BlueprintConfig(pub Vec<BlueprintProp>);

/// Properties for configuring blueprint entities.
///
/// Don't necessarily map 1:1 to the entity's components.
#[derive(Clone, Debug, Deserialize)]
pub enum BlueprintProp {
    Player,
    Guard,
    SecurityCamera,
    Pickup,
    Weapon,
    //Trigger {} // Probably want to have a sub-enum with pre-allowed events?
    FloorSwitch,
    Door,
    Glass,
    Speed {
        linear_speed: f32,
        angular_speed: f32,
    },
    Physics {
        radius: f32,
    },
    Footsteps {
        sound_wave: String,
    },
    DropShadow,
    Vision,
    Hearing,
    Stunnable,
    Barrier,
    BlocksVision,
    DeflectsSounds,
    Scene(String),
    AnimationClips(HashMap<String, String>),
}

/// Assets that need to be loaded in advance of spawning entities.
#[derive(Debug, Resource)]
pub struct PreloadedBlueprintAssets {
    pub scenes: HashMap<String, Handle<Scene>>,
    pub animation_clips: HashMap<String, Handle<AnimationClip>>,
}

impl FromWorld for PreloadedBlueprintAssets {
    fn from_world(world: &mut World) -> Self {
        let mut system_state: SystemState<(
            Res<AssetServer>,
            Res<GameAssets>,
            Res<Assets<BlueprintConfig>>,
        )> = SystemState::new(world);
        let (asset_server, game_assets, blueprint_config_assets) =
            system_state.get_mut(world);
        let mut scenes: HashMap<String, Handle<Scene>> = HashMap::default();
        let mut animation_clips: HashMap<String, Handle<AnimationClip>> =
            HashMap::default();

        for (_, blueprint) in &game_assets.blueprints {
            let Some(blueprint) = blueprint_config_assets.get(blueprint) else {
                continue;
            };

            // Preload all referenced assets in entity configs.
            for property in &blueprint.0 {
                match property {
                    BlueprintProp::Scene(path) => {
                        if scenes.get(path).is_none() {
                            scenes.insert(
                                path.to_string(),
                                asset_server.load(path),
                            );
                        }
                    },
                    BlueprintProp::AnimationClips(mappings) => {
                        for (_, path) in mappings {
                            if animation_clips.get(path).is_none() {
                                animation_clips.insert(
                                    path.to_string(),
                                    asset_server.load(path),
                                );
                            }
                        }
                    },
                    _ => {},
                }
            }
        }

        Self {
            scenes,
            animation_clips,
        }
    }
}

fn spawn_blueprint_from_config_with_matrix(
    trigger: Trigger<SpawnBlueprint>,
    blueprint_configs: Res<Assets<BlueprintConfig>>,
    game_assets: Res<GameAssets>,
    mut commands: Commands,
    preloaded_blueprint_assets: Res<PreloadedBlueprintAssets>,
) {
    let (filename, matrix) = match trigger.event() {
        SpawnBlueprint::WithTransform(filename, matrix) => (filename, matrix),
    };

    let handle = game_assets.blueprints.get(filename.as_str()).unwrap();
    let blueprint_config = blueprint_configs.get(handle).unwrap();
    let mut entity_commands = commands.spawn(ForStates::new([
        GameState::Paused,
        GameState::Gameplay,
        GameState::GameOver,
    ]));

    for property in &blueprint_config.0 {
        match property {
            BlueprintProp::Player => {
                entity_commands.insert(PlayerBundle::default());
            },
            BlueprintProp::Guard => {
                entity_commands.insert(GuardBundle::with_starting_location(
                    Transform::from_matrix(*matrix),
                ));
            },
            BlueprintProp::SecurityCamera => {
                entity_commands.insert(SecurityCameraBundle::default());
            },
            BlueprintProp::Pickup => {
                entity_commands.insert(PickupBundle::default());
            },
            BlueprintProp::Weapon => {
                entity_commands.insert(Weapon::default());
            },
            //Trigger {} // TODO: Probably want to have a sub-enum with
            // pre-allowed events?
            BlueprintProp::FloorSwitch => {
                entity_commands.insert(FloorSwitchBundle::default());
            },
            BlueprintProp::Door => {
                entity_commands.insert(DoorBundle::default());
            },
            BlueprintProp::Glass => {
                entity_commands.insert(GlassBundle::default());
            },
            BlueprintProp::Speed {
                linear_speed,
                angular_speed,
            } => {
                entity_commands.insert(SpeedBundle {
                    linear_speed: LinearSpeed(*linear_speed),
                    angular_speed: AngularSpeed(*angular_speed),
                    ..default()
                });
            },
            BlueprintProp::Physics { radius } => {
                // TODO: Need a component for this one.
            },
            BlueprintProp::Footsteps { sound_wave } => {
                let sound_wave_handle =
                    game_assets.sound_waves.get(sound_wave.as_str()).unwrap();

                entity_commands.insert(FootstepsBundle {
                    footsteps: Footsteps {
                        sound_wave: sound_wave_handle.clone(),
                    },
                });
            },
            BlueprintProp::DropShadow => {
                entity_commands.insert(DropShadow::default());
            },
            BlueprintProp::Vision => {
                // TODO: Implement setting the fields.
                entity_commands.insert(Vision::default());
            },
            BlueprintProp::Hearing => {
                // TODO: Implement setting the fields.
                entity_commands.insert(Hearing::default());
            },
            BlueprintProp::Stunnable => {
                entity_commands.insert(Stunnable::default());
            },
            BlueprintProp::Barrier => {
                entity_commands.insert(Barrier::default());
            },
            BlueprintProp::BlocksVision => {
                entity_commands.insert(BlocksVision::default());
            },
            BlueprintProp::DeflectsSounds => {
                entity_commands.insert(DeflectsSounds::default());
            },
            BlueprintProp::AnimationClips(clips) => {
                let mut loaded_clips = HashMap::default();

                for (k, v) in clips {
                    loaded_clips.insert(
                        k.to_string(),
                        preloaded_blueprint_assets
                            .animation_clips
                            .get(v)
                            .unwrap()
                            .clone(),
                    );
                }

                entity_commands.insert(AnimationClips(loaded_clips));
            },
            BlueprintProp::Scene(scene) => {
                entity_commands.insert(SceneBundle {
                    scene: preloaded_blueprint_assets
                        .scenes
                        .get(scene)
                        .unwrap()
                        .clone(),
                    transform: Transform::from_matrix(*matrix),
                    ..default()
                });
            },
        }
    }
}
