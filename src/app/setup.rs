use bevy::prelude::*;

use crate::app::{AppPlugin, config::GameConfig};

pub fn setup_game() {
    let mut app = App::new();

    let cfg = GameConfig::default();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: cfg.title.into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins(AppPlugin);
    app.insert_resource(ClearColor(Color::BLACK));

    app.run();
}
