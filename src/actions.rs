use crate::{
    cameras::{CanvasResetRequest, CanvasScale},
    cursor::CursorResetRequest,
    elements::{
        spawn_element, ConfirmAction, Element, ElementAction, ElementId, ElementMap, SettingOp,
        ViewStackOp,
    },
    game_data::GameData,
    settings::{GameSettings, SETTINGS_PATH},
    steps::NextStepID,
    views::{ViewMap, ViewStack},
};
use bevy::{prelude::*, window::WindowMode};
use std::fs;
pub struct ClicksPlugin;
impl Plugin for ClicksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Actions(Vec::new()))
            .insert_resource(RestoreFullscreen(false))
            .add_systems(
                Update,
                (handle_actions, restore_fullscreen.before(handle_actions)),
            );
    }
}
#[derive(Resource)]
pub struct Actions(pub Vec<ElementAction>);
#[derive(Resource)]
pub struct RestoreFullscreen(bool);
pub fn handle_actions(
    mut commands: Commands,
    mut actions: ResMut<Actions>,
    mut next_step: ResMut<NextStepID>,
    mut game_data: ResMut<GameData>,
    mut view_stack: ResMut<ViewStack>,
    view_map: Res<ViewMap>,
    mut element_map: ResMut<ElementMap>,
    asset_server: Res<AssetServer>,
    elements_query: Query<(Entity, &ElementId)>,
    canvas_scale: Res<CanvasScale>,
    mut settings: ResMut<GameSettings>,
    mut query: Query<&mut Text>,
    parents_query: Query<(&Children, &ElementId)>,
    mut windows: Query<&mut Window>,
    mut restore_fullscreen_flag: ResMut<RestoreFullscreen>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for action in actions.0.drain(..) {
        match action {
            ElementAction::ChangeStep(step_id) => {
                next_step.0 = step_id;
            }
            ElementAction::ChangeGameData(field_name, op) => {
                game_data.change_field(&field_name, &op);
            }
            ElementAction::ChangeViewStack(op) => match op {
                ViewStackOp::Push(new_view) => {
                    if let Some(current_view) = view_stack.0.last() {
                        if let Some(view) = view_map.0.get(current_view) {
                            for element_id in view.0.iter() {
                                for (entity, id) in elements_query.iter() {
                                    if id.0 == *element_id {
                                        commands.entity(entity).despawn_recursive();
                                    }
                                }
                            }
                        }
                    }
                    view_stack.0.push(new_view.clone());
                    if let Some(view) = view_map.0.get(&new_view) {
                        for element_id in view.0.iter() {
                            if let Some(element) = element_map.0.get(element_id) {
                                spawn_element(
                                    &mut commands,
                                    &asset_server,
                                    canvas_scale.fit,
                                    canvas_scale.cover,
                                    element_id.clone(),
                                    element,
                                    &settings,
                                );
                            }
                        }
                    }
                }
                ViewStackOp::Pop() => {
                    if let Some(current_view) = view_stack.0.pop() {
                        if let Some(view) = view_map.0.get(&current_view) {
                            for element_id in view.0.iter() {
                                for (entity, id) in elements_query.iter() {
                                    if id.0 == *element_id {
                                        commands.entity(entity).despawn_recursive();
                                    }
                                }
                            }
                        }
                    }
                    if let Some(old_view) = view_stack.0.last() {
                        if let Some(view) = view_map.0.get(old_view) {
                            for element_id in view.0.iter() {
                                if let Some(element) = element_map.0.get(element_id) {
                                    spawn_element(
                                        &mut commands,
                                        &asset_server,
                                        canvas_scale.fit,
                                        canvas_scale.cover,
                                        element_id.clone(),
                                        element,
                                        &settings,
                                    );
                                }
                            }
                        }
                    }
                }
            },
            ElementAction::ChangeSetting(op) => match op {
                SettingOp::SetResolution(width, height) => {
                    settings.window.width = width;
                    settings.window.height = height;
                    for (children, element_id) in parents_query.iter() {
                        if element_id.0 == "settings_resolution" {
                            for &child in children.iter() {
                                if let Ok(mut text) = query.get_mut(child) {
                                    text.sections[0].value = format!(
                                        "{} x {}",
                                        settings.window.width, settings.window.height
                                    );
                                }
                            }
                        }
                    }
                    if let Some(element) = element_map.0.get_mut("settings_resolution") {
                        match element {
                            Element::TextImage(text_image) => {
                                text_image.content = format!(
                                    "{} x {}",
                                    settings.window.width, settings.window.height
                                );
                            }
                            _ => (),
                        }
                    }
                    if let Ok(toml_string) = toml::to_string_pretty(&settings.clone()) {
                        let _ = fs::write(SETTINGS_PATH, toml_string);
                    }
                    commands.spawn(CanvasResetRequest);
                    let mut window = windows.single_mut();
                    let was_fullscreen = window.mode == WindowMode::BorderlessFullscreen;
                    window.mode = WindowMode::Windowed;
                    if was_fullscreen {
                        restore_fullscreen_flag.0 = true;
                    }
                }
                SettingOp::ToggleWindowMode() => {
                    let mut window = windows.single_mut();
                    match settings.window.mode.as_str() {
                        "fullscreen" => {
                            settings.window.mode = "windowed".into();
                            window.resolution = Vec2::new(1280.0, 720.0).into();
                            window.mode = WindowMode::Windowed;
                        }
                        "windowed" => {
                            settings.window.mode = "fullscreen".into();
                            window.mode = WindowMode::BorderlessFullscreen;
                        }
                        _ => (),
                    }
                    for (children, element_id) in parents_query.iter() {
                        if element_id.0 == "settings_window_mode" {
                            for &child in children.iter() {
                                if let Ok(mut text) = query.get_mut(child) {
                                    text.sections[0].value = settings.window.mode.clone()
                                }
                            }
                        }
                    }
                    if let Some(element) = element_map.0.get_mut("settings_window_mode") {
                        match element {
                            Element::TextImage(text_image) => {
                                text_image.content = settings.window.mode.clone()
                            }
                            _ => (),
                        }
                    }
                    if let Ok(toml_string) = toml::to_string_pretty(&settings.clone()) {
                        let _ = fs::write(SETTINGS_PATH, toml_string);
                    }
                    commands.spawn(CanvasResetRequest);
                }
                SettingOp::ToggleBackgroundImage() => {
                    match settings.window.background_image.as_str() {
                        "cover" => {
                            settings.window.background_image = "fit".into();
                        }
                        "fit" => {
                            settings.window.background_image = "cover".into();
                        }
                        _ => (),
                    }
                    for (children, element_id) in parents_query.iter() {
                        if element_id.0 == "settings_background_image" {
                            for &child in children.iter() {
                                if let Ok(mut text) = query.get_mut(child) {
                                    text.sections[0].value =
                                        settings.window.background_image.clone()
                                }
                            }
                        }
                    }
                    if let Some(element) = element_map.0.get_mut("settings_background_image") {
                        match element {
                            Element::TextImage(text_image) => {
                                text_image.content = settings.window.background_image.clone()
                            }
                            _ => (),
                        }
                    }
                    if let Ok(toml_string) = toml::to_string_pretty(&settings.clone()) {
                        let _ = fs::write(SETTINGS_PATH, toml_string);
                    }
                    commands.spawn(CanvasResetRequest);
                }
                SettingOp::ToggleCustomCursor() => {
                    match settings.other.custom_cursor {
                        true => {
                            settings.other.custom_cursor = false;
                        }
                        false => {
                            settings.other.custom_cursor = true;
                        }
                    }
                    for (children, element_id) in parents_query.iter() {
                        if element_id.0 == "settings_custom_cursor" {
                            for &child in children.iter() {
                                if let Ok(mut text) = query.get_mut(child) {
                                    text.sections[0].value = match settings.other.custom_cursor {
                                        true => "on".into(),
                                        false => "off".into(),
                                    }
                                }
                            }
                        }
                    }
                    if let Some(element) = element_map.0.get_mut("settings_custom_cursor") {
                        match element {
                            Element::TextImage(text_image) => {
                                text_image.content = match settings.other.custom_cursor {
                                    true => "on".into(),
                                    false => "off".into(),
                                }
                            }
                            _ => (),
                        }
                    }
                    if let Ok(toml_string) = toml::to_string_pretty(&settings.clone()) {
                        let _ = fs::write(SETTINGS_PATH, toml_string);
                    }
                    commands.spawn(CursorResetRequest);
                }
            },
            ElementAction::Confirm(confirm_action) => {
                match confirm_action {
                    ConfirmAction::ExitApp => {
                        if let Some(element) = element_map.0.get_mut("confirm_text") {
                            match element {
                                Element::Text(text) => {
                                    text.content =
                                        "Are you sure? Any unsaved changes will be lost."
                                            .to_string();
                                }
                                _ => (),
                            }
                        }
                        if let Some(element) = element_map.0.get_mut("confirm_button") {
                            match element {
                                Element::TextImage(text_image) => {
                                    text_image.actions = Vec::from([ElementAction::ExitApp]);
                                }
                                _ => (),
                            }
                        }
                    }
                }
                if let Some(current_view) = view_stack.0.last() {
                    if let Some(view) = view_map.0.get(current_view) {
                        for element_id in view.0.iter() {
                            for (entity, id) in elements_query.iter() {
                                if id.0 == *element_id {
                                    commands.entity(entity).despawn_recursive();
                                }
                            }
                        }
                    }
                }
                view_stack.0.push("confirm".to_string());
                if let Some(view) = view_map.0.get(&"confirm".to_string()) {
                    for element_id in view.0.iter() {
                        if let Some(element) = element_map.0.get(element_id) {
                            spawn_element(
                                &mut commands,
                                &asset_server,
                                canvas_scale.fit,
                                canvas_scale.cover,
                                element_id.clone(),
                                element,
                                &settings,
                            );
                        }
                    }
                }
            }
            ElementAction::ExitApp => {
                app_exit_events.send(AppExit::Success);
            }
        }
    }
}
fn restore_fullscreen(
    mut windows: Query<&mut Window>,
    mut restore_fullscreen_flag: ResMut<RestoreFullscreen>,
) {
    if restore_fullscreen_flag.0 {
        if let Ok(mut window) = windows.get_single_mut() {
            window.mode = WindowMode::BorderlessFullscreen;
        }
        restore_fullscreen_flag.0 = false;
    }
}
