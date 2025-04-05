use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_kira_audio::prelude::*;

use crate::{
    badguy::BadGuy,
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_BADGUY},
    dude::Dude,
    resources::GameAssets,
};

#[derive(Component)]
pub struct Castle;

pub struct CastleService {
    handle_width: f32,
    handle_height: f32,
}

impl CastleService {
    pub fn new() -> Self {
        Self {
            handle_width: 109.,
            handle_height: 105.,
        }
    }

    /// Spawn the castles, left of the screen
    pub fn spawn(&mut self, commands: &mut Commands, game_asset: &GameAssets) {
        let start_x: f32 = -RESOLUTION_WIDTH / 2. + self.handle_width / 2.;
        let start_y: f32 = -RESOLUTION_HEIGHT / 2. + self.handle_height / 2.;

        let rows = (RESOLUTION_HEIGHT / self.handle_height).ceil() as i32;

        for i in 0..rows {
            commands.spawn((
                Sprite::from_image(game_asset.castle_texture.clone()),
                Transform::from_xyz(start_x, start_y + i as f32 * self.handle_height, -0.1),
                Castle,
            ));
        }
    }

    pub fn check_badguy_collisions(
        mut commands: Commands,
        castle_query: Query<(Entity, &Transform), With<Castle>>,
        badguy_query: Query<(Entity, &Transform), With<BadGuy>>,
        mut dude_query: Query<&mut Dude>,
        audio: Res<Audio>,
        game_asset: Res<GameAssets>,
    ) {
        let mut dude = dude_query.single_mut();
        for (badguy_entity, badguy_transform) in &badguy_query {
            for (_, castle_transform) in &castle_query {
                let intersects = Aabb2d::new(
                    castle_transform.translation.truncate(),
                    Self::handle_size() / 2.0,
                )
                .intersects(&Aabb2d::new(
                    badguy_transform.translation.truncate(),
                    SIZE_BADGUY / 2.0,
                ));

                if intersects {
                    if let Some(mut badguy_entity) = commands.get_entity(badguy_entity) {
                        badguy_entity.despawn();
                    }

                    dude.health = dude.health.saturating_sub(1);
                    audio
                        .play(game_asset.explode_sound.clone())
                        .with_volume(0.5);

                    // badguy hit the castle, so we don't need to check for more collisions
                    break;
                }
            }
        }
    }

    pub fn handle_size() -> Vec2 {
        Vec2::new(109., 105.)
    }
}

impl Default for CastleService {
    fn default() -> Self {
        Self::new()
    }
}
