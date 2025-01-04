use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bunnybadger::{
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH},
    game_plugin::GamePlugin,
};

fn main() {
    // disable log of bevy_kira_audio
    std::env::set_var("RUST_LOG", "bevy=info");

    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bunnybadger".into(),
                    resolution: (RESOLUTION_WIDTH, RESOLUTION_HEIGHT).into(),
                    resizable: false,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
            AudioPlugin,
            GamePlugin,
        ))
        .run();
}
