use bevy::prelude::*;

use crate::{
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_HEALTH_BAR},
    resources::GameAssets,
};

pub struct HealthBarService;

impl HealthBarService {
    pub fn spawn(commands: &mut Commands, game_assets: &GameAssets) {
        let health_bar_pos = Vec2::new(
            -RESOLUTION_WIDTH / 2. + SIZE_HEALTH_BAR.x / 2.,
            RESOLUTION_HEIGHT / 2. - SIZE_HEALTH_BAR.y / 2.,
        );

        commands.spawn((
            Sprite::from_image(game_assets.healthbar_texture.clone()),
            Transform::from_xyz(health_bar_pos.x, health_bar_pos.y, 0.),
        ));

        // for point in 0..200 {
        //     if point == 0 || point == 199 {
        //         continue;
        //     }

        //     commands.spawn((
        //         Sprite::from_image(self.health_handle.clone()),
        //         Transform::from_xyz(
        //             health_bar_pos.x - 100.0 + point as f32,
        //             health_bar_pos.y,
        //             0.,
        //         ),
        //     ));
        // }
    }
}
