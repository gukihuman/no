use crate::{
    elements::{Element, ElementAction, ElementMap, ImageElement, TextButtonElement, ViewStackOp},
    steps::ElementSet,
    views::{View, ViewMap},
};
use bevy::prelude::*;
pub struct ViewSettingsPlugin;
impl Plugin for ViewSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
fn setup(mut element_map: ResMut<ElementMap>, mut view_map: ResMut<ViewMap>) {
    element_map.0.insert(
        "black_screen".into(),
        Element::Image(ImageElement {
            path: "black_screen.webp".into(),
            position: Vec3::new(0., 0., 50.),
            color: Color::srgba(1., 1., 1., 0.8),
            ..default()
        }),
    );
    element_map.0.insert(
        "settings_button_return".into(),
        Element::TextButton(TextButtonElement {
            content: "Return".into(),
            path: "button.webp".into(),
            position: Vec3::new(0., 40., 100.),
            font_size: 20.,
            actions: Vec::from([ElementAction::ChangeViewStack(ViewStackOp::Pop())]),
            ..default()
        }),
    );
    // view
    let mut elements = ElementSet::new();
    elements.insert("black_screen".into());
    elements.insert("settings_button_return".into());
    view_map.0.insert("settings".into(), View(elements));
}
