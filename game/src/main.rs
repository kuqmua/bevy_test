#![allow(
    clippy::needless_pass_by_value,
    clippy::float_arithmetic,
    clippy::arithmetic_side_effects,
    clippy::cast_precision_loss,
    clippy::as_conversions,
    clippy::single_call_fn
)]
use bevy::prelude::*;
const PLAYER_SPEED: f32 = 5.0;
const COLLECTIBLE_COUNT: usize = 10;
const COLLECTIBLE_SPIN: f32 = 2.0;
const GROUND_SIZE: f32 = 20.0;
#[derive(Component)]
struct Player;
#[derive(Component)]
struct Collectible;
#[derive(Resource)]
struct Score(u32);
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    #[default]
    Playing,
    Won,
}
fn main() {
    let _exit = App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Collect the Cubes!"),
                resolution: (800u32, 600u32).into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(Score(0))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (mv_player, spin_collectibles, check_collection, update_ui)
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}
fn setup(
    mut cmds: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut mats: ResMut<'_, Assets<StandardMaterial>>,
) {
    let _ground = cmds.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(GROUND_SIZE, GROUND_SIZE))),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        Transform::from_translation(Vec3::ZERO),
    ));
    let _player = cmds.spawn((
        Player,
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(3).expect("e4a1b2c3"))),
        MeshMaterial3d(mats.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.9),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    let cube_mesh = meshes.add(Cuboid::new(0.6, 0.6, 0.6));
    let cube_mat = mats.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.8, 0.1),
        emissive: LinearRgba::new(0.5, 0.4, 0.0, 1.0),
        ..default()
    });
    let half = GROUND_SIZE / 2.0 - 1.0;
    for i in 0..COLLECTIBLE_COUNT {
        use std::f32::consts::TAU;
        let angle = TAU * i as f32 / COLLECTIBLE_COUNT as f32;
        let radius = half * 0.6;
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;
        let _coll = cmds.spawn((
            Collectible,
            Mesh3d(cube_mesh.clone()),
            MeshMaterial3d(cube_mat.clone()),
            Transform::from_xyz(x, 0.8, z),
        ));
    }
    let _light = cmds.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));
    let _cam = cmds.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let _ui = cmds.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
    ));
}
fn mv_player(
    kbd: Res<'_, ButtonInput<KeyCode>>,
    time: Res<'_, Time>,
    mut qry: Query<'_, '_, &mut Transform, With<Player>>,
) {
    let Ok(mut tf) = qry.single_mut() else {
        return;
    };
    let mut dir = Vec3::ZERO;
    if kbd.pressed(KeyCode::KeyW) || kbd.pressed(KeyCode::ArrowUp) {
        dir.z -= 1.0;
    }
    if kbd.pressed(KeyCode::KeyS) || kbd.pressed(KeyCode::ArrowDown) {
        dir.z += 1.0;
    }
    if kbd.pressed(KeyCode::KeyA) || kbd.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if kbd.pressed(KeyCode::KeyD) || kbd.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }
    if dir != Vec3::ZERO {
        dir = dir.normalize();
    }
    tf.translation += dir * PLAYER_SPEED * time.delta_secs();
    let bound = GROUND_SIZE / 2.0 - 0.5;
    tf.translation.x = tf.translation.x.clamp(-bound, bound);
    tf.translation.z = tf.translation.z.clamp(-bound, bound);
}
fn spin_collectibles(
    time: Res<'_, Time>,
    mut qry: Query<'_, '_, &mut Transform, With<Collectible>>,
) {
    for mut tf in &mut qry {
        tf.rotate_y(COLLECTIBLE_SPIN * time.delta_secs());
        tf.translation.y = (time.elapsed_secs() * 2.0).sin().mul_add(0.2, 0.8);
    }
}
fn check_collection(
    mut cmds: Commands<'_, '_>,
    player_qry: Query<'_, '_, &Transform, With<Player>>,
    coll_qry: Query<'_, '_, (Entity, &Transform), With<Collectible>>,
    mut score: ResMut<'_, Score>,
    mut next_state: ResMut<'_, NextState<GameState>>,
) {
    let Ok(player_tf) = player_qry.single() else {
        return;
    };
    for (ent, coll_tf) in &coll_qry {
        if player_tf.translation.distance(coll_tf.translation) < 1.0 {
            cmds.entity(ent).despawn();
            score.0 += 1;
            if score.0 as usize >= COLLECTIBLE_COUNT {
                next_state.set(GameState::Won);
            }
        }
    }
}
fn update_ui(score: Res<'_, Score>, mut qry: Query<'_, '_, &mut Text>) {
    if score.is_changed() {
        for mut txt in &mut qry {
            if score.0 as usize >= COLLECTIBLE_COUNT {
                **txt = String::from("You Win!");
            } else {
                **txt = format!("Score: {}", score.0);
            }
        }
    }
}
