pub mod config;
pub mod input;
pub mod setup;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{app::input::InputActions, camera, entities, scenes, utils};

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            entities::EntityPlugin,
            scenes::ScenePlugin,
            utils::UtilityPlugin,
            InputManagerPlugin::<InputActions>::default(),
        ));

        app.add_systems(Startup, input::setup_input);
        app.add_systems(PreUpdate, input::quick_exit);
    }
}
