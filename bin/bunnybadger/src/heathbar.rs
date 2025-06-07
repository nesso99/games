use bevy::prelude::*;

use crate::{
    common::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH, SIZE_HEALTH, SIZE_HEALTH_BAR},
    dude::Dude,
    resources::GameAssets,
};

/// Marker component for the health bar background.
#[derive(Component)]
pub struct HealthBar;

/// Marker component for a single health point segment.
#[derive(Component)]
pub struct HealthPoint;

/// Service for spawning and updating the health bar UI.
pub struct HealthBarService;

impl HealthBarService {
    /// Spawns the health bar background and segment points based on bar size and segment size.
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

        // Calculate how many health points fit in bar, leaving a border of 1 segment on each side
        let num_points = (SIZE_HEALTH_BAR.x / SIZE_HEALTH.x).floor() as usize;
        let start_idx = 1; // leave one segment as border
        let end_idx = num_points - 1; // leave one segment as border

        for point in start_idx..end_idx {
            let x = health_bar_pos.x - SIZE_HEALTH_BAR.x / 2.
                + SIZE_HEALTH.x / 2.
                + (point as f32) * SIZE_HEALTH.x;
            commands.spawn((
                Sprite::from_image(game_assets.health_texture.clone()),
                Transform::from_xyz(x, health_bar_pos.y, 1.),
                HealthPoint,
            ));
        }
    }

    /// Updates the visible health points to match the dude's current health.
    pub fn update(
        dude_query: Query<&Dude>,
        health_point_query: Query<(Entity, &Transform), With<HealthPoint>>,
        mut commands: Commands,
    ) {
        let dude = dude_query.single().expect("dude is empty");

        // Order health points by x; rightmost points will be removed first (simulate damage from right)
        let mut health_points: Vec<(Entity, &Transform)> = health_point_query.iter().collect();
        health_points.sort_by(|a, b| b.1.translation.x.partial_cmp(&a.1.translation.x).unwrap());

        let current_health = health_points.len();
        let dude_health = dude.health as usize;
        let points_to_remove = current_health.saturating_sub(dude_health);

        for (entity, _) in health_points.iter().take(points_to_remove) {
            if let Ok(mut entity) = commands.get_entity(*entity) {
                entity.despawn();
            }
        }
    }
}
