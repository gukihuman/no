use crate::cameras::CanvasResource;
use crate::elements::{spawn_element, ElementCollection, ElementId};
use crate::settings::GameSettings;
use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
pub type ElementSet = HashSet<String>;
pub struct Step {
    pub elements: ElementSet,
}
#[derive(Resource)]
pub struct StepCollection {
    pub steps: HashMap<String, Step>,
}
#[derive(Resource)]
pub struct CurrentStep {
    pub id: String,
}
#[derive(Resource)]
pub struct NextStep {
    pub id: String,
}
pub struct StepsPlugin;
impl Plugin for StepsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StepCollection {
            steps: HashMap::from([(
                "empty".to_string(),
                Step {
                    elements: HashSet::new(),
                },
            )]),
        })
        .insert_resource(CurrentStep {
            id: "empty".to_string(),
        })
        .insert_resource(NextStep {
            id: "empty".to_string(),
        })
        .add_systems(Update, on_step_change);
    }
}
fn on_step_change(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut current_step_resource: ResMut<CurrentStep>,
    next_step_resource: Res<NextStep>,
    elements_query: Query<(Entity, &ElementId)>,
    asset_server: Res<AssetServer>,
    canvas: Res<CanvasResource>,
    element_collection: Res<ElementCollection>,
    step_collection: Res<StepCollection>,
    settings: Res<GameSettings>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        if let Some(current_step) = step_collection.steps.get(&current_step_resource.id) {
            if let Some(next_step) = step_collection.steps.get(&next_step_resource.id) {
                let current = &current_step.elements;
                let next = &next_step.elements;
                let to_spawn: ElementSet = next.difference(&current).cloned().collect();
                let to_despawn: ElementSet = current.difference(&next).cloned().collect();
                for element_id in to_despawn {
                    for (entity, id) in elements_query.iter() {
                        if id.0 == element_id {
                            commands.entity(entity).despawn();
                        }
                    }
                }
                for element_id in to_spawn {
                    if let Some(element) = element_collection.elements.get(&element_id) {
                        spawn_element(
                            &mut commands,
                            &asset_server,
                            canvas.scale,
                            element_id,
                            element,
                            &settings,
                        );
                    }
                }
            }
        }
        current_step_resource.id = next_step_resource.id.clone()
    }
}
