use bevy::prelude::*;

pub mod behaviours;
pub mod camera;

pub struct FeaturePlugin;
impl Plugin for FeaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, behaviours::BehaviourPlugin));
    }
}
