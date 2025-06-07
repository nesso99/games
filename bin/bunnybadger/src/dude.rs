use bevy::prelude::*;

use crate::{common::MainCamera, resources::GameAssets};

const DUDE_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Dude {
    pub coords: Vec2,
    pub rotation: Quat,
    pub health: u8,
}

pub struct DudeService;

impl DudeService {
    pub fn spawn(commands: &mut Commands, game_asset: &GameAssets) {
        commands.spawn((
            Sprite::from_image(game_asset.dude_texture.clone()),
            Dude {
                coords: Vec2::default(),
                rotation: Quat::default(),
                health: 198,
            },
        ));
    }

    pub fn update(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut query: Query<(&mut Dude, &mut Transform)>,
        q_windows: Query<&Window, With<Window>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        time: Res<Time>,
    ) {
        let (mut dude, mut dude_transform) = query.single_mut().unwrap();

        let mut x_direction = 0.0;
        let mut y_direction = 0.0;
        if keyboard_input.pressed(KeyCode::KeyA) {
            x_direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            x_direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            y_direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            y_direction -= 1.0;
        }
        let new_x_position =
            dude_transform.translation.x + x_direction * DUDE_SPEED * time.delta_secs();
        let new_y_position =
            dude_transform.translation.y + y_direction * DUDE_SPEED * time.delta_secs();
        dude_transform.translation.x = new_x_position.clamp(-320., 320.);
        dude_transform.translation.y = new_y_position.clamp(-240., 240.);
        dude.coords = dude_transform.translation.truncate();

        if q_windows.is_empty() {
            print!("q_windows is empty");
            return;
        }
        let window = q_windows.single().unwrap();
        let (camera, camera_transform) = q_camera.single().unwrap();

        if let Some(target) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
            .map(|ray| ray.origin.truncate())
        {
            let pos = dude_transform.translation.truncate();
            let direction = target - pos;
            let angle = direction.y.atan2(direction.x);

            // update dude
            dude_transform.rotation = Quat::from_rotation_z(angle);
            // for arrow rotation
            dude.rotation = dude_transform.rotation;
        }
    }
}
