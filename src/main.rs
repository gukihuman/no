use bevy::prelude::*;
mod cameras;
mod settings;

fn main() {
    let mut app = App::new();
    app.add_plugins(settings::SettingsPlugin)
        .add_systems(Startup, spawn_test_sprite.after(cameras::setup_cameras))
        .add_plugins(cameras::CameraPlugin)
        .run();
}

fn spawn_test_sprite(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    canvas: Res<cameras::CanvasResource>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("main_menu.png"),
            transform: Transform::from_scale(Vec3::new(canvas.scale, canvas.scale, 1.)),
            ..default()
        },
        cameras::CANVAS_LAYER,
    ));
}
