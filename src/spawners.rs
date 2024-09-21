use crate::{assets::*, components::*, configs::*, game_state::*};
use bevy::{ecs::prelude::*, prelude::*, utils::HashMap};
use spew::prelude::*;

pub(super) struct SpawnersPlugin;

impl Plugin for SpawnersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SpewPlugin::<Config, (Handle<ActorConfig>, Mat4)>::default(),
        ))
        .add_spawner((Config::Actor, spawn_actor_from_config_with_matrix));
    }
}

/// Entities that can be spawned from config resources.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Config {
    Actor,
    Emote,
    Level,
    SoundWave,
}

fn spawn_actor_from_config_with_matrix(
    In((handle, matrix)): In<(Handle<ActorConfig>, Mat4)>,
    actor_configs: Res<Assets<ActorConfig>>,
    mut commands: Commands,
    preloaded_actor_assets: Res<PreloadedActorAssets>,
) {
    let actor_config = actor_configs.get(&handle).unwrap();
    let mut entity_commands = commands
        .spawn(ForStates(vec![GameState::Gameplay, GameState::GameOver]));

    for component_config in &actor_config.0 {
        match component_config {
            ComponentConfig::Player => {
                entity_commands.insert(PlayerBundle::default());
            },
            ComponentConfig::Guard => {
                entity_commands.insert(GuardBundle::default());
            },
            ComponentConfig::SecurityCamera => {
                entity_commands.insert(SecurityCameraBundle::default());
            },
            ComponentConfig::Pickup => {
                entity_commands.insert(PickupBundle::default());
            },
            ComponentConfig::Weapon => {
                entity_commands.insert(Weapon::default());
            },
            //Trigger {} // TODO: Probably want to have a sub-enum with
            // pre-allowed events?
            ComponentConfig::FloorSwitch => {
                entity_commands.insert(FloorSwitchBundle::default());
            },
            ComponentConfig::Door => {
                entity_commands.insert(DoorBundle::default());
            },
            ComponentConfig::Glass => {
                entity_commands.insert(GlassBundle::default());
            },
            ComponentConfig::Speed {
                linear_speed,
                angular_speed,
            } => {
                entity_commands.insert(SpeedBundle {
                    linear_speed: LinearSpeed(*linear_speed),
                    angular_speed: AngularSpeed(*angular_speed),
                    ..default()
                });
            },
            ComponentConfig::Physics { radius } => {
                // TODO: Need a component for this one.
            },
            ComponentConfig::Footsteps { sound_wave } => {
                // TODO: Need a component for this one.
            },
            ComponentConfig::DropShadow => {
                entity_commands.insert(DropShadow::default());
            },
            ComponentConfig::Vision => {
                // TODO: Implement setting the fields.
                entity_commands.insert(Vision::default());
            },
            ComponentConfig::Hearing => {
                // TODO: Implement setting the fields.
                entity_commands.insert(Hearing::default());
            },
            ComponentConfig::Stunnable => {
                entity_commands.insert(Stunnable::default());
            },
            ComponentConfig::Barrier => {
                entity_commands.insert(Barrier::default());
            },
            ComponentConfig::BlocksVision => {
                entity_commands.insert(BlocksVision::default());
            },
            ComponentConfig::DeflectsSounds => {
                entity_commands.insert(DeflectsSounds::default());
            },
            ComponentConfig::AnimationClips(clips) => {
                let mut loaded_clips = HashMap::default();

                for (k, v) in clips {
                    loaded_clips.insert(
                        k.to_string(),
                        preloaded_actor_assets
                            .animation_clips
                            .get(v)
                            .unwrap()
                            .clone(),
                    );
                }

                entity_commands.insert(AnimationClips(loaded_clips));
            },
            ComponentConfig::Scene(scene) => {
                entity_commands.insert(SceneBundle {
                    scene: preloaded_actor_assets
                        .scenes
                        .get(scene)
                        .unwrap()
                        .clone(),
                    transform: Transform::from_matrix(matrix),
                    ..default()
                });
            },
        }
    }
}
