use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_kira_audio::prelude::*;

use crate::{
    badguy::BadGuy,
    common::{Lifetime, Velocity, SIZE_ARROW, SIZE_BADGUY},
    dude::Dude,
};

#[derive(Component)]
pub struct Arrow;

pub struct ArrowService<'a, 'w, 's> {
    dude: &'a Dude,
    texture: Handle<Image>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's> ArrowService<'a, 'w, 's> {
    pub fn new(
        dude: &'a Dude,
        asset_server: &Res<AssetServer>,
        commands: &'a mut Commands<'w, 's>,
    ) -> Self {
        Self {
            dude,
            texture: asset_server.load("images/bullet.png"),
            commands,
        }
    }

    pub fn spawn(&mut self) {
        let movement_direction = self.dude.rotation * Vec3::X;
        self.commands
            .spawn((
                SpriteBundle {
                    texture: self.texture.clone(),
                    transform: Transform {
                        translation: Vec3::new(self.dude.coords.x, self.dude.coords.y, 0.),
                        rotation: self.dude.rotation,
                        ..default()
                    },
                    ..default()
                },
                Velocity(movement_direction.truncate().normalize() * 500.),
                Lifetime(3.),
            ))
            .insert(Arrow {});
    }

    pub fn check_for_collisions(
        mut commands: Commands,
        arrow_query: Query<(Entity, &Transform), With<Arrow>>,
        badguy_query: Query<(Entity, &Transform), With<BadGuy>>,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
    ) {
        for (arrow_entity, arrow_transform) in &arrow_query {
            for (badguy_entity, badguy_transform) in &badguy_query {
                let collision = collide(
                    arrow_transform.translation,
                    SIZE_ARROW,
                    badguy_transform.translation,
                    SIZE_BADGUY,
                );

                if collision.is_some() {
                    commands.entity(arrow_entity).despawn();
                    commands.entity(badguy_entity).despawn();
                    audio
                        .play(asset_server.load("audios/enemy.wav"))
                        .with_volume(0.5);
                }
            }
        }
    }

    pub fn mouse_button_input(
        buttons: Res<Input<MouseButton>>,
        query: Query<&mut Dude>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
    ) {
        if buttons.just_released(MouseButton::Left) {
            let dude = query.single();
            let mut arrow_service = ArrowService::new(dude, &asset_server, &mut commands);
            arrow_service.spawn();
            audio
                .play(asset_server.load("audios/shoot.wav"))
                .with_volume(0.5);
        }
    }
}
