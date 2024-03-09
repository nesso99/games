use bevy::prelude::*;

use crate::common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};

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
            commands.spawn(SpriteBundle {
                texture: self.handle.clone(),
                transform: Transform::from_xyz(
                    start_x,
                    start_y + i as f32 * self.handle_height,
                    0.,
                ),
                ..default()
            });
        }
    }

    pub fn handle_height() -> f32 {
        105.
    }
}
