use std::sync::atomic::{AtomicBool, Ordering};

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::common::{apply_animate_sprite, apply_lifetime, apply_velocity, MainCamera};
use crate::components::arrow::{ArrowService, ShootTimer};
use crate::components::badguy::BadGuyService;
use crate::components::castle::CastleService;
use crate::components::dude::DudeService;
use crate::components::grass::GrassComponent;
use crate::components::healthbar::HealthBarService;
use crate::plugins::start_screen::GameState;
use crate::resources::GameAssets;
use crate::systems::in_game_menu::handle_ingame_menu;

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
                    handle_ingame_menu,
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

pub static IS_GAME_STARTED: AtomicBool = AtomicBool::new(false);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    if IS_GAME_STARTED.load(Ordering::Relaxed) {
        return;
    }

    let game_assets = GameAssets::new(&asset_server);
    commands.insert_resource(game_assets.clone());

    if camera_query.is_empty() {
        commands.spawn((Camera2d, MainCamera));
    }

    GrassComponent::spawn(&mut commands, &game_assets);
    let mut castle_service = CastleService::new();
    castle_service.spawn(&mut commands, &game_assets);
    DudeService::spawn(&mut commands, &game_assets);
    BadGuyService::spawn_spawner(&mut commands);
    HealthBarService::spawn(&mut commands, &game_assets);

    audio.play(game_assets.background_music).looped();

    IS_GAME_STARTED.store(true, Ordering::Relaxed);
}
