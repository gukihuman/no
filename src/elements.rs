use crate::settings::GameSettings;
use bevy::prelude::*;
use std::collections::HashMap;
#[derive(Clone)]
pub struct TextElement {
    pub content: String,
    pub position: Vec3,
    pub font_size: f32,
}
impl Default for TextElement {
    fn default() -> Self {
        Self {
            content: String::new(),
            position: Vec3::ZERO,
            font_size: 24.0,
        }
    }
}
#[derive(Clone)]
pub struct ImageElement {
    pub path: String,
    pub position: Vec3,
}
impl Default for ImageElement {
    fn default() -> Self {
        Self {
            path: String::new(),
            position: Vec3::ZERO,
        }
    }
}
#[derive(Component, Clone)]
pub enum Element {
    Text(TextElement),
    Image(ImageElement),
}
#[derive(Component)]
pub struct ElementId(pub String);
#[derive(Resource)]
pub struct ElementCollection {
    pub elements: HashMap<String, Element>,
}
pub struct ElementsPlugin;
impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ElementCollection {
            elements: HashMap::new(),
        });
    }
}
pub fn spawn_element(
    commands: &mut Commands,
    asset_server: &AssetServer,
    canvas_scale: f32,
    element_id: String,
    element: &Element,
    settings: &Res<GameSettings>,
) {
    match element {
        Element::Image(image) => {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(&image.path),
                    transform: Transform::from_scale(Vec3::new(canvas_scale, canvas_scale, 1.))
                        .with_translation(image.position),
                    ..default()
                },
                element.clone(),
                ElementId(element_id.to_string()),
            ));
        }
        Element::Text(text) => {
            commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        text.content.clone(),
                        TextStyle {
                            font_size: text.font_size * settings.window.ui_scale,
                            ..default()
                        },
                    ),
                    transform: Transform::from_translation(text.position),
                    ..default()
                },
                element.clone(),
                ElementId(element_id.to_string()),
            ));
        }
    }
}
