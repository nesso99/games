use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_kira_audio::prelude::*;

use crate::{
    badguy::BadGuy,
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_BADGUY},
};

#[derive(Component)]
pub struct Castle;

pub struct CastleService {
    handle_width: f32,
    handle_height: f32,
    handle: Handle<Image>,
}

impl CastleService {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            handle_width: 109.,
            handle_height: 105.,
            handle: asset_server.load("images/castle.png"),
        }
    }

    /// Spawn the castles, left of the screen
    pub fn spawn(&mut self, commands: &mut Commands) {
        let start_x: f32 = -RESOLUTION_WIDTH / 2. + self.handle_width / 2.;
        let start_y: f32 = -RESOLUTION_HEIGHT / 2. + self.handle_height / 2.;

        for i in 0..4 {
            commands.spawn((
                Sprite::from_image(self.handle.clone()),
                Transform::from_xyz(start_x, start_y + i as f32 * self.handle_height, 0.),
                Castle,
            ));
        }
    }

    pub fn check_badguy_collisions(
        mut commands: Commands,
        castle_query: Query<(Entity, &Transform), With<Castle>>,
        badguy_query: Query<(Entity, &Transform), With<BadGuy>>,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
    ) {
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
                    commands.entity(badguy_entity).despawn();
                    audio
                        .play(asset_server.load("audios/explode.wav"))
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
