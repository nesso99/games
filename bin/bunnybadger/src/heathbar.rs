use bevy::prelude::*;

use crate::{
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_HEALTH, SIZE_HEALTH_BAR},
    resources::GameAssets,
};

#[derive(Component)]
pub struct HealthBar;

#[derive(Component)]
pub struct HealthPoint;

pub struct HealthBarService;

impl HealthBarService {
    pub fn spawn(commands: &mut Commands, game_assets: &GameAssets) {
        let health_bar_pos = Vec2::new(
            -RESOLUTION_WIDTH / 2. + SIZE_HEALTH_BAR.x / 2.,
            RESOLUTION_HEIGHT / 2. - SIZE_HEALTH_BAR.y / 2.,
        );

        // Spawn health bar background
        commands.spawn((
            Sprite::from_image(game_assets.healthbar_texture.clone()),
            Transform::from_xyz(health_bar_pos.x, health_bar_pos.y, 0.),
            HealthBar,
        ));

        // Spawn individual health points
        for point in 0..200 {
            if point == 0 || point == 199 {
                continue;
            }

            commands.spawn((
                // Sprite {
                //     color: Color::srgb(0., 1., 0.),
                //     custom_size: Some(SIZE_HEALTH),
                //     ..default()
                // },
                Sprite::from_image(game_assets.health_texture.clone()),
                Transform::from_xyz(
                    health_bar_pos.x - SIZE_HEALTH_BAR.x / 2. + SIZE_HEALTH.x / 2. + point as f32,
                    health_bar_pos.y,
                    1.,
                ),
                HealthPoint,
            ));
        }
    }
}
