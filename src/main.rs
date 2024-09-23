#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod actions;
mod assets;
mod components;
mod events;
mod game_state;
mod spawners;
mod system_params;
mod ui;
mod util;

use actions::*;
use assets::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::{
    asset::LoadState,
    color::palettes,
    gltf::{Gltf, GltfMesh},
    math::Vec3Swizzles,
    pbr::NotShadowCaster,
    prelude::*,
    time::common_conditions::on_timer,
    window::PresentMode,
};
use bevy_sequential_actions::*;
use bevy_tweening::*;
use components::*;
use events::*;
use game_state::*;
use polyanya::Triangulation;
use rand::Rng;
use seldom_state::prelude::*;
use spawners::*;
use std::{f32::consts::FRAC_PI_2, time::Duration};
use ui::*;
use vleue_navigator::{
    prelude::{
        NavMeshBundle, NavMeshSettings, NavMeshUpdateMode, NavmeshUpdaterPlugin,
    },
    NavMesh, NavMeshDebug, VleueNavigatorPlugin,
};

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::srgb(0., 0., 0.01)))
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Stealth Sound Game".to_owned(),
                        present_mode: PresentMode::AutoVsync,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    ..Default::default()
                }),
            PhysicsPlugins::default().with_length_unit(20.0),
            VleueNavigatorPlugin,
            NavmeshUpdaterPlugin::<Collider, Obstacle>::default(),
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
            UiPlugin,
        ))
        .add_systems(OnEnter(GameState::StartMenu), setup)
        .add_systems(
            Update,
            check_textures.run_if(in_state(GameState::StartMenu)),
        )
        .add_systems(OnExit(GameState::StartMenu), setup_scene)
    // .add_systems(
    //     Update,
    //     refresh_path.run_if(on_timer(Duration::from_secs_f32(0.1))),
    // );
    .add_systems(
        Update,
        (
            target_activity,
            display_navigator_path,
            // despawn_obstacles
        ).run_if(in_state(GameState::Gameplay))
    );
    // .add_systems(
    //     Update,
    //     spawn_obstacles.run_if(on_timer(Duration::from_secs_f32(0.5))),
    // );

    let mut config_store = app
        .world_mut()
        .get_resource_mut::<GizmoConfigStore>()
        .unwrap();
    for (_, config, _) in config_store.iter_mut() {
        config.depth_bias = -1.0;
    }
    app.run();
}

const PICKUP_HALF_SIZE: f32 = 2.5;

#[derive(Component)]
struct Obstacle(Timer);

#[derive(Resource, Default, Deref)]
struct GltfHandle(Handle<Gltf>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .insert_resource(GltfHandle(asset_server.load("models/navmesh.glb")));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::default()
            .looking_at(Vec3::new(-1.0, -2.5, -1.5), Vec3::Y),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        camera: Camera {
            #[cfg(not(target_arch = "wasm32"))]
            hdr: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 70.0, 5.0)
            .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn check_textures(
    mut next_state: ResMut<NextState<GameState>>,
    gltf: ResMut<GltfHandle>,
    asset_server: Res<AssetServer>,
) {
    if let Some(LoadState::Loaded) = asset_server.get_load_state(gltf.id()) {
        next_state.set(GameState::Gameplay);
    }
}

fn setup_scene(
    mut commands: Commands,
    gltf: Res<GltfHandle>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut material: StandardMaterial =
        Color::Srgba(palettes::css::ALICE_BLUE).into();
    material.perceptual_roughness = 1.0;
    let ground_material = materials.add(material);
    if let Some(gltf) = gltfs.get(gltf.id()) {
        let mesh = gltf_meshes.get(&gltf.named_meshes["obstacles"]).unwrap();
        let mut material: StandardMaterial =
            Color::Srgba(palettes::css::GRAY).into();
        material.perceptual_roughness = 1.0;
        commands.spawn((
            PbrBundle {
                mesh: mesh.primitives[0].mesh.clone(),
                material: materials.add(material),
                ..default()
            },
            RigidBody::Static,
            ColliderConstructor::TrimeshFromMesh,
        ));

        let mesh = gltf_meshes.get(&gltf.named_meshes["plane"]).unwrap();
        commands.spawn(PbrBundle {
            mesh: mesh.primitives[0].mesh.clone(),
            transform: Transform::from_xyz(0.0, 0.01, 0.0),
            material: ground_material.clone(),
            ..default()
        });
    }

    if let Some(gltf) = gltfs.get(gltf.id()) {
        {
            let navmesh = vleue_navigator::NavMesh::from_bevy_mesh(
                meshes
                    .get(
                        &gltf_meshes
                            .get(&gltf.named_meshes["navmesh"])
                            .unwrap()
                            .primitives[0]
                            .mesh,
                    )
                    .unwrap(),
            );

            let mut material: StandardMaterial =
                Color::Srgba(palettes::css::ANTIQUE_WHITE).into();
            material.unlit = true;

            commands.spawn((
                NavMeshBundle {
                    settings: NavMeshSettings {
                        fixed: Triangulation::from_mesh(
                            navmesh.get().as_ref(),
                            0,
                        ),
                        build_timeout: Some(5.0),
                        upward_shift: 0.5,
                        ..default()
                    },
                    transform: Transform::from_rotation(Quat::from_rotation_x(
                        FRAC_PI_2,
                    )),
                    update_mode: NavMeshUpdateMode::Direct,
                    ..NavMeshBundle::with_default_id()
                },
                NavMeshDebug(palettes::tailwind::RED_400.into()),
            ));
        }

        commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Capsule3d { ..default() })),
                    material: materials.add(StandardMaterial {
                        base_color: palettes::css::BLUE.into(),
                        emissive: (palettes::css::BLUE * 5.0).into(),
                        ..default()
                    }),
                    transform: Transform::from_xyz(-1.0, 0.0, -2.0),
                    ..Default::default()
                },
                NotShadowCaster,
            ))
            .with_children(|object| {
                object.spawn(PointLightBundle {
                    point_light: PointLight {
                        color: palettes::css::BLUE.into(),
                        range: 500.0,
                        intensity: 100000.0,
                        shadows_enabled: true,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 1.2, 0.0),
                    ..default()
                });
            });
    }

    commands.trigger(SpawnActor::WithTransform(
        "birthday_cake_pickup.actor".to_string(),
        Mat4::from_scale_rotation_translation(
            Vec3::splat(PICKUP_HALF_SIZE),
            Quat::IDENTITY,
            Vec3::new(
                PICKUP_HALF_SIZE + 0.25,
                PICKUP_HALF_SIZE + 0.1,
                -PICKUP_HALF_SIZE,
            ),
        ),
    ));

    commands.trigger(SpawnActor::WithTransform(
        "guard_dog.actor".to_string(),
        Mat4::from_scale_rotation_translation(
            Vec3::splat(0.025),
            Quat::IDENTITY,
            Vec3::ZERO,
        ),
    ));

    commands.trigger(SpawnActor::WithTransform(
        "player.actor".to_string(),
        Mat4::from_scale_rotation_translation(
            Vec3::splat(0.025),
            Quat::IDENTITY,
            Vec3::new(0.25, 0.0, 0.0),
        ),
    ));
}

// fn spawn_obstacles(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let size = rand::thread_rng().gen_range(1.5..2.0);
//     commands.spawn((
//         PbrBundle {
//             mesh: meshes.add(Cuboid::new(size, size, size)),
//             material: materials.add(Color::srgb(0.2, 0.7, 0.9)),
//             transform: Transform::from_xyz(
//                 rand::thread_rng().gen_range(-50.0..50.0),
//                 10.0,
//                 rand::thread_rng().gen_range(-25.0..25.0),
//             )
//             .looking_to(
//                 Vec3::new(
//                     rand::thread_rng().gen_range(-1.0..1.0),
//                     rand::thread_rng().gen_range(-1.0..1.0),
//                     rand::thread_rng().gen_range(-1.0..1.0),
//                 )
//                 .normalize(),
//                 Vec3::Y,
//             ),
//             ..default()
//         },
//         RigidBody::Dynamic,
//         Collider::cuboid(size, size, size),
//         Obstacle(Timer::from_seconds(30.0, TimerMode::Once)),
//     ));
// }

// fn despawn_obstacles(
//     mut commands: Commands,
//     mut obstacles: Query<(
//         Entity,
//         &mut Obstacle,
//         &mut LinearVelocity,
//         &mut Transform,
//     )>,
//     time: Res<Time>,
// ) {
//     for (entity, mut timer, mut linear_velocity, mut transform) in
//         &mut obstacles
//     {
//         if timer.0.tick(time.delta()).just_finished() {
//             linear_velocity.0 = Vec3::new(0.0, 50.0, 0.0);
//         }
//         if timer.0.finished() {
//             transform.scale *= 0.98;
//             if transform.scale.x < 0.01 {
//                 commands.entity(entity).despawn();
//             }
//         }
//     }
// }

fn target_activity(
    target: Query<&Children, With<Target>>,
    mut point_light: Query<&mut PointLight>,
    time: Res<Time>,
) {
    for children in &target {
        point_light.get_mut(children[0]).unwrap().intensity =
            (time.elapsed_seconds() * 10.0).sin().abs() * 100000.0;
    }
}

fn display_navigator_path(navigator: Query<&Path>, mut gizmos: Gizmos) {
    for path in &navigator {
        if !path.0.is_empty() {
            gizmos.linestrip(
                path.0.iter().map(|xz| Vec3::new(xz.x, 0.2, xz.z)),
                palettes::tailwind::TEAL_400,
            );
        }
    }
}

// fn refresh_path(
//     mut object_query: Query<(&Transform, &mut Path)>,
//     target: Query<&Transform, With<Target>>,
//     navmeshes: Res<Assets<NavMesh>>,
// ) {
//     for (transform, mut path) in &mut object_query {
//         let navmesh = navmeshes.get(&Handle::default()).unwrap();
//         let Some(new_path) = navmesh.transformed_path(
//             transform.translation,
//             target.single().translation,
//         ) else {
//             break;
//         };
//         if let Some((first, remaining)) = new_path.path.split_first() {
//             let mut remaining = remaining.to_vec();
//             remaining.reverse();
//             *path = Path {
//                 current: *first,
//                 next: remaining,
//             };
//         }
//     }
// }
