use bevy::{ecs::system::SystemState, prelude::*, utils::HashMap};
use bevy_common_assets::ron::RonAssetPlugin;
use derive_new::new;
use serde::Deserialize;

use crate::prelude::*;

pub(super) struct BlueprintsPlugin;

impl Plugin for BlueprintsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Blueprint>::new(&["blueprint.ron"]))
            .observe(spawn_blueprint_with_matrix);
    }
}

#[derive(Event, new)]
pub struct SpawnBlueprint {
    #[new(into)]
    file_stem: String,
    matrix: Mat4,
}

/// Blueprint entity configuration.
#[derive(Asset, Debug, Deserialize, Resource, TypePath)]
pub struct Blueprint(pub Vec<BlueprintProp>);

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
            Res<Assets<Blueprint>>,
        )> = SystemState::new(world);
        let (asset_server, game_assets, blueprint_assets) =
            system_state.get_mut(world);
        let mut scenes: HashMap<String, Handle<Scene>> = HashMap::default();
        let mut animation_clips: HashMap<String, Handle<AnimationClip>> =
            HashMap::default();

        for (_, blueprint_handle) in &game_assets.blueprints {
            let Some(blueprint) = blueprint_assets.get(blueprint_handle) else {
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

fn spawn_blueprint_with_matrix(
    trigger: Trigger<SpawnBlueprint>,
    mut commands: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    blueprints: Res<Assets<Blueprint>>,
    game_assets: Res<GameAssets>,
    preloaded_blueprint_assets: Res<PreloadedBlueprintAssets>,
) {
    let SpawnBlueprint { file_stem, matrix } = trigger.event();
    let handle = game_assets.blueprints.get(file_stem.as_str()).unwrap();
    let blueprint = blueprints.get(handle).unwrap();
    let mut entity_commands = commands.spawn(ForStates::new([
        GameState::Paused,
        GameState::Gameplay,
        GameState::GameOver,
    ]));

    for property in &blueprint.0 {
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
                let mut graph = AnimationGraph::new();

                for (animation_name, animation_file_path) in clips {
                    let handle = preloaded_blueprint_assets
                        .animation_clips
                        .get(animation_file_path)
                        .unwrap()
                        .clone();
                    let node_index = graph.add_clip(handle, 1.0, graph.root);
                    loaded_clips.insert(animation_name.clone(), node_index);
                }

                let handle = graphs.add(graph);

                entity_commands.insert(AnimationsBundle {
                    animation_graph_handle: AnimationGraphHandle(handle),
                    animation_clips: AnimationClips(loaded_clips),
                });
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
