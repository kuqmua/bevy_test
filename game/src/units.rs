use std::time::Duration;

use bevy::{color::palettes::css, prelude::*};
#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum UnitType {
    Peasant,
    Footman,
    Knight,
    Archer,
    Mage,
}
#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Faction {
    Human,
    Orc,
}
#[derive(Component)]
#[allow(dead_code)]
pub(crate) struct Unit {
    pub unit_type: UnitType,
    pub faction: Faction,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
    pub attack_damage: f32,
    pub attack_range: f32,
    pub attack_cooldown: Duration,
    pub last_attack: Option<Duration>,
}
#[derive(Component)]
pub(crate) struct Selected;
impl Unit {
    pub(crate) fn new(unit_type: UnitType, faction: Faction) -> Self {
        let (hp, speed, damage, range, cooldown) = match unit_type {
            UnitType::Peasant => (50.0, 4.0, 5.0, 1.5, Duration::from_secs(1)),
            UnitType::Footman => (100.0, 5.0, 12.0, 1.5, Duration::from_millis(800)),
            UnitType::Knight => (150.0, 6.0, 20.0, 1.5, Duration::from_millis(1200)),
            UnitType::Archer => (60.0, 5.5, 15.0, 8.0, Duration::from_millis(1000)),
            UnitType::Mage => (40.0, 4.5, 25.0, 6.0, Duration::from_millis(1500)),
        };
        Self {
            unit_type,
            faction,
            hp,
            max_hp: hp,
            speed,
            attack_damage: damage,
            attack_range: range,
            attack_cooldown: cooldown,
            last_attack: None,
        }
    }
}
pub(crate) fn spawn_unit(
    commands: &mut Commands<'_, '_>,
    materials: &mut Assets<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
    unit_type: UnitType,
    faction: Faction,
    pos: Vec3,
) -> Entity {
    let unit = Unit::new(unit_type, faction);
    let color = match faction {
        Faction::Human => css::BLUE,
        Faction::Orc => css::RED,
    };
    let size = match unit_type {
        UnitType::Peasant => 0.4,
        UnitType::Footman => 0.5,
        UnitType::Knight => 0.6,
        UnitType::Archer => 0.35,
        UnitType::Mage => 0.35,
    };
    let material = materials.add(StandardMaterial {
        base_color: Color::from(color),
        ..default()
    });
    let mesh = meshes.add(Mesh::from(Capsule3d::new(size, size * 2.0)));
    commands
        .spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(pos),
            unit,
        ))
        .id()
}
#[derive(Component)]
pub(crate) struct MovementTarget {
    pub position: Vec3,
}
#[derive(Component)]
pub(crate) struct AttackTarget {
    pub entity: Entity,
}
