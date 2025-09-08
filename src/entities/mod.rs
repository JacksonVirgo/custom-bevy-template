use bevy::prelude::*;

pub mod behaviours;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(behaviours::BehaviourPlugin);
    }
}
