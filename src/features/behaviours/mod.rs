use bevy::prelude::*;

pub mod movement;

pub struct BehaviourPlugin;
impl Plugin for BehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, movement::handle_movement);
    }
}
