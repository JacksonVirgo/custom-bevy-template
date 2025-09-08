use bevy::prelude::*;

use crate::scenes::manager::{GameScene, add_scene};

pub mod default;
pub mod manager;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameScene>();
        add_scene(app, GameScene::Default, default::default_load);
    }
}
