use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameConfig {
    pub title: &'static str,
    pub unit_size: f32,
    pub viewport_height_units: f32,
}

impl GameConfig {
    pub fn default() -> Self {
        Self {
            title: "Game",
            unit_size: 48.0,
            viewport_height_units: 15.0,
        }
    }

    pub fn calc_viewport_height(&self) -> f32 {
        self.viewport_height_units * self.unit_size
    }
}
