use bevy::prelude::*;
pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let _player = app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}
#[derive(Component)]
struct Player;
#[expect(
    clippy::single_call_fn,
    clippy::needless_pass_by_value,
    clippy::arithmetic_side_effects,
    reason = "Bevy systems take value parameters and movement requires vector arithmetic"
)]
fn player_movement(
    keys: Res<'_, ButtonInput<KeyCode>>,
    time: Res<'_, Time>,
    mut player_q: Query<'_, '_, &mut Transform, With<Player>>,
    cam_q: Query<'_, '_, &Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(cam) = cam_q.single() else {
        return;
    };
    for mut player_transform in player_q.iter_mut() {
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) {
            direction += *cam.forward();
        }
        if keys.pressed(KeyCode::KeyS) {
            direction += *cam.back();
        }
        if keys.pressed(KeyCode::KeyA) {
            direction += *cam.left();
        }
        if keys.pressed(KeyCode::KeyD) {
            direction += *cam.right();
        }
        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * 2.0 * time.delta_secs();
        player_transform.translation += movement;
    }
}
#[expect(
    clippy::single_call_fn,
    reason = "named Bevy systems keep schedule registration explicit"
)]
fn spawn_player(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let _player = commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player,
    ));
}
