use bevy::prelude::*;
pub(crate) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        let _player = app.add_systems(Startup, spawn_player);
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
    ));
}
