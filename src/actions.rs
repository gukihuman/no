use crate::{
    cameras::{CanvasScaleCover, CanvasScaleFit},
    elements::{spawn_element, ElementAction, ElementId, ElementMap, ViewStackOp},
    game_data::GameData,
    settings::GameSettings,
    steps::NextStepID,
    views::{ViewMap, ViewStack},
};
use bevy::prelude::*;
pub struct ClicksPlugin;
impl Plugin for ClicksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>()
            .add_systems(Update, handle_actions);
    }
}
#[derive(Resource, Default)]
pub struct Actions(pub Vec<ElementAction>);
fn handle_actions(
    mut actions: ResMut<Actions>,
    mut next_step: ResMut<NextStepID>,
    mut game_data: ResMut<GameData>,
    mut view_stack: ResMut<ViewStack>,
    view_map: Res<ViewMap>,
    element_map: Res<ElementMap>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    elements_query: Query<(Entity, &ElementId)>,
    canvas_scale_fit: Res<CanvasScaleFit>,
    canvas_scale_cover: Res<CanvasScaleCover>,
    settings: Res<GameSettings>,
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
                ViewStackOp::Push(view_id) => {
                    view_stack.0.push(view_id.clone());
                    if let Some(view) = view_map.0.get(&view_id) {
                        for element_id in view.0.iter() {
                            if let Some(element) = element_map.0.get(element_id) {
                                spawn_element(
                                    &mut commands,
                                    &asset_server,
                                    canvas_scale_fit.0,
                                    canvas_scale_cover.0,
                                    element_id.clone(),
                                    element,
                                    &settings,
                                );
                            }
                        }
                    }
                }
                ViewStackOp::Pop() => {
                    if let Some(view_id) = view_stack.0.pop() {
                        if let Some(view) = view_map.0.get(&view_id) {
                            for element_id in view.0.iter() {
                                for (entity, id) in elements_query.iter() {
                                    if id.0 == *element_id {
                                        commands.entity(entity).despawn_recursive();
                                    }
                                }
                            }
                        }
                    }
                }
            },
        }
    }
}
