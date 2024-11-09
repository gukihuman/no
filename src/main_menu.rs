use crate::{
    elements::{
        ConfirmAction, Element, ElementAction, ElementMap, GameDataOp, ImageElement, TextElement,
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
const Z_INDEX: f32 = 1.;
const FONT_SIZE_BUTTON: f32 = 48.;
const OPACITY: f32 = 0.95;
const X_COLUMN: f32 = -575.;
const Y_1_ROW: f32 = 0.;
const Y_2_ROW: f32 = -115.;
const Y_3_ROW: f32 = -230.;
const X_EDGE: f32 = 1240.;
const Y_EDGE: f32 = 680.;
fn setup(
    mut element_map: ResMut<ElementMap>,
    mut step_map: ResMut<StepMap>,
    mut next_step: ResMut<NextStepID>,
) {
    let mut elements = ElementSet::new();
    elements.insert("chapter_01_initial".into());
    element_map.0.insert(
        "chapter_01_initial".into(),
        Element::Image(ImageElement {
            path: "chapter_01/initial.webp".into(),
            ..default()
        }),
    );
    elements.insert("main_menu_logo".into());
    element_map.0.insert(
        "main_menu_logo".into(),
        Element::Image(ImageElement {
            path: "main_menu_logo.webp".into(),
            position: Vec3::new(0., 0., Z_INDEX),
            only_fit_scale: true,
            ..default()
        }),
    );
    elements.insert("main_menu_version".into());
    element_map.0.insert(
        "main_menu_version".into(),
        Element::Text(TextElement {
            content: "version 0.1.0".into(),
            position: Vec3::new(X_EDGE, -Y_EDGE, Z_INDEX),
            anchor: BottomRight,
            ..default()
        }),
    );
    elements.insert("test_arrow".into());
    element_map.0.insert(
        "test_arrow".into(),
        Element::Image(ImageElement {
            path: "wip/test_arrow.webp".into(),
            color: Color::srgba(1., 1., 1., 0.5),
            position: Vec3::new(0., 0., Z_INDEX),
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
            image_color: Color::srgba(1., 1., 1., OPACITY),
            position: Vec3::new(X_COLUMN, Y_1_ROW, Z_INDEX),
            font_size: FONT_SIZE_BUTTON,
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
            image_color: Color::srgba(1., 1., 1., OPACITY),
            position: Vec3::new(X_COLUMN, Y_2_ROW, Z_INDEX),
            font_size: FONT_SIZE_BUTTON,
            actions: Vec::from([ElementAction::ChangeViewStack(ViewStackOp::Push(
                "settings_root".into(),
            ))]),
            ..default()
        }),
    );
    elements.insert("main_menu_button_exit".into());
    element_map.0.insert(
        "main_menu_button_exit".into(),
        Element::TextImage(TextImageElement {
            content: "Exit".into(),
            path: "button.webp".into(),
            image_color: Color::srgba(1., 1., 1., OPACITY),
            position: Vec3::new(X_COLUMN, Y_3_ROW, Z_INDEX),
            font_size: FONT_SIZE_BUTTON,
            actions: Vec::from([ElementAction::Confirm(ConfirmAction::ExitApp)]),
            ..default()
        }),
    );
    step_map.0.insert("main_menu".into(), Step(elements));
    next_step.0 = "main_menu".into()
}
