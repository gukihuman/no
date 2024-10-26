use bevy::prelude::*;
mod cameras;
mod elements;
mod main_menu;
mod settings;
mod steps;
fn main() {
    let mut app = App::new();
    app.add_plugins((
        settings::SettingsPlugin,
        cameras::CameraPlugin,
        steps::StepsPlugin,
        elements::ElementsPlugin,
        main_menu::MainMenuPlugin,
    ))
    .run();
}
