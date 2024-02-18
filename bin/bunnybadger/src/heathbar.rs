use bevy::prelude::*;

use crate::common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_HEALTH_BAR};

pub struct HealthBarSevice<'a, 'w, 's> {
    healthbar_handle: Handle<Image>,
    health_handle: Handle<Image>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's> HealthBarSevice<'a, 'w, 's> {
    pub fn new(asset_server: &Res<AssetServer>, commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            healthbar_handle: asset_server.load("images/healthbar.png"),
            health_handle: asset_server.load("images/health.png"),
            commands,
        }
    }

    pub fn spawn(&mut self) {
        let health_bar_pos = Vec2::new(
            -RESOLUTION_WIDTH / 2. + SIZE_HEALTH_BAR.x / 2.,
            RESOLUTION_HEIGHT / 2. - SIZE_HEALTH_BAR.y / 2.,
        );

        self.commands.spawn(SpriteBundle {
            texture: self.healthbar_handle.clone(),
            transform: Transform::from_xyz(health_bar_pos.x, health_bar_pos.y, 0.),
            ..default()
        });

        for point in 0..200 {
            if point == 0 || point == 199 {
                continue;
            }

            self.commands.spawn(SpriteBundle {
                texture: self.health_handle.clone(),
                transform: Transform::from_xyz(
                    health_bar_pos.x - 100.0 + point as f32,
                    health_bar_pos.y,
                    0.,
                ),
                ..default()
            });
        }
    }
}
