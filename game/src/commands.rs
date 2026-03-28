use bevy::prelude::*;

use crate::units::{MovementTarget, Selected};
#[allow(unused_results)]
pub(crate) struct CommandsPlugin;
impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        let _ = app.add_systems(Update, handle_commands);
    }
}
fn handle_commands(
    mut commands: Commands<'_, '_>,
    mouse_buttons: Res<'_, ButtonInput<MouseButton>>,
    _keys: Res<'_, ButtonInput<KeyCode>>,
    q_cam: Query<'_, '_, (&Camera, &GlobalTransform)>,
    q_selected: Query<'_, '_, (Entity, &Transform), With<Selected>>,
    windows: Query<'_, '_, &Window>,
) {
    if !mouse_buttons.just_pressed(MouseButton::Right) {
        return;
    }
    if q_selected.is_empty() {
        return;
    }
    let Ok((camera, cam_transform)) = q_cam.single() else {
        return;
    };
    let Ok(window) = windows.single() else { return };
    if let Some(pos) = window.cursor_position() {
        if let Ok(ray) = camera.viewport_to_world(cam_transform, pos) {
            let ray_dir = ray.direction.normalize();
            let ray_origin = ray.origin;
            let target_pos = ray_origin + ray_dir * 20.0;
            let center_pos = q_selected
                .iter()
                .map(|(_, t)| t.translation)
                .fold(Vec3::ZERO, |acc, p| acc + p)
                / q_selected.iter().count() as f32;
            for (entity, transform) in q_selected.iter() {
                let offset = transform.translation - center_pos;
                let final_pos = target_pos + offset;
                let _ = commands.entity(entity).insert(MovementTarget {
                    position: Vec3::new(final_pos.x, 0.5, final_pos.z),
                });
            }
        }
    }
}
