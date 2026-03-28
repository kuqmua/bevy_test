use bevy::prelude::*;

use crate::{
    buildings::{Building, BuildingType},
    units::{AttackTarget, Faction, MovementTarget, Unit},
};
#[allow(unused_results)]
pub(crate) struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.init_resource::<AiState>();
        let _ = app.add_systems(Update, ai_behavior);
    }
}
#[derive(Resource, Default)]
struct AiState {
    last_action_time: f32,
}
#[allow(
    clippy::float_arithmetic,
    clippy::min_ident_chars,
    clippy::needless_pass_by_value
)]
fn ai_behavior(
    time: Res<'_, Time>,
    mut ai_state: ResMut<'_, AiState>,
    q_orc_units: Query<
        '_,
        '_,
        (Entity, &Transform, &Unit, Option<&MovementTarget>),
        (With<Unit>, Without<AttackTarget>),
    >,
    q_human_units: Query<'_, '_, (Entity, &Transform, &Unit), With<Unit>>,
    q_orc_buildings: Query<'_, '_, (&Transform, &Building), (With<Building>, Without<Unit>)>,
    mut commands: Commands<'_, '_>,
) {
    let elapsed = time.elapsed_secs();
    if elapsed - ai_state.last_action_time < 2.0 {
        return;
    }
    ai_state.last_action_time = elapsed;
    let human_targets: Vec<(Entity, Vec3)> = q_human_units
        .iter()
        .map(|(ent, tr, _)| (ent, tr.translation))
        .collect();
    let _town_center = q_orc_buildings
        .iter()
        .find(|(_, b)| b.faction == Faction::Orc && b.building_type == BuildingType::TownHall);
    if human_targets.is_empty() {
        return;
    }
    let Some(target) = human_targets.first() else {
        return;
    };
    for (entity, transform, unit, move_target) in q_orc_units.iter() {
        if unit.faction != Faction::Orc {
            continue;
        }
        if move_target.is_some() {
            continue;
        }
        let distance = transform.translation.distance(target.1);
        if distance < 10.0 {
            let _: &mut EntityCommands<'_> = commands
                .entity(entity)
                .insert(AttackTarget { entity: target.0 });
        } else {
            let _: &mut EntityCommands<'_> = commands
                .entity(entity)
                .insert(MovementTarget { position: target.1 });
        }
    }
}
