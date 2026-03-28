use bevy::prelude::*;
#[derive(Resource, Clone, Default)]
pub(crate) struct PlayerResources {
    pub gold: i32,
    pub wood: i32,
}
#[derive(Component)]
pub(crate) struct GoldMine {
    #[allow(dead_code)]
    pub amount: i32,
}
#[derive(Component)]
pub(crate) struct Tree {
    #[allow(dead_code)]
    pub wood_amount: i32,
}
pub(crate) fn spawn_gold_mine(
    commands: &mut Commands<'_, '_>,
    materials: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    pos: Vec3,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.84, 0.0),
        ..default()
    });
    let mesh = meshes.add(Mesh::from(Cylinder::new(1.5, 2.0)));
    let _ = commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(pos),
        GoldMine { amount: 10000 },
    ));
}
pub(crate) fn spawn_tree(
    commands: &mut Commands<'_, '_>,
    materials: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    pos: Vec3,
) {
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.13, 0.55, 0.13),
        ..default()
    });
    let mesh = meshes.add(Mesh::from(Cone::new(0.8, 3.0)));
    let _ = commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(pos),
        Tree { wood_amount: 500 },
    ));
}
