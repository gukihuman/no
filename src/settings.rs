use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode, WindowTheme},
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
pub struct SettingsPlugin;
impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        let settings = get_settings();
        let window_plugin = apply_window_settings(&settings);
        app.insert_resource(settings)
            .insert_resource(ClearColor(Color::BLACK))
            .add_plugins(DefaultPlugins.set(window_plugin).build());
    }
}
const TITLE: &str = "Nectar Obsession";
pub const SETTINGS_PATH: &str = "settings.toml";
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WindowSettings {
    pub width: u32,
    pub height: u32,
    pub mode: String,
    pub background_image: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OtherSettings {
    pub custom_cursor: bool,
}
#[derive(Resource, Serialize, Deserialize, Debug, Clone)]
pub struct GameSettings {
    pub window: WindowSettings,
    pub other: OtherSettings,
}
impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            window: WindowSettings {
                width: 1920,
                height: 1080,
                mode: "fullscreen".into(),
                background_image: "cover".into(),
            },
            other: OtherSettings {
                custom_cursor: true,
            },
        }
    }
}
fn get_settings() -> GameSettings {
    let config_path = Path::new(SETTINGS_PATH);
    if let Ok(contents) = fs::read_to_string(config_path) {
        if let Ok(settings) = toml::from_str(&contents) {
            return settings;
        }
    }
    if let Ok(toml_string) = toml::to_string_pretty(&GameSettings::default()) {
        let _ = fs::write(SETTINGS_PATH, toml_string);
    }
    GameSettings::default()
}
fn apply_window_settings(settings: &GameSettings) -> WindowPlugin {
    let mode = match settings.window.mode.as_str() {
        "fullscreen" => WindowMode::BorderlessFullscreen,
        "windowed" => WindowMode::Windowed,
        _ => WindowMode::BorderlessFullscreen,
    };
    WindowPlugin {
        primary_window: Some(Window {
            mode,
            present_mode: PresentMode::AutoVsync,
            title: TITLE.into(),
            resizable: true,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}
