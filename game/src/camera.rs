use bevy::prelude::*;
pub(crate) struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let _camera = app.add_systems(Startup, spawn_camera);
    }
}
#[expect(
    clippy::single_call_fn,
    reason = "named Bevy systems keep schedule registration explicit"
)]
fn spawn_camera(mut commands: Commands<'_, '_>) {
    let _camera = commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}