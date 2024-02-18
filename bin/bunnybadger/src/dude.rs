use bevy::prelude::*;

use crate::common::MainCamera;

const DUDE_SPEED: f32 = 500.0;

#[derive(Component)]
pub struct Dude {
    pub coords: Vec2,
    pub rotation: Quat,
}

pub struct DudeService<'a, 'w, 's> {
    texture: Handle<Image>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'w, 's> DudeService<'a, 'w, 's> {
    pub fn new(asset_server: &Res<AssetServer>, commands: &'a mut Commands<'w, 's>) -> Self {
        Self {
            texture: asset_server.load("images/dude.png"),
            commands,
        }
    }

    pub fn spawn(&mut self) {
        self.commands.spawn((
            SpriteBundle {
                texture: self.texture.clone(),
                ..default()
            },
            Dude {
                coords: Vec2::default(),
                rotation: Quat::default(),
            },
        ));
    }

    pub fn update(
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&mut Dude, &mut Transform)>,
        q_windows: Query<&Window, With<Window>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        time: Res<Time>,
    ) {
        let (mut dude, mut dude_transform) = query.single_mut();
        let mut x_direction = 0.0;
        let mut y_direction = 0.0;
        if keyboard_input.pressed(KeyCode::A) {
            x_direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            x_direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            y_direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            y_direction -= 1.0;
        }
        let new_x_position =
            dude_transform.translation.x + x_direction * DUDE_SPEED * time.delta_seconds();
        let new_y_position =
            dude_transform.translation.y + y_direction * DUDE_SPEED * time.delta_seconds();
        dude_transform.translation.x = new_x_position.clamp(-320., 320.);
        dude_transform.translation.y = new_y_position.clamp(-240., 240.);
        dude.coords = dude_transform.translation.truncate();

        let window = q_windows.single();
        let (camera, camera_transform) = q_camera.single();
        if let Some(target) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let pos = dude_transform.translation.truncate();
            let direction = target - pos;
            let angle = direction.y.atan2(direction.x);
            dude_transform.rotation = Quat::from_rotation_z(angle);
            dude.rotation = dude_transform.rotation;
        }
    }
}
