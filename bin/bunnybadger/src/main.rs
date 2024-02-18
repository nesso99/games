use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bunnybadger::{
    arrow::ArrowService,
    badguy::BadGuyService,
    castle::CastleService,
    common::{
        animate_sprite, apply_lifetime, apply_velocity, MainCamera, RESOLUTION_HEIGHT,
        RESOLUTION_WIDTH,
    },
    dude::DudeService,
    grass::GrassSevice,
    heathbar::HealthBarSevice,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "bunnybadger".into(),
                    resolution: (RESOLUTION_WIDTH, RESOLUTION_HEIGHT).into(),
                    resizable: false,
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
                animate_sprite,
                apply_velocity,
                apply_lifetime,
                ArrowService::mouse_button_input,
                ArrowService::check_for_collisions,
                DudeService::update,
                BadGuyService::timer,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    audio: Res<Audio>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // commands.spawn(AudioBundle {
    //     source: asset_server.load("audios/enemy.wav"),
    //     ..default()
    // });

    let mut grass_service = GrassSevice::new(&asset_server, &mut commands);
    grass_service.spawn();
    let mut castle_service = CastleService::new(&asset_server, &mut commands);
    castle_service.spawn();
    let mut dude_service = DudeService::new(&asset_server, &mut commands);
    dude_service.spawn();
    let mut bad_guy_service =
        BadGuyService::new(&asset_server, &mut commands, &mut texture_atlas_layouts);
    bad_guy_service.spawn_spawner();
    let mut healthbar_service = HealthBarSevice::new(&asset_server, &mut commands);
    healthbar_service.spawn();

    audio
        .play(asset_server.load("audios/moonlight.wav"))
        .looped();
}
