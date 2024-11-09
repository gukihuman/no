use crate::{
    cameras::CanvasScale,
    elements::{spawn_element, ElementId, ElementMap},
    settings::GameSettings,
};
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
pub struct StepsPlugin;
impl Plugin for StepsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepMap(HashMap::from([(
            "empty".into(),
            Step(HashSet::new()),
        )])))
        .insert_resource(CurrentStepID("empty".into()))
        .insert_resource(NextStepID("empty".into()))
        .add_systems(Update, on_step_change);
    }
}
pub type ElementSet = HashSet<String>;
pub struct Step(pub ElementSet);
#[derive(Resource)]
pub struct StepMap(pub HashMap<String, Step>);
#[derive(Resource)]
pub struct CurrentStepID(pub String);
#[derive(Resource)]
pub struct NextStepID(pub String);
fn on_step_change(
    mut commands: Commands,
    mut current_step_id: ResMut<CurrentStepID>,
    next_step_id: Res<NextStepID>,
    elements_query: Query<(Entity, &ElementId)>,
    asset_server: Res<AssetServer>,
    canvas_scale: Res<CanvasScale>,
    element_map: Res<ElementMap>,
    step_map: Res<StepMap>,
    settings: Res<GameSettings>,
) {
    if current_step_id.0 == next_step_id.0 {
        return;
    }
    if let Some(current_step) = step_map.0.get(&current_step_id.0) {
        if let Some(next_step) = step_map.0.get(&next_step_id.0) {
            let to_spawn: ElementSet = next_step.0.difference(&current_step.0).cloned().collect();
            let to_despawn: ElementSet = current_step.0.difference(&next_step.0).cloned().collect();
            for element_id in to_despawn {
                for (entity, id) in elements_query.iter() {
                    if id.0 == element_id {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
            for element_id in to_spawn {
                if let Some(element) = element_map.0.get(&element_id) {
                    spawn_element(
                        &mut commands,
                        &asset_server,
                        canvas_scale.fit,
                        canvas_scale.cover,
                        element_id,
                        element,
                        &settings,
                    );
                }
            }
        }
    }
    current_step_id.0 = next_step_id.0.clone()
}
