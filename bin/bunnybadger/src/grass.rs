use bevy::prelude::*;

use crate::{
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH},
    resources::GameAssets,
};

pub struct Grass;

impl Grass {
    pub const WIDTH: f32 = 100.0;
    pub const HEIGHT: f32 = 100.0;

    pub fn spawn(commands: &mut Commands, game_assets: &GameAssets) {
        let start_x = -RESOLUTION_WIDTH / 2.0 + Self::WIDTH / 2.0;
        let start_y = -RESOLUTION_HEIGHT / 2.0 + Self::HEIGHT / 2.0;

        let columns = (RESOLUTION_WIDTH / Self::WIDTH).ceil() as i32;
        let rows = (RESOLUTION_HEIGHT / Self::HEIGHT).ceil() as i32;

        for yi in 0..rows {
            let y = start_y + yi as f32 * Self::HEIGHT;
            for xi in 0..columns {
                let x = start_x + xi as f32 * Self::WIDTH;
                commands.spawn((
                    Sprite::from_image(game_assets.grass_texture.clone()),
                    Transform::from_xyz(x, y, -1.0),
                ));
            }
        }
    }
}
