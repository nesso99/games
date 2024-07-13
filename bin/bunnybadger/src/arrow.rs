use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};
use bevy_kira_audio::prelude::*;

use crate::{
    badguy::BadGuy,
    common::{Lifetime, Velocity, SIZE_ARROW, SIZE_BADGUY},
    dude::Dude,
};

#[derive(Component)]
pub struct Arrow;

pub struct ArrowService {}

impl ArrowService {
    const ARROW_PATH: &'static str = "images/bullet.png";

    pub fn spawn(commands: &mut Commands, asset_server: &Res<AssetServer>, dude: &Dude) {
        let texture: Handle<Image> = asset_server.load(Self::ARROW_PATH);

        let movement_direction = dude.rotation * Vec3::X;
        commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(dude.coords.x, dude.coords.y, 0.),
                        rotation: dude.rotation,
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
                let intersects =
                    Aabb2d::new(arrow_transform.translation.truncate(), SIZE_ARROW / 2.0)
                        .intersects(&Aabb2d::new(
                            badguy_transform.translation.truncate(),
                            SIZE_BADGUY / 2.0,
                        ));

                if intersects {
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
        buttons: Res<ButtonInput<MouseButton>>,
        query: Query<&mut Dude>,
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        audio: Res<Audio>,
    ) {
        if buttons.just_released(MouseButton::Left) {
            let dude = query.single();
            Self::spawn(&mut commands, &asset_server, dude);
            audio
                .play(asset_server.load("audios/shoot.wav"))
                .with_volume(0.5);
        }
    }
}
