use bevy::{color::palettes::css, prelude::*};
#[allow(unused_results)]
pub(crate) struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.add_systems(Startup, setup_map);
    }
}
fn setup_map(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
) {
    let size = 50;
    let plane = meshes.add(Mesh::from(Plane3d::new(
        Vec3::Y,
        Vec2::new(size as f32, size as f32),
    )));
    let material = materials.add(StandardMaterial {
        base_color: Color::from(css::DARK_GREEN),
        ..default()
    });
    let _ = commands.spawn((
        Mesh3d(plane),
        MeshMaterial3d(material),
        Transform::default(),
    ));
    let grid_size = size;
    let step = 2.0;
    for x in -grid_size / 2..grid_size / 2 {
        for z in -grid_size / 2..grid_size / 2 {
            if x % 5 == 0 && z % 5 == 0 {
                let pos = Vec3::new(x as f32 * step, 0.01, z as f32 * step);
                let circle_mesh = meshes.add(Mesh::from(Circle { radius: 0.1 }));
                let circle_material = materials.add(StandardMaterial {
                    base_color: Color::srgb(0.3, 0.5, 0.3),
                    ..default()
                });
                let _ = commands.spawn((
                    Mesh3d(circle_mesh),
                    MeshMaterial3d(circle_material),
                    Transform::from_translation(pos),
                ));
            }
        }
    }
}
