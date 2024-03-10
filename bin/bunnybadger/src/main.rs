use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bunnybadger::{
    arrow::ArrowService,
    badguy::BadGuyService,
    castle::CastleService,
    common::{
        apply_animate_sprite, apply_lifetime, apply_velocity, MainCamera, RESOLUTION_HEIGHT,
        RESOLUTION_WIDTH,
    },
    dude::DudeService,
    grass::GrassSevice,
    heathbar::HealthBarSevice,
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
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                apply_animate_sprite,
                apply_velocity,
                apply_lifetime,
                ArrowService::mouse_button_input,
                ArrowService::check_for_collisions,
                DudeService::update,
                BadGuyService::timer,
                CastleService::check_badguy_collisions,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let mut grass_service = GrassSevice::new(&asset_server);
    grass_service.spawn(&mut commands);
    let mut castle_service = CastleService::new(&asset_server);
    castle_service.spawn(&mut commands);
    let mut dude_service = DudeService::new(&asset_server);
    dude_service.spawn(&mut commands);
    let mut bad_guy_service = BadGuyService::new(&asset_server);
    bad_guy_service.spawn_spawner(&mut commands);
    let mut healthbar_service = HealthBarSevice::new(&asset_server);
    healthbar_service.spawn(&mut commands);

    audio
        .play(asset_server.load("audios/moonlight.wav"))
        .looped();
}
