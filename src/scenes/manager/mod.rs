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

pub fn warn_persist_on_child(
    q: Query<
        (Entity, Option<&Name>, Option<&ChildOf>),
        (
            With<ScenePersist>,
            Or<(Added<ScenePersist>, Added<ChildOf>)>,
        ),
    >,
) {
    for (e, name, child_of) in &q {
        if let Some(p) = child_of {
            warn!(
                "ScenePersist attached to CHILD {:?}{} (parent {:?}). \
                 Scene cleanup only respects roots; tag the root instead.",
                e,
                name.map(|n| format!(" \"{n}\"")).unwrap_or_default(),
                p.parent()
            );
        }
    }
}
