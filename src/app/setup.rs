use bevy::prelude::*;

use crate::app::{AppPlugin, config};

pub fn setup_game() {
    let mut app = App::new();

    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: config::TITLE.into(),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    app.add_plugins(AppPlugin);
    app.insert_resource(ClearColor(config::CLEAR_COLOR));

    app.run();
}
