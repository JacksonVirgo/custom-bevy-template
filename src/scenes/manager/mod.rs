use bevy::{ecs::system::ScheduleSystem, prelude::*};

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Copy)]
pub enum GameScene {
    #[default]
    Default,
}

// Attach to anything that should not despawn on a scene change.
#[derive(Component)]
pub struct ScenePersist;

fn scene_cleanup(
    mut commands: Commands,
    roots: Query<Entity, (Without<ScenePersist>, Without<ChildOf>)>,
) {
    for e in &roots {
        commands.entity(e).despawn();
    }
}

pub fn add_scene<M>(
    app: &mut App,
    scene: GameScene,
    systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
) {
    app.add_systems(OnEnter(scene), systems);
    app.add_systems(OnExit(scene), scene_cleanup);
}
