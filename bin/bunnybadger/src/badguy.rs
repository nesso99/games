use bevy::prelude::*;
use rand::Rng;

use crate::common::{
    AnimationIndices, AnimationTimer, Lifetime, Velocity, RESOLUTION_HEIGHT, SIZE_BADGUY,
};

#[derive(Component)]
pub struct BadGuy;

#[derive(Component)]
pub struct BadGuySpawner {
    pub timer: Timer,
}

pub struct BadGuyService<'a, 'w, 's, 'ta> {
    texture_handle: Handle<Image>,
    commands: &'a mut Commands<'w, 's>,
    texture_atlases: &'a mut ResMut<'ta, Assets<TextureAtlas>>,
}

impl<'a, 'w, 's, 'ta> BadGuyService<'a, 'w, 's, 'ta> {
    pub fn new(
        asset_server: &Res<AssetServer>,
        commands: &'a mut Commands<'w, 's>,
        texture_atlases: &'a mut ResMut<'ta, Assets<TextureAtlas>>,
    ) -> Self {
        Self {
            texture_handle: asset_server.load("images/badguy_sheet.png"),
            commands,
            texture_atlases,
        }
    }

    pub fn spawn_spawner(&mut self) {
        self.commands
            .spawn(TransformBundle { ..default() })
            .insert(BadGuySpawner {
                timer: Timer::from_seconds(1., TimerMode::Repeating),
            });
    }

    pub fn spawn(&mut self) {
        let bound_size =
            (RESOLUTION_HEIGHT / 2.).floor() as i32 - (SIZE_BADGUY.y / 2.).floor() as i32;
        let y = rand::thread_rng().gen_range(-bound_size..bound_size);

        let texture_atlas = TextureAtlas::from_grid(
            self.texture_handle.clone(),
            Vec2::new(64.0, 29.0),
            4,
            1,
            None,
            None,
        );
        let texture_atlas_handle = self.texture_atlases.add(texture_atlas);
        let animation_indices = AnimationIndices { first: 0, last: 3 };

        self.commands
            .spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform {
                        translation: Vec3::new(320., y as f32, 0.),
                        ..default()
                    },
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Velocity(Vec2::new(-1., 0.).normalize() * 300.),
                Lifetime(3.),
            ))
            .insert(BadGuy);
    }

    pub fn timer(
        mut query: Query<&mut BadGuySpawner>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        time: Res<Time>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let mut badguy_spawner = query.single_mut();
        badguy_spawner.timer.tick(time.delta());
        if badguy_spawner.timer.just_finished() {
            let mut badguy_service =
                BadGuyService::new(&asset_server, &mut commands, &mut texture_atlases);
            badguy_service.spawn();
        }
    }
}