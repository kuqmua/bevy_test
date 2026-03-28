use bevy::prelude::*;

use crate::units::{AttackTarget, MovementTarget, Unit};
#[allow(unused_results)]
pub(crate) struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        let _ = app
            .add_systems(Update, move_units)
            .add_systems(Update, attack_units)
            .add_systems(Update, unit_cleanup);
    }
}
fn move_units(
    time: Res<'_, Time>,
    mut q_units: Query<'_, '_, (&mut Transform, &mut Unit, &MovementTarget)>,
) {
    for (mut transform, unit, target) in q_units.iter_mut() {
        let direction = (target.position - transform.translation).normalize();
        let distance = transform.translation.distance(target.position);
        if distance > 0.1 {
            let movement = direction * unit.speed * time.delta_secs();
            transform.translation += movement;
        }
    }
}
fn attack_units(
    mut commands: Commands<'_, '_>,
    time: Res<'_, Time>,
    q_attackers: Query<'_, '_, (Entity, &AttackTarget)>,
    mut q_units: ParamSet<
        '_,
        '_,
        (
            Query<'_, '_, (Entity, &Unit, Option<&Transform>)>,
            Query<'_, '_, (Entity, &mut Unit, Option<&mut Transform>)>,
        ),
    >,
) {
    let mut to_remove = Vec::new();
    let mut to_despawn = Vec::new();
    let mut attacks: Vec<(Entity, Entity, f32)> = Vec::new();
    let mut movements: Vec<(Entity, Vec3)> = Vec::new();
    let q_read = q_units.p0();
    for (attacker_entity, attack_target) in q_attackers.iter() {
        let Ok((_, attacker, attacker_transform)) = q_read.get(attacker_entity) else {
            to_remove.push(attacker_entity);
            continue;
        };
        let Ok((target_entity, target, target_transform)) = q_read.get(attack_target.entity) else {
            to_remove.push(attacker_entity);
            continue;
        };
        let attacker_pos = attacker_transform
            .map(|t| t.translation)
            .unwrap_or(Vec3::ZERO);
        let target_pos = target_transform
            .map(|t| t.translation)
            .unwrap_or(Vec3::ZERO);
        let distance = attacker_pos.distance(target_pos);
        let now = time.elapsed();
        if distance <= attacker.attack_range {
            let can_attack = match attacker.last_attack {
                Some(last) => now - last >= attacker.attack_cooldown,
                None => true,
            };
            if can_attack {
                attacks.push((attacker_entity, target_entity, attacker.attack_damage));
            }
        } else {
            let direction = (target_pos - attacker_pos).normalize();
            movements.push((
                attacker_entity,
                direction * attacker.speed * time.delta_secs(),
            ));
        }
        if target.hp <= 0.0 {
            to_despawn.push(target_entity);
            to_remove.push(attacker_entity);
        }
    }
    let mut q_write = q_units.p1();
    for (attacker, target, damage) in attacks {
        if let Ok((_, mut target_unit, _)) = q_write.get_mut(target) {
            target_unit.hp -= damage;
        }
        if let Ok((_, mut attacker_unit, _)) = q_write.get_mut(attacker) {
            attacker_unit.last_attack = Some(time.elapsed());
        }
    }
    for (entity, movement) in movements {
        if let Ok((_, _, mut transform)) = q_write.get_mut(entity) {
            if let Some(t) = transform.as_mut() {
                t.translation += movement;
            }
        }
    }
    for entity in to_despawn {
        commands.entity(entity).despawn();
    }
    for entity in to_remove {
        let _ = commands.entity(entity).remove::<AttackTarget>();
    }
}
fn unit_cleanup(
    mut commands: Commands<'_, '_>,
    q_dead: Query<'_, '_, (Entity, &Unit), (Without<MovementTarget>, Without<AttackTarget>)>,
) {
    for (entity, unit) in q_dead.iter() {
        if unit.hp <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
