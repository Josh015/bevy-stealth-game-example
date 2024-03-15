#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod actions;
pub mod common;
pub mod components;
pub mod entities;
pub mod events;
pub mod game;
pub mod states;
pub mod ui;

use actions::{
    face_direction_action::FaceDirectionAction, move_to_action::MoveToAction,
};
use bevy_sequential_actions::*;
use bevy_tweening::*;
use components::player::*;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

fn main() {
    App::new()
        .add_plugins(
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
        )
        .add_plugins(SequentialActionsPlugin)
        .add_plugins(TweeningPlugin)
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::rgba(0.7, 0.9, 1.0, 1.0)))
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(Startup, tinkering_zone_system)
        .run();
}

// TODO: Remove this after testing.
#[allow(dead_code)]
fn tinkering_zone_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            illuminance: 2_500.0,
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

    // ---- Sphere with a nose ----
    let sphere_radius = 0.0625;
    let sphere_height = sphere_radius + 0.01;
    let cylinder_radius = 0.0625;
    let cylinder = meshes.add(Cylinder {
        radius: 0.5 * cylinder_radius,
        half_height: cylinder_radius,
    });
    let movement_range = 0.5;

    let agent = commands
        .spawn((
            Player,
            ActionsBundle::new(),
            PbrBundle {
                mesh: meshes.add(Sphere {
                    radius: sphere_radius,
                }),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, sphere_height, 0.0),
                ..default()
            },
            // TODO: States:
            // Spawn sphere with vision cone.
            //  State Spin back and forth with delays. Switch to Move when done.
            //    LoopAction
            //  State move back and forth with delays. Switch to Spin when done.
            //    RepeatAction
        ))
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: cylinder,
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    ..default()
                }),
                transform: Transform::from_matrix(
                    Mat4::from_translation(Vec3::new(0.0, 0.0, -sphere_radius))
                        * Mat4::from_rotation_x(std::f32::consts::FRAC_PI_2),
                ),
                ..default()
            });
        })
        .id();

    // commands
    //     .actions(agent)
    //     .add(RotateToFaceDirectionAction::new(
    //         Direction3d::new_unchecked(Vec3::X),
    //     ));

    commands.actions(agent).add_many(actions![
        FaceDirectionAction::new(Direction3d::X),
        FaceDirectionAction::new(Direction3d::Z),
        FaceDirectionAction::new(Direction3d::NEG_X),
        FaceDirectionAction::new(Direction3d::NEG_Z),
        MoveToAction::new(Vec3::new(
            movement_range,
            sphere_height,
            movement_range
        )),
        MoveToAction::new(Vec3::new(
            movement_range,
            sphere_height,
            -movement_range
        )),
        MoveToAction::new(Vec3::new(
            -movement_range,
            sphere_height,
            -movement_range
        )),
        MoveToAction::new(Vec3::new(
            -movement_range,
            sphere_height,
            movement_range
        )),
        MoveToAction::new(Vec3::new(0.0, sphere_height, 0.0)),
        FaceDirectionAction::new(Direction3d::NEG_Z),
    ]);
}
