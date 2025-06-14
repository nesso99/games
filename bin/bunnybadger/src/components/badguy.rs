use bevy::prelude::*;
use rand::Rng;

use crate::{
    common::{
        AnimationIndices, AnimationTimer, Lifetime, Velocity, RESOLUTION_HEIGHT, RESOLUTION_WIDTH,
    },
    components::castle::CastleService,
    resources::GameAssets,
};

#[derive(Component)]
pub struct BadGuy;

#[derive(Component)]
pub struct BadGuySpawner {
    pub timer: Timer,
}

pub struct BadGuyComponent;

impl BadGuyComponent {
    pub const HEIGHT: f32 = 29.0;

    pub fn spawn_spawner(commands: &mut Commands) {
        commands.spawn(Transform::default()).insert(BadGuySpawner {
            timer: Timer::from_seconds(1., TimerMode::Repeating),
        });
    }

    // https://github.com/bevyengine/bevy/blob/v0.14.0/examples/2d/sprite_sheet.rs
    pub fn spawn(
        commands: &mut Commands,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
        game_assets: &Res<GameAssets>,
    ) {
        let min_y: i32 = (-RESOLUTION_HEIGHT / 2. + Self::HEIGHT / 2.).floor() as i32;
        let max_y: i32 =
            min_y + 4 * CastleService::handle_size().y as i32 - Self::HEIGHT.floor() as i32;
        let y = rand::rng().random_range(min_y..max_y);

        let layout = TextureAtlasLayout::from_grid(UVec2::new(64, 29), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };

        commands
            .spawn((
                Sprite::from_atlas_image(
                    game_assets.badguy_texture.clone(),
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                ),
                Transform {
                    translation: Vec3::new(RESOLUTION_WIDTH / 2., y as f32, 0.),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Velocity(Vec2::new(-1., 0.).normalize() * 300.),
                Lifetime(4.),
            ))
            .insert(BadGuy);
    }

    pub fn timer(
        mut query: Query<&mut BadGuySpawner>,
        mut commands: Commands,
        time: Res<Time>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
        game_assets: Res<GameAssets>,
    ) {
        let mut badguy_spawner = query.single_mut().unwrap();
        badguy_spawner.timer.tick(time.delta());
        if badguy_spawner.timer.just_finished() {
            Self::spawn(&mut commands, &mut texture_atlas_layouts, &game_assets);
        }
    }
}
