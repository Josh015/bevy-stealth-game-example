use crate::{assets::*, components::*};
use bevy::{ecs::prelude::*, prelude::*, utils::HashMap};
use spew::prelude::*;

pub(super) struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SpewPlugin::<Config, (Handle<ActorConfig>, Mat4)>::default(),
        ))
        .add_spawner((Config::Actor, spawn_actor_from_config_with_matrix));
    }
}

/// Entities that can be spawned from config files.
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
    asset_server: Res<AssetServer>,
) {
    let actor_config = actor_configs.get(handle).unwrap();
    let mut actor = commands.spawn_empty();

    for component_config in &actor_config.components {
        match component_config {
            ComponentConfig::Player => {
                actor.insert(PlayerBundle::default());
            },
            ComponentConfig::Guard => {
                actor.insert(GuardBundle::default());
            },
            ComponentConfig::SecurityCamera => {
                actor.insert(SecurityCameraBundle::default());
            },
            ComponentConfig::Pickup => {
                actor.insert(PickupBundle::default());
            },
            ComponentConfig::Weapon => {
                actor.insert(Weapon::default());
            },
            //Trigger {} // TODO: Probably want to have a sub-enum with
            // pre-allowed events?
            ComponentConfig::FloorSwitch => {
                actor.insert(FloorSwitchBundle::default());
            },
            ComponentConfig::Door => {
                actor.insert(DoorBundle::default());
            },
            ComponentConfig::Glass => {
                actor.insert(GlassBundle::default());
            },
            ComponentConfig::Movement {
                linear_speed,
                angular_speed,
            } => {
                actor.insert(Movement {
                    linear_speed: *linear_speed,
                    angular_speed: *angular_speed,
                });
            },
            ComponentConfig::Physics { radius } => {
                // TODO: Need a component for this one.
            },
            ComponentConfig::Footsteps { sound_wave } => {
                // TODO: Need a component for this one.
            },
            ComponentConfig::DropShadow => {
                actor.insert(DropShadow::default());
            },
            ComponentConfig::Vision => {
                // TODO: Implement setting the fields.
                actor.insert(Vision::default());
            },
            ComponentConfig::Hearing => {
                // TODO: Implement setting the fields.
                actor.insert(Hearing::default());
            },
            ComponentConfig::Stunnable => {
                actor.insert(Stunnable::default());
            },
            ComponentConfig::Barrier => {
                actor.insert(Barrier::default());
            },
            ComponentConfig::BlocksVision => {
                actor.insert(BlocksVision::default());
            },
            ComponentConfig::DeflectsSounds => {
                actor.insert(DeflectsSounds::default());
            },
            ComponentConfig::AnimationClips(clips) => {
                let mut loaded_clips = HashMap::default();

                for (k, v) in clips {
                    loaded_clips.insert(k.to_string(), asset_server.load(v));
                }

                actor.insert(AnimationClips(loaded_clips));
            },
            ComponentConfig::Scene(scene) => {
                actor.insert(SceneBundle {
                    scene: asset_server.load(scene),
                    transform: Transform::from_matrix(matrix),
                    ..default()
                });
            },
        }
    }
}
