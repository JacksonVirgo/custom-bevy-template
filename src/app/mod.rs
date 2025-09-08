pub mod config;
pub mod input;
pub mod setup;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;

use crate::{app::input::InputActions, features, scenes, utils};

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            scenes::ScenePlugin,
            utils::UtilityPlugin,
            features::FeaturePlugin,
            InputManagerPlugin::<InputActions>::default(),
        ));

        app.add_systems(Startup, input::setup_input);
        app.add_systems(PreUpdate, input::quick_exit);
    }
}
