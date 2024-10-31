use crate::{
    cameras::{setup_cameras, CanvasScaleFit},
    game_data::GameData,
};
use bevy::{prelude::*, sprite::Anchor::TopLeft, time::Time};
pub struct DevToolsPlugin;
impl Plugin for DevToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_timer, setup.after(setup_cameras)))
            .add_systems(Update, (update_timer, update_cursor_position));
    }
}
#[derive(Resource)]
pub struct DebugTimer(pub Timer);
#[derive(Component)]
pub struct Gold;
fn setup(mut commands: Commands, canvas_scale_fit: Res<CanvasScaleFit>) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    font_size: 18. * canvas_scale_fit.0,
                    ..default()
                },
            )
            .with_justify(JustifyText::Left),
            text_anchor: TopLeft,
            transform: Transform::from_translation(Vec3::new(-470., 260., 1.) * canvas_scale_fit.0),
            ..default()
        },
        Gold,
    ));
}
pub fn update_cursor_position(mut query: Query<&mut Text, With<Gold>>, game_data: Res<GameData>) {
    for mut text in &mut query {
        text.sections[0].value = format!("Gold: {}", game_data.get("gold"));
    }
}

fn setup_timer(mut commands: Commands) {
    commands.insert_resource(DebugTimer(Timer::from_seconds(1., TimerMode::Repeating)));
}
fn update_timer(time: Res<Time>, mut debug_timer: ResMut<DebugTimer>) {
    debug_timer.0.tick(time.delta());
}
