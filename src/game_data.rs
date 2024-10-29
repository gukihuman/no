use crate::elements::ResourceOp;
use bevy::prelude::*;
use std::collections::HashMap;
pub struct GameDataPlugin;
impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut App) {
        let mut game_data = GameData(HashMap::new());
        game_data.add("gold", 0);
        app.insert_resource(game_data);
    }
}
#[derive(Resource)]
pub struct GameData(pub HashMap<String, i32>);
impl GameData {
    fn add(&mut self, name: &str, value: i32) {
        self.0.insert(name.to_string(), value);
    }
    pub fn get(&self, name: &str) -> i32 {
        *self.0.get(name).unwrap_or(&0)
    }
    pub fn set(&mut self, name: &str, value: i32) {
        if self.0.contains_key(name) {
            self.0.insert(name.to_string(), value);
        } else {
            println!("Warning: Trying to set non-existent key '{}'", name);
        }
    }
    pub fn modify_resource(&mut self, name: &str, op: &ResourceOp) {
        let current = self.get(name);
        let new_value = match op {
            ResourceOp::Increment(value) => current + value,
            ResourceOp::Decrement(value) => current - value,
            ResourceOp::SetValue(value) => *value,
        };
        self.set(name, new_value);
    }
}
