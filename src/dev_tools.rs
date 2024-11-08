use crate::{
    cameras::setup_cameras,
    elements::{Element, ElementId, ElementMap, TextElement},
    game_data::GameData,
    steps::{ElementSet, StepMap},
};
use bevy::{prelude::*, sprite::Anchor::TopLeft, time::Time};
pub struct DevToolsPlugin;
impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_timer, setup.after(setup_cameras)))
            .add_systems(Update, (update_timer, update_cursor_position));
    }
}
const X_EDGE: f32 = 1240.;
const Y_EDGE: f32 = 680.;
#[derive(Resource)]
pub struct DebugTimer(pub Timer);
fn setup(
    mut element_map: ResMut<ElementMap>,
    mut step_map: ResMut<StepMap>,
    game_data: Res<GameData>,
) {
    let mut elements = ElementSet::new();
    elements.insert("dev_tools_gold".into());
    element_map.0.insert(
        "dev_tools_gold".into(),
        Element::Text(TextElement {
            content: format!("Gold: {}", game_data.get("gold")).into(),
            position: Vec3::new(-X_EDGE, Y_EDGE, 1.),
            anchor: TopLeft,
            ..default()
        }),
    );
    if let Some(main_menu) = step_map.0.get_mut("main_menu") {
        main_menu.0.extend(elements.clone());
    }
}
pub fn update_cursor_position(mut query: Query<(&mut Text, &ElementId)>, game_data: Res<GameData>) {
    for (mut text, element_id) in query.iter_mut() {
        if element_id.0 == "dev_tools_gold" {
            text.sections[0].value = format!("Gold: {}", game_data.get("gold"));
        }
    }
}

fn setup_timer(mut commands: Commands) {
    commands.insert_resource(DebugTimer(Timer::from_seconds(1., TimerMode::Repeating)));
}
fn update_timer(time: Res<Time>, mut debug_timer: ResMut<DebugTimer>) {
    debug_timer.0.tick(time.delta());
}
