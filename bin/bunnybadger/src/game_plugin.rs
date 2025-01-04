use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    arrow::ArrowService,
    badguy::BadGuyService,
    castle::CastleService,
    common::{apply_animate_sprite, apply_lifetime, apply_velocity, MainCamera},
    dude::DudeService,
    grass::GrassService,
    heathbar::HealthBarService,
    resources::GameAssets,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
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
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let game_assets = GameAssets::new(&asset_server);
    commands.insert_resource(game_assets.clone());

    commands.spawn((Camera2d, MainCamera));

    let mut grass_service = GrassService::new();
    grass_service.spawn(&mut commands, &game_assets);
    let mut castle_service = CastleService::new();
    castle_service.spawn(&mut commands, &game_assets);
    DudeService::spawn(&mut commands, &game_assets);
    BadGuyService::spawn_spawner(&mut commands);
    HealthBarService::spawn(&mut commands, &game_assets);

    audio.play(game_assets.background_music).looped();
}
