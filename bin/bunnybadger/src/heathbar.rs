use bevy::{prelude::*, render::render_resource::encase::private::Length};

use crate::{
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_HEALTH, SIZE_HEALTH_BAR},
    dude::Dude,
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

    pub fn update(
        dude_query: Query<&Dude>,
        health_point_query: Query<(Entity, &Transform), With<HealthPoint>>,
        mut commands: Commands,
    ) {
        let dude = dude_query.single();

        // Sort health points by x position to remove them from right to left
        let mut health_points: Vec<(Entity, &Transform)> = health_point_query.iter().collect();
        health_points.sort_by(|a, b| b.1.translation.x.partial_cmp(&a.1.translation.x).unwrap());

        let current_health = health_points.length();
        let points_to_remove = current_health.saturating_sub(dude.health as usize);

        // Remove excess health points
        for (entity, _) in health_points.iter().take(points_to_remove) {
            if let Some(mut entity) = commands.get_entity(*entity) {
                entity.despawn();
            }
        }
    }
}
