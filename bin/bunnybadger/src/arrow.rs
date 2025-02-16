use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_kira_audio::prelude::*;

use crate::{
    badguy::BadGuy,
    common::{Lifetime, Velocity, SIZE_ARROW, SIZE_BADGUY},
    dude::Dude,
    resources::GameAssets,
};

#[derive(Component)]
pub struct Arrow;

pub struct ArrowService;

impl ArrowService {
    pub const SHOOT_COOLDOWN_SECS: f32 = 0.3;

    pub fn spawn(commands: &mut Commands, dude: &Dude, game_assets: &Res<GameAssets>) {
        let movement_direction = dude.rotation * Vec3::X;
        commands
            .spawn((
                Sprite::from_image(game_assets.arrow_texture.clone()),
                Transform {
                    translation: dude.coords.extend(0.),
                    rotation: dude.rotation,
                    ..default()
                },
                Velocity(movement_direction.truncate().normalize() * 500.),
                Lifetime(3.),
            ))
            .insert(Arrow {});
    }

    pub fn check_for_collisions(
        mut commands: Commands,
        arrow_query: Query<(Entity, &Transform), With<Arrow>>,
        badguy_query: Query<(Entity, &Transform), With<BadGuy>>,
        audio: Res<Audio>,
        game_assets: Res<GameAssets>,
    ) {
        for (arrow_entity, arrow_transform) in &arrow_query {
            for (badguy_entity, badguy_transform) in &badguy_query {
                let intersects =
                    Aabb2d::new(arrow_transform.translation.truncate(), SIZE_ARROW / 2.0)
                        .intersects(&Aabb2d::new(
                            badguy_transform.translation.truncate(),
                            SIZE_BADGUY / 2.0,
                        ));

                if intersects {
                    if let Some(mut arrow_entity) = commands.get_entity(arrow_entity) {
                        arrow_entity.despawn();
                    }
                    if let Some(mut badguy_entity) = commands.get_entity(badguy_entity) {
                        badguy_entity.despawn();
                    }

                    audio.play(game_assets.enemy_sound.clone()).with_volume(0.5);
                }
            }
        }
    }

    pub fn mouse_button_input(
        buttons: Res<ButtonInput<MouseButton>>,
        query: Query<&Dude>,
        mut commands: Commands,
        audio: Res<Audio>,
        game_assets: Res<GameAssets>,
        time: Res<Time>,
        mut shoot_timer: ResMut<ShootTimer>,
    ) {
        shoot_timer.timer.tick(time.delta());
        if buttons.just_released(MouseButton::Left) && shoot_timer.timer.finished() {
            let dude = query.single();
            Self::spawn(&mut commands, dude, &game_assets);
            audio.play(game_assets.shoot_sound.clone()).with_volume(0.5);
            shoot_timer.timer.reset();
        }
    }
}

#[derive(Resource)]
pub struct ShootTimer {
    timer: Timer,
}

impl Default for ShootTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ArrowService::SHOOT_COOLDOWN_SECS, TimerMode::Once),
        }
    }
}
