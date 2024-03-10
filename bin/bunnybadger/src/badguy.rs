use bevy::prelude::*;
use rand::Rng;

use crate::{
    castle::CastleService,
    common::{
        AnimationIndices, AnimationTimer, Lifetime, Velocity, RESOLUTION_HEIGHT, RESOLUTION_WIDTH,
    },
};

#[derive(Component)]
pub struct BadGuy;

#[derive(Component)]
pub struct BadGuySpawner {
    pub timer: Timer,
}

pub struct BadGuyService {
    handle_height: f32,
    texture_handle: Handle<Image>,
}

impl BadGuyService {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        Self {
            handle_height: 29.,
            texture_handle: asset_server.load("images/badguy_sheet.png"),
        }
    }

    pub fn spawn_spawner(&mut self, commands: &mut Commands) {
        commands
            .spawn(TransformBundle { ..default() })
            .insert(BadGuySpawner {
                timer: Timer::from_seconds(1., TimerMode::Repeating),
            });
    }

    pub fn spawn(
        &mut self,
        commands: &mut Commands,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let min_y: i32 = (-RESOLUTION_HEIGHT / 2. + self.handle_height / 2.).floor() as i32;
        let max_y: i32 =
            min_y + 4 * CastleService::handle_size().y as i32 - self.handle_height.floor() as i32;
        let y = rand::thread_rng().gen_range(min_y..max_y);

        let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 29.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };

        commands
            .spawn((
                SpriteSheetBundle {
                    texture: self.texture_handle.clone(),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                    transform: Transform {
                        translation: Vec3::new(RESOLUTION_WIDTH / 2., y as f32, 0.),
                        ..default()
                    },
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
        asset_server: Res<AssetServer>,
        time: Res<Time>,
        mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let mut badguy_spawner = query.single_mut();
        badguy_spawner.timer.tick(time.delta());
        if badguy_spawner.timer.just_finished() {
            let mut badguy_service: BadGuyService = BadGuyService::new(&asset_server);
            badguy_service.spawn(&mut commands, &mut texture_atlas_layouts);
        }
    }
}
