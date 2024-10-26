use crate::{
    elements::{Element, ElementCollection, ImageElement, TextElement},
    steps::{ElementSet, NextStep, Step, StepCollection},
};
use bevy::prelude::*;
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(
    mut element_collection: ResMut<ElementCollection>,
    mut step_collection: ResMut<StepCollection>,
    mut next_step: ResMut<NextStep>,
) {
    element_collection.elements.insert(
        "bg_main_menu".to_string(),
        Element::Image(ImageElement {
            path: "bg_main_menu.png".to_string(),
            ..default()
        }),
    );
    element_collection.elements.insert(
        "text_main_menu".to_string(),
        Element::Text(TextElement {
            content: "MAIN MENU".to_string(),
            position: Vec3::new(0., 300., 1.),
            font_size: 40.,
        }),
    );
    let mut elements = ElementSet::new();
    elements.insert("bg_main_menu".to_string());
    elements.insert("text_main_menu".to_string());
    step_collection
        .steps
        .insert("main_menu".to_string(), Step { elements });
    next_step.id = "main_menu".to_string()
}
