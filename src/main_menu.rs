use crate::{
    elements::{
        Element, ElementAction, ElementMap, GameDataOp, ImageElement, TextElement,
        TextImageElement, ViewStackOp,
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
    mut step_map: ResMut<StepMap>,
    mut next_step: ResMut<NextStepID>,
) {
    let mut elements = ElementSet::new();
    elements.insert("main_menu_background".into());
    element_map.0.insert(
        "main_menu_background".into(),
        Element::Image(ImageElement {
            path: "main_menu/bunny_initial.webp".into(),
            ..default()
        }),
    );
    elements.insert("main_menu_version".into());
    element_map.0.insert(
        "main_menu_version".into(),
        Element::Text(TextElement {
            content: "version 0.1.0".into(),
            position: Vec3::new(470., -260., 1.),
            anchor: BottomRight,
            ..default()
        }),
    );
    elements.insert("main_menu_arrow".into());
    element_map.0.insert(
        "main_menu_arrow".into(),
        Element::Image(ImageElement {
            path: "main_menu/arrow.webp".into(),
            color: Color::srgba(1., 1., 1., 0.5),
            position: Vec3::new(0., 0., 1.),
            actions: Vec::from([
                ElementAction::ChangeGameData("gold".into(), GameDataOp::SetValue(5)),
                ElementAction::ChangeGameData("gold".into(), GameDataOp::Increment(10)),
                ElementAction::ChangeGameData("gold".into(), GameDataOp::Decrement(5)),
            ]),
            ..default()
        }),
    );
    elements.insert("main_menu_button_start".into());
    element_map.0.insert(
        "main_menu_button_start".into(),
        Element::TextImage(TextImageElement {
            content: "Start".into(),
            path: "button.webp".into(),
            position: Vec3::new(0., 0., 1.),
            font_size: 22.,
            actions: Vec::from([ElementAction::ChangeStep("empty".into())]),
            ..default()
        }),
    );
    elements.insert("main_menu_button_settings".into());
    element_map.0.insert(
        "main_menu_button_settings".into(),
        Element::TextImage(TextImageElement {
            content: "Settings".into(),
            path: "button.webp".into(),
            position: Vec3::new(0., -50., 1.),
            font_size: 22.,
            actions: Vec::from([ElementAction::ChangeViewStack(ViewStackOp::Push(
                "settings_root".into(),
            ))]),
            ..default()
        }),
    );
    step_map.0.insert("main_menu".into(), Step(elements));
    next_step.0 = "main_menu".into()
}
