use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_kira_audio::prelude::*;

use crate::{
    common::{Lifetime, Velocity, SIZE_ARROW, SIZE_BADGUY},
    components::badguy::BadGuy,
    components::dude::Dude,
    resources::GameAssets,
};

/// Marker component for arrows shot by the player.
#[derive(Component)]
pub struct Arrow;

/// Systems for spawning arrows, handling input, and detecting collisions.
pub struct ArrowComponent;

impl ArrowComponent {
    pub const SHOOT_COOLDOWN_SECS: f32 = 0.3;

    /// Spawns a new arrow at the `dude`'s position and orientation.
    pub fn spawn(commands: &mut Commands, dude: &Dude, game_assets: &GameAssets) {
        let movement_direction = dude.rotation * Vec3::X;
        commands.spawn((
            Sprite::from_image(game_assets.arrow_texture.clone()),
            Transform {
                translation: dude.coords.extend(0.),
                rotation: dude.rotation,
                ..default()
            },
            Velocity(movement_direction.truncate().normalize() * 500.),
            Lifetime(3.),
            Arrow,
        ));
    }

    pub fn check_for_collisions(
        mut commands: Commands,
        arrow_query: Query<(Entity, &Transform), With<Arrow>>,
        badguy_query: Query<(Entity, &Transform), With<BadGuy>>,
        audio: Res<Audio>,
        game_assets: Res<GameAssets>,
    ) {
        // For better performance with many objects, consider spatial partitioning.
        // Check each arrow against each bad guy for collisions.
        for (arrow_entity, arrow_transform) in arrow_query.iter() {
            let arrow_aabb = Aabb2d::new(arrow_transform.translation.truncate(), SIZE_ARROW / 2.0);
            for (badguy_entity, badguy_transform) in badguy_query.iter() {
                let badguy_aabb =
                    Aabb2d::new(badguy_transform.translation.truncate(), SIZE_BADGUY / 2.0);
                if arrow_aabb.intersects(&badguy_aabb) {
                    if let Ok(mut arrow_ref) = commands.get_entity(arrow_entity) {
                        arrow_ref.despawn();
                    }
                    if let Ok(mut badguy_ref) = commands.get_entity(badguy_entity) {
                        badguy_ref.despawn();
                    }
                    audio.play(game_assets.enemy_sound.clone()).with_volume(0.5);
                }
            }
        }
    }

    /// Handles shooting input with left mouse button and enforces cooldown.
    pub fn mouse_button_input(
        buttons: Res<ButtonInput<MouseButton>>,
        query: Query<&Dude>,
        mut commands: Commands,
        audio: Res<Audio>,
        game_assets: Res<GameAssets>,
        time: Res<Time>,
        mut shoot_timer: ResMut<ShootTimer>,
    ) {
        shoot_timer.timer.tick(time.delta());
        if buttons.just_released(MouseButton::Left) && shoot_timer.timer.finished() {
            if let Ok(dude) = query.single() {
                Self::spawn(&mut commands, dude, &game_assets);
                audio.play(game_assets.shoot_sound.clone()).with_volume(0.5);
                shoot_timer.timer.reset();
            } else {
                warn!("Dude entity missing while trying to shoot arrow");
            }
        }
    }
}

/// Timer resource for controlling arrow shooting cooldown.
#[derive(Resource)]
pub struct ShootTimer {
    timer: Timer,
}

impl Default for ShootTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ArrowComponent::SHOOT_COOLDOWN_SECS, TimerMode::Once),
        }
    }
}
