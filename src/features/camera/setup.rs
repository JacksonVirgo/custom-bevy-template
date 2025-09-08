use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

use crate::app::config::GameConfig;

#[derive(Component)]
pub struct CameraController;

pub fn setup_camera(
    mut commands: Commands,
    windows: Query<&Window, With<PrimaryWindow>>,
    config: Res<GameConfig>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let aspect_ratio = window.width() / window.height();

    let viewport_height = config.calc_viewport_height();
    let viewport_width = viewport_height * aspect_ratio;

    commands.spawn((
        Camera2d { ..default() },
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: viewport_height,
            },
            ..OrthographicProjection::default_2d()
        }),
        CameraController,
        Transform::from_xyz(viewport_width / 2.0, -viewport_height / 2.0, 999.0),
        GlobalTransform::default(),
    ));
}
