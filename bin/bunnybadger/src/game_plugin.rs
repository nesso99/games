use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{
    arrow::{ArrowService, ShootTimer},
    badguy::BadGuyService,
    castle::CastleService,
    common::{apply_animate_sprite, apply_lifetime, apply_velocity, MainCamera},
    dude::DudeService,
    grass::GrassService,
    heathbar::HealthBarService,
    resources::GameAssets,
    start_screen::GameState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>()
            .init_resource::<ShootTimer>()
            .add_systems(OnEnter(GameState::InGame), setup)
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
                    HealthBarService::update,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    let game_assets = GameAssets::new(&asset_server);
    commands.insert_resource(game_assets.clone());

    if camera_query.is_empty() {
        commands.spawn((Camera2d, MainCamera));
    }

    let mut grass_service = GrassService::new();
    grass_service.spawn(&mut commands, &game_assets);
    let mut castle_service = CastleService::new();
    castle_service.spawn(&mut commands, &game_assets);
    DudeService::spawn(&mut commands, &game_assets);
    BadGuyService::spawn_spawner(&mut commands);
    HealthBarService::spawn(&mut commands, &game_assets);

    audio.play(game_assets.background_music).looped();
}
