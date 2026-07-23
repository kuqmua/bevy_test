use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;
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
#[derive(Component)]
struct Speed(f32);
#[expect(
    clippy::single_call_fn,
    clippy::needless_pass_by_value,
    clippy::arithmetic_side_effects,
    reason = "Bevy systems take value parameters and movement requires vector arithmetic"
)]
fn player_movement(
    keys: Res<'_, ButtonInput<KeyCode>>,
    time: Res<'_, Time>,
    mut player_q: Query<'_, '_, (&mut Transform, &Speed), With<Player>>,
    cam_q: Query<'_, '_, &Transform, (With<Camera3d>, Without<Player>)>,
) {
    let Ok(cam) = cam_q.single() else {
        return;
    };
    for (mut player_transform, player_speed) in &mut player_q {
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
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_secs();
        player_transform.translation += movement;
    }
}
#[expect(
    clippy::single_call_fn,
    clippy::needless_pass_by_value,
    reason = "named Bevy systems require value system parameters"
)]
fn spawn_player(mut commands: Commands<'_, '_>, asset_server: Res<'_, AssetServer>) {
    let _player = commands.spawn((
        SceneRoot(asset_server.load("Player.gltf#Scene0")),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player,
        ThirdPersonCameraTarget,
        Speed(2.5),
    ));
}
