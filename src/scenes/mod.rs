use bevy::prelude::*;

use crate::scenes::manager::{GameScene, add_scene};

pub mod default;
pub mod manager;

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, manager::warn_persist_on_child);
        app.init_state::<GameScene>();
        add_scene(app, GameScene::Default, default::default_load);
    }
}
