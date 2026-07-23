use bevy::prelude::*;
pub(crate) struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        let _player = app.add_systems(Startup, (spawn_light, spawn_floor));
    }
}
#[expect(
    clippy::single_call_fn,
    reason = "named Bevy systems keep schedule registration explicit"
)]
fn spawn_light(mut commands: Commands<'_, '_>) {
    let _light = commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            range: 20.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0),
    ));
}
#[expect(
    clippy::single_call_fn,
    reason = "named Bevy systems keep schedule registration explicit"
)]
fn spawn_floor(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let _floor = commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(15.0, 15.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));
}
