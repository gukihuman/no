use bevy::prelude::*;
mod cameras;
mod cursor;
mod dev_tools;
mod elements;
mod game_data;
mod interactions;
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
        game_data::GameDataPlugin,
        interactions::InteractionsPlugin,
        cursor::CursorPlugin,
        dev_tools::DevToolsPlugin,
    ))
    .run();
}
