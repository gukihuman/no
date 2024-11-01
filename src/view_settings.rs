use crate::{
    elements::{
        Element, ElementAction, ElementMap, ImageElement, SettingOp, TextElement, TextImageElement,
        ViewStackOp,
    },
    settings::GameSettings,
    steps::ElementSet,
    views::{View, ViewMap},
};
use bevy::{prelude::*, sprite::Anchor::CenterLeft};
pub struct ViewSettingsPlugin;
impl Plugin for ViewSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_root, setup_resolution));
    }
}
const FONT_SIZE: f32 = 20.;
const RETURN_BUTTON_X: f32 = -200.;
const ROOT_Z_INDEX: f32 = 10.;
const ROOT_X_RIGHT_COLUMN: f32 = 80.;
const ROOT_X_LEFT_COLUMN: f32 = -120.;
const ROOT_Y_1_ROW: f32 = 100.;
const ROOT_Y_2_ROW: f32 = 60.;
const ROOT_Y_3_ROW: f32 = 20.;
const ROOT_Y_4_ROW: f32 = -20.;
const RESOLUTION_Z_INDEX: f32 = 20.;
const RESOLUTION_WIDTH: [u32; 6] = [1280, 1366, 1600, 1920, 2560, 3840];
const RESOLUTION_HEIGHT: [u32; 9] = [720, 768, 900, 1024, 1080, 1200, 1440, 1600, 2160];
const RESOLUTION_X_SPACING: f32 = 140.;
const RESOLUTION_Y_SPACING: f32 = 40.;
const RESOLUTION_X_GRID: f32 = -350.;
const RESOLUTION_Y_GRID: f32 = 180.;
fn setup_root(
    mut element_map: ResMut<ElementMap>,
    mut view_map: ResMut<ViewMap>,
    settings: Res<GameSettings>,
) {
    let mut elements = ElementSet::new();
    elements.insert("settings_black_screen".into());
    element_map.0.insert(
        "settings_black_screen".into(),
        Element::Image(ImageElement {
            path: "black_screen.webp".into(),
            position: Vec3::new(0., 0., ROOT_Z_INDEX - 1.),
            color: Color::srgba(1., 1., 1., 0.98),
            ..default()
        }),
    );
    elements.insert("settings_button_return".into());
    element_map.0.insert(
        "settings_button_return".into(),
        Element::TextImage(TextImageElement {
            content: "Return".into(),
            path: "button.webp".into(),
            position: Vec3::new(0., RETURN_BUTTON_X, ROOT_Z_INDEX),
            font_size: FONT_SIZE,
            actions: Vec::from([ElementAction::ChangeViewStack(ViewStackOp::Pop())]),
            ..default()
        }),
    );
    elements.insert("settings_resolution_text".into());
    element_map.0.insert(
        "settings_resolution_text".into(),
        Element::Text(TextElement {
            content: "Resolution".into(),
            position: Vec3::new(ROOT_X_LEFT_COLUMN, ROOT_Y_1_ROW, ROOT_Z_INDEX),
            anchor: CenterLeft,
            ..default()
        }),
    );
    elements.insert("settings_resolution".into());
    element_map.0.insert(
        "settings_resolution".into(),
        Element::TextImage(TextImageElement {
            content: format!("{} x {}", settings.window.width, settings.window.height),
            path: "button.webp".into(),
            position: Vec3::new(ROOT_X_RIGHT_COLUMN, ROOT_Y_1_ROW, ROOT_Z_INDEX),
            font_size: FONT_SIZE,
            actions: Vec::from([ElementAction::ChangeViewStack(ViewStackOp::Push(
                "settings_resolution".into(),
            ))]),
            ..default()
        }),
    );
    elements.insert("settings_window_mode_text".into());
    element_map.0.insert(
        "settings_window_mode_text".into(),
        Element::Text(TextElement {
            content: "Window mode".into(),
            position: Vec3::new(ROOT_X_LEFT_COLUMN, ROOT_Y_2_ROW, ROOT_Z_INDEX),
            anchor: CenterLeft,
            ..default()
        }),
    );
    elements.insert("settings_window_mode".into());
    element_map.0.insert(
        "settings_window_mode".into(),
        Element::TextImage(TextImageElement {
            content: settings.window.mode.clone(),
            path: "button.webp".into(),
            position: Vec3::new(ROOT_X_RIGHT_COLUMN, ROOT_Y_2_ROW, ROOT_Z_INDEX),
            font_size: FONT_SIZE,
            actions: Vec::from([ElementAction::ChangeSetting(SettingOp::ToggleWindowMode())]),
            ..default()
        }),
    );
    elements.insert("settings_background_image_text".into());
    element_map.0.insert(
        "settings_background_image_text".into(),
        Element::Text(TextElement {
            content: "Background image".into(),
            position: Vec3::new(ROOT_X_LEFT_COLUMN, ROOT_Y_3_ROW, ROOT_Z_INDEX),
            anchor: CenterLeft,
            ..default()
        }),
    );
    elements.insert("settings_background_image".into());
    element_map.0.insert(
        "settings_background_image".into(),
        Element::TextImage(TextImageElement {
            content: settings.window.background_image.clone(),
            path: "button.webp".into(),
            position: Vec3::new(ROOT_X_RIGHT_COLUMN, ROOT_Y_3_ROW, ROOT_Z_INDEX),
            font_size: FONT_SIZE,
            actions: Vec::from([ElementAction::ChangeSetting(
                SettingOp::ToggleBackgroundImage(),
            )]),
            ..default()
        }),
    );
    elements.insert("settings_custom_cursor_text".into());
    element_map.0.insert(
        "settings_custom_cursor_text".into(),
        Element::Text(TextElement {
            content: "Custom cursor".into(),
            position: Vec3::new(ROOT_X_LEFT_COLUMN, ROOT_Y_4_ROW, ROOT_Z_INDEX),
            anchor: CenterLeft,
            ..default()
        }),
    );
    elements.insert("settings_custom_cursor".into());
    element_map.0.insert(
        "settings_custom_cursor".into(),
        Element::TextImage(TextImageElement {
            content: match settings.other.custom_cursor {
                true => "on".into(),
                false => "off".into(),
            },
            path: "button.webp".into(),
            position: Vec3::new(ROOT_X_RIGHT_COLUMN, ROOT_Y_4_ROW, ROOT_Z_INDEX),
            font_size: FONT_SIZE,
            actions: Vec::from([ElementAction::ChangeSetting(SettingOp::ToggleCustomCursor())]),
            ..default()
        }),
    );
    view_map.0.insert("settings_root".into(), View(elements));
}
fn setup_resolution(mut element_map: ResMut<ElementMap>, mut view_map: ResMut<ViewMap>) {
    let mut elements = ElementSet::new();
    elements.insert("resolution_black_screen".into());
    element_map.0.insert(
        "resolution_black_screen".into(),
        Element::Image(ImageElement {
            path: "black_screen.webp".into(),
            position: Vec3::new(0., 0., RESOLUTION_Z_INDEX - 1.),
            color: Color::srgba(1., 1., 1., 0.98),
            ..default()
        }),
    );
    elements.insert("resolution_button_return".into());
    element_map.0.insert(
        "resolution_button_return".into(),
        Element::TextImage(TextImageElement {
            content: "Return".into(),
            path: "button.webp".into(),
            position: Vec3::new(0., RETURN_BUTTON_X, RESOLUTION_Z_INDEX),
            font_size: FONT_SIZE,
            actions: Vec::from([ElementAction::ChangeViewStack(ViewStackOp::Pop())]),
            ..default()
        }),
    );
    for (y_idx, &height) in RESOLUTION_HEIGHT.iter().enumerate() {
        for (x_idx, &width) in RESOLUTION_WIDTH.iter().enumerate() {
            let element_id = format!("resolution_{}x{}", width, height);
            let x_pos = RESOLUTION_X_GRID + (x_idx as f32 * RESOLUTION_X_SPACING);
            let y_pos = RESOLUTION_Y_GRID - (y_idx as f32 * RESOLUTION_Y_SPACING);
            elements.insert(element_id.clone());
            element_map.0.insert(
                element_id,
                Element::TextImage(TextImageElement {
                    content: format!("{} x {}", width, height),
                    path: "button.webp".into(),
                    position: Vec3::new(x_pos, y_pos, RESOLUTION_Z_INDEX),
                    font_size: FONT_SIZE,
                    actions: Vec::from([ElementAction::ChangeSetting(SettingOp::SetResolution(
                        width, height,
                    ))]),
                    ..default()
                }),
            );
        }
    }
    elements.insert("resolution_text".into());
    element_map.0.insert(
        "resolution_text".into(),
        Element::Text(TextElement {
            content: "You can set any resolution in settings.toml file".into(),
            position: Vec3::new(0., -240., RESOLUTION_Z_INDEX),
            ..default()
        }),
    );
    view_map
        .0
        .insert("settings_resolution".into(), View(elements));
}
