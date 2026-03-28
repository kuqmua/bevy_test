use bevy::prelude::*;

use crate::units::{Selected, Unit};
#[allow(unused_results)]
pub(crate) struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.init_resource::<SelectionState>();
        let _ = app.add_systems(Update, handle_selection);
    }
}
#[derive(Resource, Default)]
struct SelectionState {
    click_start: Option<Vec2>,
}
fn handle_selection(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<StandardMaterial>>,
    mut selection_state: ResMut<'_, SelectionState>,
    mouse_buttons: Res<'_, ButtonInput<MouseButton>>,
    q_cam: Query<'_, '_, (&Camera, &GlobalTransform)>,
    q_units: Query<'_, '_, (Entity, &Transform, &Unit)>,
    q_selected: Query<'_, '_, Entity, With<Selected>>,
    windows: Query<'_, '_, &Window>,
) {
    let Ok((camera, cam_transform)) = q_cam.single() else {
        return;
    };
    let Ok(window) = windows.single() else { return };
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };
    if mouse_buttons.just_pressed(MouseButton::Left) {
        selection_state.click_start = Some(cursor_pos);
    }
    if mouse_buttons.just_released(MouseButton::Left) {
        let Some(start_pos) = selection_state.click_start else {
            return;
        };
        selection_state.click_start = None;
        let end_pos = cursor_pos;
        for entity in q_selected.iter() {
            let _ = commands.entity(entity).remove::<Selected>();
        }
        if start_pos.distance(end_pos) < 5.0 {
            if let Some(pos) = window.cursor_position() {
                if let Ok(ray) = camera.viewport_to_world(cam_transform, pos) {
                    let ray_dir = ray.direction.normalize();
                    let ray_origin = ray.origin;
                    let plane_normal = Vec3::Y;
                    let plane_d = 0.0;
                    let denom = ray_dir.dot(plane_normal);
                    if denom.abs() > 1e-6 {
                        let t = -(ray_origin.dot(plane_normal) + plane_d) / denom;
                        if t > 0.0 {
                            let hit_point = ray_origin + ray_dir * t;
                            let mut closest_unit: Option<Entity> = None;
                            let mut closest_dist = f32::MAX;
                            for (entity, transform, _) in q_units.iter() {
                                let dist = transform.translation.distance(hit_point);
                                if dist < 2.0 && dist < closest_dist {
                                    closest_dist = dist;
                                    closest_unit = Some(entity);
                                }
                            }
                            if let Some(entity) = closest_unit {
                                let _ = commands.entity(entity).insert(Selected);
                                if let Ok((_, transform, _)) = q_units.get(entity) {
                                    let circle = meshes.add(Mesh::from(Circle { radius: 1.0 }));
                                    let material = materials.add(StandardMaterial {
                                        base_color: Color::srgb(0.0, 1.0, 0.0),
                                        alpha_mode: AlphaMode::Blend,
                                        ..default()
                                    });
                                    let _ = commands.spawn((
                                        Mesh3d(circle),
                                        MeshMaterial3d(material),
                                        Transform::from_translation(
                                            transform.translation + Vec3::new(0.0, 0.05, 0.0),
                                        ),
                                        SelectionRing { unit: entity },
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
#[derive(Component)]
#[allow(dead_code)]
struct SelectionRing {
    unit: Entity,
}
