use crate::{
    elements::{
        Element, ElementAction, ElementMap, ImageElement, TextElement, TextImageElement,
        ViewStackOp,
    },
    steps::ElementSet,
    views::{View, ViewMap},
};
use bevy::prelude::*;
pub struct ConfirmPlugin;
impl Plugin for ConfirmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
const Z_INDEX: f32 = 300.;
const FONT_SIZE: f32 = 56.;
const FONT_SIZE_BUTTON: f32 = 48.;
const OPACITY: f32 = 0.6;
const RETURN_BUTTON_Y: f32 = -550.;
const TEXT_Y: f32 = 70.;
const CONFIRM_BUTTON_Y: f32 = -50.;
fn setup(mut element_map: ResMut<ElementMap>, mut view_map: ResMut<ViewMap>) {
    let mut elements = ElementSet::new();
    elements.insert("confirm_black_screen".into());
    element_map.0.insert(
        "confirm_black_screen".into(),
        Element::Image(ImageElement {
            path: "black_screen.webp".into(),
            position: Vec3::new(0., 0., Z_INDEX - 80.),
            color: Color::srgba(1., 1., 1., 0.98),
            ..default()
        }),
    );
    elements.insert("confirm_return_button".into());
    element_map.0.insert(
        "confirm_return_button".into(),
        Element::TextImage(TextImageElement {
            content: "Return".into(),
            path: "button.webp".into(),
            image_color: Color::srgba(1., 1., 1., OPACITY),
            position: Vec3::new(0., RETURN_BUTTON_Y, Z_INDEX),
            font_size: FONT_SIZE_BUTTON,
            actions: Vec::from([ElementAction::ChangeViewStack(ViewStackOp::Pop())]),
            ..default()
        }),
    );
    elements.insert("confirm_text".into());
    element_map.0.insert(
        "confirm_text".into(),
        Element::Text(TextElement {
            content: "".into(),
            position: Vec3::new(0., TEXT_Y, Z_INDEX),
            font_size: FONT_SIZE,
            ..default()
        }),
    );
    elements.insert("confirm_button".into());
    element_map.0.insert(
        "confirm_button".into(),
        Element::TextImage(TextImageElement {
            content: "Confirm".to_string(),
            path: "button.webp".into(),
            image_color: Color::srgba(1., 1., 1., OPACITY),
            position: Vec3::new(0., CONFIRM_BUTTON_Y, Z_INDEX),
            font_size: FONT_SIZE_BUTTON,
            ..default()
        }),
    );
    view_map.0.insert("confirm".into(), View(elements));
}
