#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod actions;
mod assets;
mod components;
mod events;
mod game_state;
mod spawners;
mod system_params;
mod system_sets;
mod ui;
mod util;

use actions::*;
use assets::*;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use bevy_sequential_actions::*;
use bevy_tweening::*;
use components::*;
use events::*;
use game_state::*;
use seldom_state::prelude::*;
use spawners::*;
use system_sets::*;
use ui::*;

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
            ActionsPlugin,
            AssetsPlugin,
            ComponentsPlugin,
            GameStatePlugin,
            EventsPlugin,
            SpawnersPlugin,
            SystemSetsPlugin,
            UiPlugin,
        ))
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::srgba(0.7, 0.9, 1.0, 1.0)))
        .add_systems(
            OnEnter(game_state::GameState::Gameplay),
            tinkering_zone_system,
        )
        .run();
}

const PICKUP_HALF_SIZE: f32 = 0.25;

// TODO: Remove this after testing.
#[allow(dead_code)]
fn tinkering_zone_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
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
            base_color: Srgba::GREEN.into(),
            ..default()
        }),
        transform: Transform::IDENTITY,
        ..default()
    });

    let birthday_cake = game_assets
        .actors
        .get("birthday_cake_pickup.actor")
        .unwrap();
    commands.trigger(SpawnActor::WithTransform(
        birthday_cake.clone_weak(),
        Mat4::from_scale_rotation_translation(
            Vec3::splat(PICKUP_HALF_SIZE),
            Quat::IDENTITY,
            Vec3::new(0.25, PICKUP_HALF_SIZE + 0.1, 0.0),
        ),
    ));

    let guard_dog = game_assets.actors.get("guard_dog.actor").unwrap();
    commands.trigger(SpawnActor::WithTransform(
        guard_dog.clone_weak(),
        Mat4::from_scale_rotation_translation(
            Vec3::splat(0.0025),
            Quat::IDENTITY,
            Vec3::ZERO,
        ),
    ));

    let player = game_assets.actors.get("player.actor").unwrap();
    commands.trigger(SpawnActor::WithTransform(
        player.clone_weak(),
        Mat4::from_scale_rotation_translation(
            Vec3::splat(0.0025),
            Quat::IDENTITY,
            Vec3::new(0.25, 0.0, 0.0),
        ),
    ));
}
