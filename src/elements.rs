use crate::{interactions::Interactive, settings::GameSettings};
use bevy::{prelude::*, sprite::Anchor};
use std::collections::HashMap;
pub struct ElementsPlugin;
impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ElementMap(HashMap::new()));
    }
}
#[derive(Component)]
pub struct OnlyFitScale(pub bool);
#[derive(Clone)]
pub struct TextElement {
    pub content: String,
    pub position: Vec3,
    pub color: Color,
    pub font_size: f32,
    pub anchor: Anchor,
}
impl Default for TextElement {
    fn default() -> Self {
        Self {
            content: String::new(),
            position: Vec3::ZERO,
            color: Color::WHITE,
            font_size: 16.0,
            anchor: Anchor::Center,
        }
    }
}
#[derive(Component, Clone)]
pub struct TextImageElement {
    pub content: String,
    pub path: String,
    pub position: Vec3,
    pub font_size: f32,
    pub image_color: Color,
    pub text_color: Color,
    pub actions: Vec<ElementAction>,
}
impl Default for TextImageElement {
    fn default() -> Self {
        Self {
            content: String::new(),
            path: String::new(),
            position: Vec3::ZERO,
            font_size: 16.0,
            image_color: Color::WHITE,
            text_color: Color::WHITE,
            actions: Vec::new(),
        }
    }
}
#[derive(Clone)]
pub struct ImageElement {
    pub path: String,
    pub position: Vec3,
    pub actions: Vec<ElementAction>,
    pub color: Color,
}
impl Default for ImageElement {
    fn default() -> Self {
        Self {
            path: String::new(),
            position: Vec3::ZERO,
            actions: Vec::new(),
            color: Color::WHITE,
        }
    }
}
#[derive(Component, Clone)]
pub enum ElementAction {
    ChangeStep(String),
    ChangeGameData(String, GameDataOp),
    ChangeViewStack(ViewStackOp),
    ChangeSetting(SettingOp),
}
#[derive(Clone)]
pub enum GameDataOp {
    Increment(i32),
    Decrement(i32),
    SetValue(i32),
}
#[derive(Clone)]
pub enum ViewStackOp {
    Push(String),
    Pop(),
}
#[derive(Clone)]
pub enum SettingOp {
    SetResolution(u32, u32),
    ToggleWindowMode(),
    ToggleBackgroundImage(),
    ToggleCustomCursor(),
}
#[derive(Component, Clone)]
pub enum Element {
    Text(TextElement),
    Image(ImageElement),
    TextImage(TextImageElement),
}
#[derive(Component)]
pub struct ElementId(pub String);
#[derive(Resource)]
pub struct ElementMap(pub HashMap<String, Element>);
pub fn spawn_element(
    commands: &mut Commands,
    asset_server: &AssetServer,
    canvas_scale_fit: f32,
    canvas_scale_cover: f32,
    element_id: String,
    element: &Element,
    settings: &GameSettings,
) {
    let canvas_scale = match settings.window.background_image.as_str() {
        "fit" => canvas_scale_fit,
        "cover" => canvas_scale_cover,
        _ => canvas_scale_fit,
    };
    let font_handle = asset_server.load("OpenSans-SemiBold.ttf");
    match element {
        Element::Text(text) => {
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        text.content.clone(),
                        TextStyle {
                            font: font_handle,
                            font_size: text.font_size * canvas_scale_fit,
                            color: text.color,
                            ..default()
                        },
                    ),
                    text_anchor: text.anchor,
                    transform: Transform::from_translation(text.position * canvas_scale_fit),
                    ..default()
                },
                OnlyFitScale(true),
                ElementId(element_id.into()),
            ));
        }
        Element::Image(image) => {
            let mut entity_commands = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: image.color,
                        ..default()
                    },
                    texture: asset_server.load(&image.path),
                    transform: Transform::from_scale(Vec3::new(canvas_scale, canvas_scale, 1.))
                        .with_translation(image.position),
                    ..default()
                },
                ElementId(element_id.into()),
            ));
            if !image.actions.is_empty() {
                entity_commands.insert(Interactive {
                    color: image.color.clone(),
                    actions: image.actions.clone(),
                    ..default()
                });
                entity_commands.insert(OnlyFitScale(false));
            }
        }
        Element::TextImage(text_button) => {
            let mut entity_commands = commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: text_button.image_color,
                        ..default()
                    },
                    texture: asset_server.load(&text_button.path),
                    transform: Transform::from_scale(Vec3::new(
                        canvas_scale_fit,
                        canvas_scale_fit,
                        1.,
                    ))
                    .with_translation(text_button.position * canvas_scale_fit),
                    ..default()
                },
                ElementId(element_id),
            ));
            if !text_button.actions.is_empty() {
                entity_commands.insert(Interactive {
                    color: text_button.image_color.clone(),
                    actions: text_button.actions.clone(),
                    ..default()
                });
                entity_commands.insert(OnlyFitScale(true));
            }
            entity_commands.with_children(|parent| {
                parent.spawn(Text2dBundle {
                    text: Text::from_section(
                        text_button.content.clone(),
                        TextStyle {
                            font: font_handle,
                            font_size: text_button.font_size * canvas_scale_fit,
                            color: text_button.text_color,
                            ..default()
                        },
                    ),
                    transform: Transform::from_scale(Vec3::new(
                        1. / canvas_scale_fit,
                        1. / canvas_scale_fit,
                        1.,
                    ))
                    .with_translation(Vec3::new(
                        0.,
                        0.,
                        text_button.position.z,
                    )),
                    ..default()
                });
            });
        }
    }
}
