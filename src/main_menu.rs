use crate::{
    elements::{
        Element, ElementAction, ElementMap, ImageElement, ResourceOp, TextButtonElement,
        TextElement,
    },
    steps::{ElementSet, NextStepID, Step, StepMap},
};
use bevy::{prelude::*, sprite::Anchor::BottomRight};
pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(
    mut element_map: ResMut<ElementMap>,
    mut step_collection: ResMut<StepMap>,
    mut next_step: ResMut<NextStepID>,
) {
    element_map.0.insert(
        "main_menu_background".into(),
        Element::Image(ImageElement {
            path: "main_menu/background.png".into(),
            ..default()
        }),
    );
    element_map.0.insert(
        "main_menu_version".into(),
        Element::Text(TextElement {
            content: "version 0.1.0".into(),
            position: Vec3::new(246., -136., 1.),
            anchor: BottomRight,
            ..default()
        }),
    );
    element_map.0.insert(
        "main_menu_arrow".into(),
        Element::Image(ImageElement {
            path: "main_menu/arrow.png".into(),
            color: Color::srgba(1., 1., 1., 0.5),
            position: Vec3::new(0., 0., 1.),
            actions: Vec::from([
                ElementAction::ModifyResource("gold".into(), ResourceOp::SetValue(5)),
                ElementAction::ModifyResource("gold".into(), ResourceOp::Increment(10)),
                ElementAction::ModifyResource("gold".into(), ResourceOp::Decrement(5)),
            ]),
            ..default()
        }),
    );
    element_map.0.insert(
        "main_menu_button".into(),
        Element::TextButton(TextButtonElement {
            content: "Start".into(),
            path: "main_menu/button.png".into(),
            position: Vec3::new(0., 0., 1.),
            font_size: 20.,
            actions: Vec::from([ElementAction::ChangeStep("empty".into())]),
            ..default()
        }),
    );
    // step
    let mut elements = ElementSet::new();
    elements.insert("main_menu_background".into());
    elements.insert("main_menu_version".into());
    elements.insert("main_menu_arrow".into());
    elements.insert("main_menu_button".into());
    step_collection.0.insert("main_menu".into(), Step(elements));
    next_step.0 = "main_menu".into()
}
