use bevy::prelude::*;

use crate::{
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH},
    resources::GameAssets,
};

pub struct GrassService {
    handle_width: f32,
    handle_height: f32,
}

impl GrassService {
    pub fn new() -> Self {
        Self {
            handle_width: 100.,
            handle_height: 100.,
        }
    }

    pub fn spawn(&mut self, commands: &mut Commands, game_assets: &GameAssets) {
        let start_x: f32 = -RESOLUTION_WIDTH / 2. + self.handle_width / 2.;
        let start_y: f32 = -RESOLUTION_HEIGHT / 2. + self.handle_height / 2.;
        for yi in 0..5 {
            let current_y = start_y + yi as f32 * self.handle_height;
            for xi in 0..7 {
                commands.spawn((
                    Sprite::from_image(game_assets.grass_texture.clone()),
                    Transform::from_xyz(start_x + xi as f32 * self.handle_width, current_y, -1.),
                ));
            }
        }
    }
}
