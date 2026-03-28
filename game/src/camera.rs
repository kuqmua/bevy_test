use bevy::prelude::*;
pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let _ = app
            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement);
    }
}
#[derive(Component)]
struct MainCamera;
fn setup_camera(mut commands: Commands<'_, '_>) {
    let _ = commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));
}
fn camera_movement(
    keys: Res<'_, ButtonInput<KeyCode>>,
    mut query: Query<'_, '_, &mut Transform, With<MainCamera>>,
    time: Res<'_, Time>,
) {
    let Ok(mut transform) = query.single_mut() else {
        return;
    };
    let speed = 15.0 * time.delta_secs();
    let forward = transform.forward();
    let right = transform.right();
    if keys.pressed(KeyCode::KeyW) {
        transform.translation += forward * speed;
    }
    if keys.pressed(KeyCode::KeyS) {
        transform.translation -= forward * speed;
    }
    if keys.pressed(KeyCode::KeyA) {
        transform.translation -= right * speed;
    }
    if keys.pressed(KeyCode::KeyD) {
        transform.translation += right * speed;
    }
    transform.translation.y = transform.translation.y.max(5.0);
}
