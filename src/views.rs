use crate::steps::ElementSet;
use bevy::prelude::*;
use std::collections::HashMap;
pub struct ViewsPlugin;
impl Plugin for ViewsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ViewMap(HashMap::new()))
            .insert_resource(ViewStack(Vec::new()));
        // .add_systems(Update, on_view_stack_change);
    }
}
#[derive()]
pub struct View(pub ElementSet);
#[derive(Resource)]
pub struct ViewMap(pub HashMap<String, View>);
#[derive(Resource)]
pub struct ViewStack(pub Vec<String>);
// fn on_view_stack_change() {}
