use bevy::{color::palettes::css, prelude::*};

use crate::resources::PlayerResources;
#[allow(unused_results)]
pub(crate) struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        let _ = app
            .add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}
#[derive(Component)]
struct ResourceUi;
fn setup_ui(mut commands: Commands<'_, '_>) {
    let _ = commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(80.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::from(css::BLACK.with_alpha(0.7))),
            ResourceUi,
        ))
        .with_children(|parent| {
            let _ = parent.spawn((
                Text::new("Gold: 500 | Wood: 200"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::from(css::GOLD)),
            ));
            let _ = parent.spawn((
                Text::new("Controls: WASD - Move, Mouse - Select/Command"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::from(css::WHITE)),
            ));
        });
}
fn update_ui(
    resources: Res<'_, PlayerResources>,
    mut q_text: Query<'_, '_, &mut Text, With<ResourceUi>>,
) {
    for mut text in q_text.iter_mut() {
        *text = Text::new(format!(
            "Gold: {} | Wood: {}",
            resources.gold, resources.wood
        ));
    }
}
