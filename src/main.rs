#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod actions;
pub mod components;
pub mod events;
pub mod game;
pub mod ui;
pub mod util;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_sequential_actions::*;
use bevy_tweening::*;
use components::*;
use game::{ActorConfig, Config, GameAssets, GameState};
use seldom_state::prelude::*;
use spew::prelude::SpawnEvent;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Stealth Sound Game".to_owned(),
                        resolution: WindowResolution::new(800.0, 800.0),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    ..Default::default()
                }),
            SequentialActionsPlugin,
            StateMachinePlugin,
            TweeningPlugin,
        ))
        .add_plugins((
            actions::ActionsPlugin,
            components::ComponentsPlugin,
            events::EventsPlugin,
            game::GamePlugin,
        ))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgba(0.7, 0.9, 1.0, 1.0)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(OnExit(GameState::Loading), tinkering_zone_system)
        .run();
}

const PICKUP_HALF_SIZE: f32 = 0.05;

// TODO: Remove this after testing.
#[allow(dead_code)]
fn tinkering_zone_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
    mut spawn_events: EventWriter<
        SpawnEvent<Config, (Handle<ActorConfig>, Vec3)>,
    >,
) {
    // ---- Camera ----
    // TODO: Follow player effect.
    commands.spawn(Camera3dBundle {
        transform: Transform::looking_at(
            Transform::from_translation(Vec3::new(0.0, 0.5, 2.0)),
            Vec3::ZERO,
            Vec3::Y,
        ),
        ..default()
    });

    // ---- Environment Lighting ----
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 180.0,
    });
    let light_transform = Mat4::from_euler(
        EulerRot::ZYX,
        0.0,
        std::f32::consts::FRAC_PI_4,
        -std::f32::consts::FRAC_PI_4,
    );
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 2_400.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_matrix(light_transform),
        ..default()
    });

    // ---- Scene ----
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,
            ..default()
        }),
        transform: Transform::IDENTITY,
        ..default()
    });

    // ---- Pickup ----
    commands.spawn((
        PickupBundle::default(),
        PbrBundle {
            mesh: meshes.add(Cuboid {
                half_size: Vec3::splat(PICKUP_HALF_SIZE),
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::YELLOW,
                ..default()
            }),
            transform: Transform::from_xyz(0.25, PICKUP_HALF_SIZE + 0.1, 0.0),
            ..default()
        },
    ));

    // ---- Actor ----
    let guard_dog = game_assets
        .actors
        .get("config/actors/guards/guard_dog.actor.yaml")
        .unwrap();
    spawn_events.send(SpawnEvent::with_data(
        Config::Actor,
        (guard_dog.clone_weak(), Vec3::ZERO),
    ));
}
