use bevy::prelude::*;
mod ai;
mod buildings;
mod camera;
mod combat;
mod commands;
mod map;
mod resources;
mod selection;
mod ui;
mod units;
fn main() {
    let _ = App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(selection::SelectionPlugin)
        .add_plugins(commands::CommandsPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(buildings::BuildingPlugin)
        .add_plugins(ai::AiPlugin)
        .add_plugins(map::MapPlugin)
        .add_plugins(ui::UiPlugin)
        .init_resource::<resources::PlayerResources>()
        .add_systems(Startup, spawn_initial_units)
        .run();
}
#[allow(dead_code)]
fn spawn_initial_units(
    mut commands: Commands<'_, '_>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
) {
    let _light = commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    for i in 0..3 {
        let _ = units::spawn_unit(
            &mut commands,
            &mut materials,
            &mut meshes,
            units::UnitType::Peasant,
            units::Faction::Human,
            Vec3::new(-12.0 + i as f32, 0.4, 3.0),
        );
    }
    for i in 0..5 {
        let _ = units::spawn_unit(
            &mut commands,
            &mut materials,
            &mut meshes,
            units::UnitType::Footman,
            units::Faction::Human,
            Vec3::new(-14.0 + i as f32 * 1.5, 0.5, 8.0),
        );
    }
    for i in 0..3 {
        let _ = units::spawn_unit(
            &mut commands,
            &mut materials,
            &mut meshes,
            units::UnitType::Peasant,
            units::Faction::Orc,
            Vec3::new(12.0 - i as f32, 0.4, -3.0),
        );
    }
    for i in 0..5 {
        let _ = units::spawn_unit(
            &mut commands,
            &mut materials,
            &mut meshes,
            units::UnitType::Footman,
            units::Faction::Orc,
            Vec3::new(14.0 - i as f32 * 1.5, 0.5, -8.0),
        );
    }
    let _ = units::spawn_unit(
        &mut commands,
        &mut materials,
        &mut meshes,
        units::UnitType::Knight,
        units::Faction::Human,
        Vec3::new(-10.0, 0.6, 0.0),
    );
    let _ = units::spawn_unit(
        &mut commands,
        &mut materials,
        &mut meshes,
        units::UnitType::Archer,
        units::Faction::Human,
        Vec3::new(-10.0, 0.35, 2.0),
    );
    let _ = units::spawn_unit(
        &mut commands,
        &mut materials,
        &mut meshes,
        units::UnitType::Mage,
        units::Faction::Human,
        Vec3::new(-10.0, 0.35, -2.0),
    );
    let _ = units::spawn_unit(
        &mut commands,
        &mut materials,
        &mut meshes,
        units::UnitType::Knight,
        units::Faction::Orc,
        Vec3::new(10.0, 0.6, 0.0),
    );
    let _ = units::spawn_unit(
        &mut commands,
        &mut materials,
        &mut meshes,
        units::UnitType::Archer,
        units::Faction::Orc,
        Vec3::new(10.0, 0.35, -2.0),
    );
    let _ = units::spawn_unit(
        &mut commands,
        &mut materials,
        &mut meshes,
        units::UnitType::Mage,
        units::Faction::Orc,
        Vec3::new(10.0, 0.35, 2.0),
    );
}
