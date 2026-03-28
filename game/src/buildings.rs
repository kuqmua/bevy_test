use bevy::{color::palettes::css, prelude::*};

use crate::{resources::PlayerResources, units::Faction};
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BuildingType {
    TownHall,
    Barracks,
    Farm,
    #[allow(dead_code)]
    Tower,
}
#[derive(Component)]
pub(crate) struct Building {
    pub building_type: BuildingType,
    pub faction: Faction,
    #[allow(dead_code)]
    pub hp: f32,
    #[allow(dead_code)]
    pub max_hp: f32,
}
impl Building {
    pub(crate) fn new(building_type: BuildingType, faction: Faction) -> Self {
        let (hp, _size) = match building_type {
            BuildingType::TownHall => (2000.0, 3.0),
            BuildingType::Barracks => (1500.0, 2.5),
            BuildingType::Farm => (500.0, 1.5),
            BuildingType::Tower => (800.0, 1.2),
        };
        Self {
            building_type,
            faction,
            hp,
            max_hp: hp,
        }
    }
}
pub(crate) fn spawn_building(
    commands: &mut Commands<'_, '_>,
    materials: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    building_type: BuildingType,
    faction: Faction,
    pos: Vec3,
) -> Entity {
    let building = Building::new(building_type, faction);
    let color = match faction {
        Faction::Human => css::BLUE,
        Faction::Orc => css::RED,
    };
    let box_size = match building_type {
        BuildingType::TownHall => 3.0,
        BuildingType::Barracks => 2.5,
        BuildingType::Farm => 1.5,
        BuildingType::Tower => 1.2,
    };
    let material = materials.add(StandardMaterial {
        base_color: Color::from(color),
        ..default()
    });
    let mesh = meshes.add(Mesh::from(Cuboid::new(box_size, 2.0, box_size)));
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(pos),
            building,
        ))
        .id()
}
#[allow(unused_results)]
pub(crate) struct BuildingPlugin;
impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.add_systems(Startup, setup_buildings);
    }
}
fn setup_buildings(
    mut commands: Commands<'_, '_>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut resources: ResMut<'_, PlayerResources>,
) {
    resources.gold = 500;
    resources.wood = 200;
    let _ = spawn_building(
        &mut commands,
        &mut materials,
        &mut meshes,
        BuildingType::TownHall,
        Faction::Human,
        Vec3::new(-15.0, 1.0, 0.0),
    );
    let _ = spawn_building(
        &mut commands,
        &mut materials,
        &mut meshes,
        BuildingType::Barracks,
        Faction::Human,
        Vec3::new(-12.0, 1.0, 5.0),
    );
    let _ = spawn_building(
        &mut commands,
        &mut materials,
        &mut meshes,
        BuildingType::Farm,
        Faction::Human,
        Vec3::new(-18.0, 0.75, 5.0),
    );
    let _ = spawn_building(
        &mut commands,
        &mut materials,
        &mut meshes,
        BuildingType::TownHall,
        Faction::Orc,
        Vec3::new(15.0, 1.0, 0.0),
    );
    let _ = spawn_building(
        &mut commands,
        &mut materials,
        &mut meshes,
        BuildingType::Barracks,
        Faction::Orc,
        Vec3::new(12.0, 1.0, -5.0),
    );
    let _ = spawn_building(
        &mut commands,
        &mut materials,
        &mut meshes,
        BuildingType::Farm,
        Faction::Orc,
        Vec3::new(18.0, 0.75, -5.0),
    );
    crate::resources::spawn_gold_mine(
        &mut commands,
        &mut materials,
        &mut meshes,
        Vec3::new(0.0, 1.0, 0.0),
    );
    crate::resources::spawn_tree(
        &mut commands,
        &mut materials,
        &mut meshes,
        Vec3::new(-8.0, 1.5, 8.0),
    );
    crate::resources::spawn_tree(
        &mut commands,
        &mut materials,
        &mut meshes,
        Vec3::new(-6.0, 1.5, 9.0),
    );
    crate::resources::spawn_tree(
        &mut commands,
        &mut materials,
        &mut meshes,
        Vec3::new(8.0, 1.5, -8.0),
    );
    crate::resources::spawn_tree(
        &mut commands,
        &mut materials,
        &mut meshes,
        Vec3::new(6.0, 1.5, -9.0),
    );
}
